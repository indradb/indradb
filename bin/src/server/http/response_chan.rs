use crossbeam_channel::{Sender, Receiver, bounded as crossbeam_bounded};
use serde_json;
use serde_json::value::Value as JsonValue;
use iron::response::WriteBody;
use std::io::{Write, Result as IoResult};

pub struct ResponseSender(pub Sender<JsonValue>);

impl ResponseSender {
    fn new(sender: Sender<JsonValue>) -> Self {
        Self { 0: sender }
    }
}

pub struct ResponseReceiver(pub Receiver<JsonValue>);

impl ResponseReceiver {
    fn new(receiver: Receiver<JsonValue>) -> Self {
        Self { 0: receiver }
    }
}

impl WriteBody for ResponseReceiver {
    fn write_body(&mut self, res: &mut Write) -> IoResult<()> {
        loop {
            let chunk = match self.0.recv() {
                Ok(chunk) => chunk,
                Err(_) => return Ok(())
            };

            let mut s = serde_json::to_string(&chunk).unwrap();
            s.write_body(res)?;
            "\r\n".write_body(res)?;
            res.flush()?;
        }
    }
}

pub fn bounded(cap: usize) -> (ResponseSender, ResponseReceiver) {
    let (sender, receiver) = crossbeam_bounded::<JsonValue>(cap);
    (ResponseSender::new(sender), ResponseReceiver::new(receiver))
}
