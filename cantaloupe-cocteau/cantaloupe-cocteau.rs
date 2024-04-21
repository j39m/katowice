use pulseaudio::protocol;
use std::{ffi::CString, os::unix::net::UnixStream};

const CACO: &str = "cantaloupe-cocteau";

type CacoResult<T> = Result<T, Box<dyn std::error::Error>>;

struct PulseContext {
    sock: std::io::BufReader<UnixStream>,
    seq: u32,
    version: u16,
}

struct SinkInputs {
    map: std::collections::HashMap<u32, protocol::SinkInputInfo>,
    quodlibet_index: Option<u32>,
}

impl PulseContext {
    fn write_command_message(
        &mut self,
        command: protocol::Command,
    ) -> Result<(), protocol::ProtocolError> {
        let seq = self.get_seq();
        protocol::write_command_message(self.sock.get_mut(), seq, command, self.version)
    }

    fn get_seq(&mut self) -> u32 {
        self.seq += 1;
        self.seq - 1
    }
}

fn make_context() -> CacoResult<PulseContext> {
    let socket_path = pulseaudio::socket_path_from_env().ok_or("socket_path_from_env()")?;
    let mut sock = std::io::BufReader::new(UnixStream::connect(socket_path)?);
    let cookie = pulseaudio::cookie_path_from_env()
        .and_then(|path| std::fs::read(path).ok())
        .unwrap_or_default();
    let auth = protocol::AuthParams {
        version: protocol::MAX_VERSION,
        supports_shm: false,
        supports_memfd: false,
        cookie,
    };

    protocol::write_command_message(
        sock.get_mut(),
        0,
        protocol::Command::Auth(auth),
        protocol::MAX_VERSION,
    )?;
    let (_, auth_info) =
        protocol::read_reply_message::<protocol::AuthReply>(&mut sock, protocol::MAX_VERSION)?;
    let version = std::cmp::min(protocol::MAX_VERSION, auth_info.version);

    Ok(PulseContext {
        sock: sock,
        seq: 1,
        version: version,
    })
}

fn set_client_name(context: &mut PulseContext) -> CacoResult<()> {
    let mut props = protocol::Props::new();
    props.set(protocol::Prop::ApplicationName, CString::new(CACO).unwrap());
    context.write_command_message(protocol::Command::SetClientName(props))?;
    let _ = protocol::read_reply_message::<protocol::SetClientNameReply>(
        &mut context.sock,
        context.version,
    )?;
    Ok(())
}

fn get_sink_indices(context: &mut PulseContext) -> CacoResult<Vec<u32>> {
    context.write_command_message(protocol::Command::GetSinkInfoList)?;
    let (_, mut sink_infos) =
        protocol::read_reply_message::<protocol::SinkInfoList>(&mut context.sock, context.version)?;
    sink_infos.sort_by_key(|info| info.index);

    println!("{}:", "sink infos");
    for info in &sink_infos {
        println!("     {:>3}: {}", info.index, info.name.to_str().unwrap());
    }
    println!("{}", "");
    Ok(sink_infos
        .into_iter()
        .filter_map(|info| Some(info.index))
        .collect())
}

fn get_sink_inputs(context: &mut PulseContext) -> CacoResult<SinkInputs> {
    context.write_command_message(protocol::Command::GetSinkInputInfoList)?;
    let (_, mut sink_input_infos) = protocol::read_reply_message::<protocol::SinkInputInfoList>(
        &mut context.sock,
        context.version,
    )?;
    sink_input_infos.sort_by_key(|info| info.index);

    let mut quodlibet_index: u32 = 0;
    println!("{}:", "sink input infos");
    for info in &sink_input_infos {
        let application_name = match info.props.get(protocol::Prop::ApplicationName) {
            Some(u8s) => String::from_utf8(u8s.to_vec()).unwrap_or(String::from("[bad UTF-8]")),
            None => String::from("[no name]"),
        };
        if application_name.starts_with("Quod Libet") {
            quodlibet_index = info.index;
        }

        println!(
            "  {:>6} (sink {:>3}): {application_name}\n          {}",
            info.index,
            info.sink_index,
            info.name.to_str().unwrap()
        );
    }
    Ok(SinkInputs {
        map: sink_input_infos
            .into_iter()
            .map(|info| (info.index, info))
            .collect(),
        quodlibet_index: match quodlibet_index {
            0 => None,
            _ => Some(quodlibet_index),
        },
    })
}

fn move_sink_input(
    context: &mut PulseContext,
    index: u32,
    sink_indices: Vec<u32>,
    sink_inputs: SinkInputs,
) -> CacoResult<()> {
    if sink_indices.len() < 2 {
        panic!("PEBCAK: why did you run this?");
    }
    let sink_input = sink_inputs.map.get(&index).expect("typo?");
    let current_sink = sink_input.sink_index;
    let mut target_sink: u32 = *sink_indices.get(0).unwrap();
    for sink in sink_indices {
        if sink > current_sink {
            target_sink = sink;
            break;
        }
    }
    println!("{index}: [sink {current_sink}] --> [sink {target_sink}]");
    context.write_command_message(protocol::Command::MoveSinkInput(
        protocol::command::MoveStreamParams {
            index: Some(index),
            device_index: Some(target_sink),
            device_name: None,
        },
    ))?;
    protocol::read_ack_message(&mut context.sock)?;
    Ok(())
}

pub fn main() -> CacoResult<()> {
    let mut context = make_context()?;
    set_client_name(&mut context)?;
    let sink_indices = get_sink_indices(&mut context)?;
    let sink_inputs = get_sink_inputs(&mut context)?;

    let selection = dialoguer::Input::<String>::new()
        .with_prompt(format!("sink input to move"))
        .allow_empty(true)
        .interact_text()
        .unwrap();
    if let Ok(index) = selection.parse::<u32>() {
        move_sink_input(&mut context, index, sink_indices, sink_inputs);
    } else if let Some(index) = sink_inputs.quodlibet_index {
        move_sink_input(&mut context, index, sink_indices, sink_inputs);
    } else {
        println!("{}", "bye");
    }
    Ok(())
}
