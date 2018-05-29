use actix::prelude::*;
use actix_web::{Error as ActixError, ws};
use indradb::{Vertex, Error};
use http;
use script::{Request, converters};
use serde_json;
use serde_json::value::Value as JsonValue;
use statics;
use std::thread::{spawn, JoinHandle};
use std::time::Duration;
use futures::future;
use super::router::{ProcessNextBatch, GetStatus, Router};

pub struct Executor {
    router: Addr<Syn, Router>
}

impl Executor {
    pub fn new(req: Request) -> Self {
        let router = SyncArbiter::start(1, move || Router::new(req));
        Self { router }
        // TODO: spawn thread
    }
}

impl Actor for Executor {
    type Context = ws::WebsocketContext<Self, http::AppState>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for Executor {
    fn handle(&mut self, message: ws::Message, context: &mut Self::Context) {
        match message {
            ws::Message::Ping(message) => context.pong(&message),
            ws::Message::Close(_) => context.stop(),
            ws::Message::Text(text) => {
                if text.trim() == "update" {
                    self.router.send(GetStatus)
                        .into_actor(self)
                        .then(|status, _, context| {
                            let contents = serde_json::to_string(&status.unwrap()).expect("Expected to be able to serialize status to JSON");
                            context.text(contents);
                            future::ok(())
                        })
                        .wait(context);
                }
            },
            _ => {}
        }
    }
}
