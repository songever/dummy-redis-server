mod ping;
pub use ping::Ping;

mod get;
pub use get::Get;
mod set;
pub use set::Set;
mod unknown;
pub use unknown::Unknown;

use crate::{Db, frame::Frame, parse::Parse, shutdown::Shutdown, Connection};

#[derive(Debug)]
pub enum Command {
    Get(Get),
    // Publish(Publish),
    Set(Set),
    // Subscribe(Subscribe),
    // Unsubscribe(Unsubscribe),
    Ping(Ping),
    Unknown(Unknown),
}

impl Command {
    pub fn from_frame(frame: Frame) -> crate::Result<Command> {
        let mut parse = Parse::new(frame)?;

        let command_name = parse.next_string()?.to_lowercase();

        let command = match &command_name[..] {
                "get" => Command::Get(Get::parse_frames(&mut parse)?),
                // "publish" => Command::Publish(Publish::parse_frames(&mut parse)?),
                "set" => Command::Set(Set::parse_frames(&mut parse)?),
                // "subscribe" => Command::Subscribe(Subscribe::parse_frames(&mut parse)?),
                // "unsubscribe" => Command::Unsubscribe(Unsubscribe::parse_frames(&mut parse)?),
                "ping" => Command::Ping(Ping::parse_frames(&mut parse)?),
                _ => {
                    return Ok(Command::Unknown(Unknown::new(command_name)));
                }
            };

        parse.finish()?;

        Ok(command)
    }

//TODO: fix the code after about db
    pub(crate) async fn apply(
        self,
        db: &Db,
        dst: &mut Connection,
        shutdown: &mut Shutdown,
    ) -> crate::Result<()> {
        use Command::*;

        match self {
            Get(cmd) => cmd.apply(db, dst).await,
            // Publish(cmd) => cmd.apply(db, dst).await,
            Set(cmd) => cmd.apply(db, dst).await,
            // Subscribe(cmd) => cmd.apply(db, dst, shutdown).await,
            Ping(cmd) => cmd.apply(dst).await,
            Unknown(cmd) => cmd.apply(dst).await,
            
            // Unsubscribe(_) => Err("`Unsubscribe` is unsupported in this context".into()),
        }
    }
}