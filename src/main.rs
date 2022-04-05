use actix::{Actor, StreamHandler};
use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use bytestring::ByteString;

mod data;

/// Define WsConnection to client, it will get a reference to the shared data
struct ClientConnection {
    id: u32,
}

impl Actor for ClientConnection {
    type Context = ws::WebsocketContext<Self>;
}

impl ClientConnection {
    fn handle_text(&mut self, msg: &ByteString, ctx: &mut ws::WebsocketContext<Self>) {
        println!("id: {}", self.id);
        self.id += 1;
        println!("{:?}", msg);
        if msg.to_string() == "cmd!" {
            ctx.text(format!("{}", self.id));
        }
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ClientConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(msg)) => {
                self.handle_text(&msg, ctx);
                ctx.text(msg);
            }
            Ok(ws::Message::Ping(text)) => ctx.pong(&text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(ClientConnection { id: 0 }, &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .route("/ws/", web::get().to(index))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
