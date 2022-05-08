use std::sync::{Arc, Mutex, MutexGuard};

use actix::{Actor, Addr, Handler, Message, StreamHandler};
use actix_web::middleware::Logger;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use bytes::Bytes;
use bytestring::ByteString;
use prost::Message as ProstMessage;

mod items;
// import protobuf generated structs
use items::*;

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

    fn handle_bin(&mut self, msg: Bytes, ctx: &mut ws::WebsocketContext<Self>) {
        let req = match match Request::decode(msg) {
            Ok(req) => req,
            Err(err) => {
                println!("{}", err);
                return;
            }
        }
        .request
        {
            Some(req) => req,
            None => {
                println!("Request type none");
                return;
            }
        };
        let mut s = self.server.lock().unwrap();
        match req {
            request::Request::Connect(c) => handle_connect(&mut s, c),
            request::Request::Move(m) => handle_move(s, m),
            request::Request::Shuffle(sh) => handle_shuffle(s, sh),
            request::Request::State(_) => handle_state(s, ctx),
        }
    }
}

fn handle_connect(s: &mut MutexGuard<Server>, req: Connect) {
    let players = &mut s.game_state.players;
    players.push(Player {
        id: req.game_id,
        name: req.player_name,
    });
    let stacks = &mut s.game_state.stacks;
    stacks.push(Stack {
        stack: Some(items::stack::Stack::Table(Table {
            id: req.game_id,
            card: vec![],
        })),
    });
    stacks.push(Stack {
        stack: Some(items::stack::Stack::Hand(Hand {
            id: req.game_id,
            card: vec![],
        })),
    });
}

fn handle_move(_s: MutexGuard<Server>, _reqq: Move) {}

fn handle_shuffle(_s: MutexGuard<Server>, _req: Shuffle) {}

fn handle_state(s: MutexGuard<Server>, ctx: &mut ws::WebsocketContext<ClientConnection>) {
    ctx.binary(s.game_state.encode_to_vec());
}

impl Handler<TextMessage> for ClientConnection {
    type Result = ();

    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.msg);
    }
}

impl Message for Response {
    type Result = ();
}

impl Handler<Response> for ClientConnection {
    type Result = ();

    fn handle(&mut self, msg: Response, ctx: &mut Self::Context) -> Self::Result {
        ctx.binary(msg.encode_to_vec());
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
            Ok(ws::Message::Binary(bin)) => {
                self.handle_bin(bin, ctx);
            }
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
    game_state: GameState,
    player_nr: u32,
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
        game_state: GameState {
            stacks: vec![],
            players: vec![],
        },
        player_nr: 0,
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
