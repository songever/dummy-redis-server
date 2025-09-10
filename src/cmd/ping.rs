use bytes::Bytes;

use crate::{parse::{Parse, ParseError}, Connection, Frame, Result};

#[derive(Debug, Default)]
pub struct Ping {
    msg: Option<Bytes>,
}

impl Ping {
    pub fn new(msg: Option<Bytes>) -> Ping{
        Ping { msg }
    }

    pub(crate) fn parse_frames(parse: &mut Parse) -> Result<Ping>{
        match parse.next_bytes() {
            Ok(msg) => Ok(Ping::new(Some(msg))),
            Err(ParseError::EndOfStream) => Ok(Ping::default()),
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) async fn apply(self, dst: &mut Connection) -> crate::Result<()> {
        let response: Frame = match self.msg {
            None => Frame::Simple("PONG".to_string()),
            Some(msg) => Frame::Bulk(msg), 
        };

        dst.write_frame(&response).await?;

        Ok(())
    }

    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("ping".as_bytes()));
        if let Some(msg) = self.msg {
            frame.push_bulk(msg);
        }
        frame
    }
}