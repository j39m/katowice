use pulseaudio::protocol;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::{ffi::CString, io::IsTerminal, os::unix::net::UnixStream};

const CACO: &str = "cantaloupe-cocteau";

type CacoResult<T> = Result<T, Box<dyn std::error::Error>>;

struct PulseContext {
    sock: std::io::BufReader<UnixStream>,
    seq: u32,
    version: u16,
    sinks: BTreeSet<Sink>,
}

#[derive(PartialOrd, PartialEq, Eq)]
struct Sink {
    index: u32,
    name: String,
}

impl Ord for Sink {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

struct SinkInputs {
    map: std::collections::HashMap<u32, protocol::SinkInputInfo>,
    quodlibet_index: Option<u32>,
}

impl PulseContext {
    fn new() -> CacoResult<PulseContext> {
        let socket_path = pulseaudio::socket_path_from_env().ok_or("socket_path_from_env()")?;
        let sock = std::io::BufReader::new(UnixStream::connect(socket_path)?);
        let cookie = pulseaudio::cookie_path_from_env()
            .and_then(|path| std::fs::read(path).ok())
            .unwrap_or_default();

        let mut c = PulseContext {
            sock: sock,
            seq: 0,
            version: protocol::MAX_VERSION, // bogus for now
            sinks: BTreeSet::new(),         // bogus for now
        };

        c.write_command_message(protocol::Command::Auth(protocol::AuthParams {
            version: protocol::MAX_VERSION,
            supports_shm: false,
            supports_memfd: false,
            cookie,
        }))?;
        let (_, auth_info) = protocol::read_reply_message::<protocol::AuthReply>(
            &mut c.sock,
            protocol::MAX_VERSION,
        )?;
        c.version = std::cmp::min(protocol::MAX_VERSION, auth_info.version);

        c.set_client_name()?;
        c.sinks = c.get_sinks()?;
        Ok(c)
    }

    fn write_command_message(
        &mut self,
        command: protocol::Command,
    ) -> Result<(), protocol::ProtocolError> {
        let seq = self.get_seq();
        protocol::write_command_message(self.sock.get_mut(), seq, &command, self.version)
    }

    fn get_seq(&mut self) -> u32 {
        self.seq += 1;
        self.seq - 1
    }

    fn set_client_name(&mut self) -> CacoResult<()> {
        let mut props = protocol::Props::new();
        props.set(protocol::Prop::ApplicationName, CString::new(CACO).unwrap());
        self.write_command_message(protocol::Command::SetClientName(props))?;
        let _ = protocol::read_reply_message::<protocol::SetClientNameReply>(
            &mut self.sock,
            self.version,
        )?;
        Ok(())
    }

    fn get_sinks(&mut self) -> CacoResult<BTreeSet<Sink>> {
        self.write_command_message(protocol::Command::GetSinkInfoList)?;
        let (_, sink_infos) =
            protocol::read_reply_message::<protocol::SinkInfoList>(&mut self.sock, self.version)?;
        Ok(sink_infos
            .into_iter()
            .map(|info| Sink {
                index: info.index,
                name: info.name.to_str().unwrap().to_string(),
            })
            .collect())
    }

    fn get_sink_inputs(&mut self) -> CacoResult<SinkInputs> {
        self.write_command_message(protocol::Command::GetSinkInputInfoList)?;
        let (_, mut sink_input_infos) = protocol::read_reply_message::<protocol::SinkInputInfoList>(
            &mut self.sock,
            self.version,
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
                "  {:>6} (sink {:>4} {}): {application_name}\n          {}",
                info.index,
                info.sink_index,
                get_sink_circle(info.sink_index),
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

    fn move_sink_input(&mut self, index: u32, sink_inputs: SinkInputs) -> CacoResult<()> {
        if self.sinks.len() < 2 {
            panic!("PEBCAK: why did you run this?");
        }
        let sink_input = sink_inputs.map.get(&index).expect("typo?");
        let current_sink = sink_input.sink_index;
        // Bootleg modulo: if `current_sink` is the max, we have to
        // snap it around to the smallest sink. Otherwise, we can simply
        // "advance" to the next available sink.
        let target_sink = if current_sink == self.sinks.last().unwrap().index {
            self.sinks.first().unwrap().index
        } else {
            self.sinks
                .iter()
                .filter(|s| s.index > current_sink)
                .collect::<Vec<_>>()[0]
                .index
        };
        println!("{index}: [sink {current_sink}] --> [sink {target_sink}]");
        self.write_command_message(protocol::Command::MoveSinkInput(
            protocol::command::MoveStreamParams {
                index: Some(index),
                device_index: Some(target_sink),
                device_name: None,
            },
        ))?;
        protocol::read_ack_message(&mut self.sock)?;
        Ok(())
    }
}

fn get_sink_circle(index: u32) -> String {
    let capped_index = index % 5;
    match capped_index {
        0 => "💢".to_owned(),
        1 => "💦".to_owned(),
        2 => "💚".to_owned(),
        3 => "🤖".to_owned(),
        4 => "🤔".to_owned(),
        _ => panic!("???"),
    }
}

pub fn main() -> CacoResult<()> {
    let mut context = PulseContext::new()?;
    let sink_inputs = context.get_sink_inputs()?;

    if !std::io::stdout().is_terminal() {
        if let Some(index) = sink_inputs.quodlibet_index {
            context.move_sink_input(index, sink_inputs)?;
            return Ok(());
        }
        return Err("hmm, something's wrong".into());
    }

    let selection = dialoguer::Input::<String>::new()
        .with_prompt(format!("sink input to move"))
        .allow_empty(true)
        .interact_text()
        .unwrap();
    if let Ok(index) = selection.parse::<u32>() {
        context.move_sink_input(index, sink_inputs)?;
    } else if let Some(index) = sink_inputs.quodlibet_index {
        context.move_sink_input(index, sink_inputs)?;
    } else {
        println!("{}", "bye");
    }
    Ok(())
}
