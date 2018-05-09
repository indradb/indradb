use actix::prelude::*;
use super::util::*;
use graphql;
use script;
use serde_json::value::Value as JsonValue;
use actix_web::{error, AsyncResponder, HttpRequest, HttpResponse, HttpMessage, Error, Json, State, FutureResponse};
use futures::{Future, Stream};

pub struct AppState {
    pub executor: Addr<Syn, graphql::Executor>,
}

pub fn graphql_handler(state: State<AppState>, data: Json<graphql::Request>) -> FutureResponse<HttpResponse> {
    state.executor.send(data.0).from_err().and_then(|res| {
        match res {
            Ok(user) => Ok(HttpResponse::Ok().content_type("application/json").body(user)),
            Err(err) => Ok(error::ErrorInternalServerError(err).into())
        }
    }).responder()
}

// TODO: offload heavy tasks to futures-cpupool
pub fn script_handler(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    let name = req.match_info().get("name").unwrap().to_string();

    req.json().from_err().and_then(move |payload: JsonValue| {
        let (path, contents) = get_script_file(name.to_string())?;

        match script::execute(&contents, &path, payload) {
            Ok(value) => {
                Ok(HttpResponse::Ok().json(value))
            },
            Err(err) => {
                let error_message = format!("Script failed: {:?}", err);
                Err(error::ErrorInternalServerError(error_message))
            }
        }
    }).responder()
}

pub fn mapreduce_handler(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    unimplemented!();
    // res.json().from_err().and_then(|payload: JsonValue| {
    //     let name = req.match_info().get("name")?;
    //     let (path, contents) = get_script_file(name)?;
    //     ws::start(req, MapReduceActor::new(path, contents))
    // }).responder();

    // ///////////////////////

    // // Get the inputs
    // let name: String = get_url_param(req, "name")?;
    // let payload = read_json(&mut req.body)?.unwrap_or_else(|| JsonValue::Null);
    // let (path, contents) = get_script_file(name)?;

    // // Construct a response
    // let mut hs = Headers::new();
    // hs.set(ContentType(get_json_mime()));
    // hs.set(TransferEncoding(vec![Encoding::Chunked]));

    // let (sender, receiver) = script::bounded(1);

    // spawn(move || {
    //     script::execute_mapreduce(contents, path, payload, sender);
    // });

    // Ok(Response {
    //     status: Some(status::Ok),
    //     headers: hs,
    //     extensions: TypeMap::new(),
    //     body: Some(Box::new(receiver)),
    // })
}

pub fn not_found_handler(_: HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::NotFound().json(json!({
        "error": "No route found"
    }))
}

// struct MapReduceActor {
//     path: String,
//     contents: String,

// }

// impl MapReduceActor {
//     fn new(path: String, contents: String) -> Self {
//         MapReduceActor {
//             path: path,
//             contents: contents,
//         }
//     }
// }

// impl Actor for MapReduceActor {
//     type Context = ws::WebsocketContext<Self>;
// }

// impl StreamHandler<ws::Message, ws::ProtocolError> for MapReduceActor {
//     fn handle(&mut self, message: ws::Message, context: &mut self::Context) {
//         match message {
//             ws::Message::Ping(contents) => context.pong(&contents),
//             ws::Message::Text(contents) => {
//                 if contents == "update".to_string() {
//                     context.text();
//                 }
//             }
//             ws::Message::Close(_) => {
//                 context.stop();
//             },
//             _ => ()
//         }
//     }
// }
