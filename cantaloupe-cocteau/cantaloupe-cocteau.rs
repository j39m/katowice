use pulseaudio::protocol;
use std::{ffi::CString, os::unix::net::UnixStream};

const CACO: &str = "cantaloupe-cocteau";

type CacoResult<T> = Result<T, Box<dyn std::error::Error>>;

struct PulseContext {
    sock: std::io::BufReader<UnixStream>,
    seq: u32,
    version: u16,
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
    for info in &sink_infos {
        println!("{}: {}", info.index, info.name.to_str().unwrap());
    }
    Ok(sink_infos
        .into_iter()
        .filter_map(|info| Some(info.index))
        .collect())
}

pub fn main() -> CacoResult<()> {
    let mut context = make_context()?;
    set_client_name(&mut context)?;
    get_sink_indices(&mut context)?;
    Ok(())
}
