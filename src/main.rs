use std::sync::{Arc, Mutex};

use actix::{Actor, Addr, Handler, Message, StreamHandler};
use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use bytestring::ByteString;

mod data;
// use data::*;

/// Define WsConnection to client, it will get a reference to the shared data
struct ClientConnection {
    id: usize,
    server: Arc<Mutex<Server>>,
}

impl Actor for ClientConnection {
    type Context = ws::WebsocketContext<Self>;
}

// impl Handler<Request> for ClientConnection {}
impl ClientConnection {
    fn handle_text(&mut self, msg: &ByteString, _ctx: &mut ws::WebsocketContext<Self>) {
        let s = self.server.lock().unwrap();
        for con in s.connections.iter() {
            if self.id != con.id {
                con.addr.do_send(TextMessage {
                    msg: msg.to_string(),
                });
            }
        }
    }
}

impl Handler<TextMessage> for ClientConnection {
    type Result = ();

    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.msg);
    }
}

struct TextMessage {
    msg: String,
}

impl Message for TextMessage {
    type Result = ();
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ClientConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(msg)) => {
                self.handle_text(&msg, ctx);
            }
            Ok(ws::Message::Ping(text)) => ctx.pong(&text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(_)) => {
                let mut s = self.server.lock().unwrap();
                let idx: usize = s
                    .connections
                    .iter()
                    .position(|con| con.id == self.id)
                    .unwrap();
                s.connections.swap_remove(idx);
            }
            _ => (),
        }
    }
}

async fn index(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Mutex<Server>>,
) -> Result<HttpResponse, Error> {
    let mut s = server.lock().unwrap();
    let (addr, resp) = match ws::WsResponseBuilder::new(
        ClientConnection {
            id: s.id,
            server: server.clone().into_inner(),
        },
        &req,
        stream,
    )
    .start_with_addr()
    {
        Ok((a, r)) => (a, r),
        Err(err) => {
            println!("{:?}", err);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };
    let id = s.id;
    s.connections.push(Connection { addr, id });
    s.id += 1;
    drop(s);
    Ok(resp)
}

struct Server {
    connections: Vec<Connection>,
    id: usize,
}

#[derive(Debug)]
struct Connection {
    addr: Addr<ClientConnection>,
    id: usize,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let data = web::Data::new(Mutex::new(Server {
        connections: vec![],
        id: 0,
    }));
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/ws/", web::get().to(index))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
