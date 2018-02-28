use crossbeam_channel::{bounded as crossbeam_bounded, Receiver, Sender};
use serde_json;
use serde_json::value::Value as JsonValue;
use iron::response::WriteBody;
use std::io::{Result as IoResult, Write};

#[derive(Clone, Debug)]
pub enum Update {
    Ping(JsonValue),
    Ok(JsonValue),
    Err(JsonValue),
}

impl Update {
    fn contents(&self) -> JsonValue {
        match *self {
            Update::Ping(ref value) => json!({ "ping": value }),
            Update::Ok(ref value) => json!({ "ok": value }),
            Update::Err(ref value) => json!({ "error": value }),
        }
    }

    fn is_last(&self) -> bool {
        if let Update::Ping(_) = *self {
            false
        } else {
            true
        }
    }
}

#[derive(Clone, Debug)]
pub struct ResponseSender(pub Sender<Update>);

impl ResponseSender {
    fn new(sender: Sender<Update>) -> Self {
        Self { 0: sender }
    }
}

#[derive(Clone, Debug)]
pub struct ResponseReceiver(pub Receiver<Update>);

impl ResponseReceiver {
    fn new(receiver: Receiver<Update>) -> Self {
        Self { 0: receiver }
    }
}

impl WriteBody for ResponseReceiver {
    fn write_body(&mut self, res: &mut Write) -> IoResult<()> {
        loop {
            let update = match self.0.recv() {
                Ok(update) => update,
                Err(_) => return Ok(()),
            };

            let mut s = serde_json::to_string(&update.contents()).unwrap();
            s.write_body(res)?;
            "\r\n".write_body(res)?;
            res.flush()?;

            if update.is_last() {
                return Ok(());
            }
        }
    }
}

pub fn bounded(cap: usize) -> (ResponseSender, ResponseReceiver) {
    let (sender, receiver) = crossbeam_bounded::<Update>(cap);
    (ResponseSender::new(sender), ResponseReceiver::new(receiver))
}
