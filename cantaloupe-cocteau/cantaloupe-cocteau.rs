use pulseaudio::protocol;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::{ffi::CString, io::IsTerminal, os::unix::net::UnixStream};

const CACO: &str = "cantaloupe-cocteau";

type CacoResult<T> = Result<T, Box<dyn std::error::Error>>;

struct PulseContext {
    sock: std::io::BufReader<UnixStream>,
    seq: u32,
    version: u16,
    sinks: BTreeMap<u32, String>,
}

#[derive(PartialOrd, PartialEq, Eq)]
struct SinkInput {
    sink_index: u32,
    index: u32,
    application_name: String,
    title: String,
}

// The larger grouping is the sink index.
impl Ord for SinkInput {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sink_index
            .cmp(&other.sink_index)
            .then(self.index.cmp(&other.index))
    }
}

impl From<pulseaudio::protocol::command::SinkInputInfo> for SinkInput {
    fn from(sink_input_info: pulseaudio::protocol::command::SinkInputInfo) -> Self {
        Self {
            sink_index: sink_input_info.sink_index,
            index: sink_input_info.index,
            application_name: match sink_input_info.props.get(protocol::Prop::ApplicationName) {
                Some(u8s) => String::from_utf8(u8s.to_vec()).unwrap_or(String::from("[bad UTF-8]")),
                None => String::from("[no name]"),
            },
            title: String::from(sink_input_info.name.to_str().unwrap_or("[no name]")),
        }
    }
}

impl std::fmt::Display for SinkInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  {:>6}・{}:\n{:13}{}",
            self.index, self.application_name, "", self.title
        )
    }
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
            sinks: BTreeMap::new(),         // bogus for now
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

    fn get_sinks(&mut self) -> CacoResult<BTreeMap<u32, String>> {
        self.write_command_message(protocol::Command::GetSinkInfoList)?;
        let (_, sink_infos) =
            protocol::read_reply_message::<protocol::SinkInfoList>(&mut self.sock, self.version)?;
        Ok(sink_infos
            .into_iter()
            .map(|info| (info.index, info.name.to_str().unwrap().to_string()))
            .collect())
    }

    fn get_sink_inputs(&mut self) -> CacoResult<(BTreeSet<SinkInput>, Option<u32>)> {
        self.write_command_message(protocol::Command::GetSinkInputInfoList)?;
        let sink_inputs = protocol::read_reply_message::<protocol::SinkInputInfoList>(
            &mut self.sock,
            self.version,
        )?
        .1
        .into_iter()
        .map(|p| SinkInput::from(p))
        .collect::<BTreeSet<SinkInput>>();

        let mut quodlibet_index: Option<u32> = None;
        let mut current_sink_index_for_printing: u32 = 0;
        for info in &sink_inputs {
            if info.application_name.starts_with("Quod Libet") {
                quodlibet_index = Some(info.index);
            }
            if info.sink_index != current_sink_index_for_printing {
                println!(
                    "sink {:<8} ({})",
                    info.sink_index,
                    self.sinks.get(&info.sink_index).unwrap()
                );
                current_sink_index_for_printing = info.sink_index;
            }
            println!("{info}");
        }
        Ok((sink_inputs, quodlibet_index))
    }

    fn move_sink_input(&mut self, index: u32, sink_inputs: BTreeSet<SinkInput>) -> CacoResult<()> {
        if self.sinks.len() < 2 {
            panic!("PEBCAK: why did you run this?");
        }
        let one_sink_input = sink_inputs
            .into_iter()
            .filter(|si| si.index == index)
            .collect::<Vec<_>>();
        assert!(one_sink_input.len() == 1usize);
        let sink_input = &one_sink_input[0];
        let current_sink = sink_input.sink_index;
        // Bootleg modulo: if `current_sink` is the max, we have to
        // snap it around to the smallest sink. Otherwise, we can simply
        // "advance" to the next available sink.
        let target_sink = if current_sink == *self.sinks.last_key_value().unwrap().0 {
            self.sinks.first_key_value().unwrap().0
        } else {
            self.sinks
                .iter()
                .filter(|e| *e.0 > current_sink)
                .collect::<Vec<_>>()[0]
                .0
        };
        println!("{index}: [sink {current_sink}] --> [sink {target_sink}]");
        self.write_command_message(protocol::Command::MoveSinkInput(
            protocol::command::MoveStreamParams {
                index: Some(index),
                device_index: Some(*target_sink),
                device_name: None,
            },
        ))?;
        protocol::read_ack_message(&mut self.sock)?;
        Ok(())
    }
}

pub fn main() -> CacoResult<()> {
    let mut context = PulseContext::new()?;
    let (sink_inputs, quodlibet_index) = context.get_sink_inputs()?;

    if !std::io::stdout().is_terminal() {
        if let Some(index) = quodlibet_index {
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
    } else if let Some(index) = quodlibet_index {
        context.move_sink_input(index, sink_inputs)?;
    } else {
        println!("{}", "bye");
    }
    Ok(())
}
