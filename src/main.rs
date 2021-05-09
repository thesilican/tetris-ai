mod config;

use crate::config::create_ai;
use actix::{Actor, StreamHandler};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use common::api::ai::TetrisAI;

struct WSActor<T: TetrisAI> {
    ai: T,
}
impl<T: TetrisAI> WSActor<T> {
    fn new(ai: T) -> Self {
        WSActor { ai }
    }
}
impl<T: TetrisAI + Unpin + 'static> Actor for WSActor<T> {
    type Context = ws::WebsocketContext<Self>;
}
impl<T: TetrisAI + Unpin + 'static> StreamHandler<Result<ws::Message, ws::ProtocolError>>
    for WSActor<T>
{
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(req)) => {
                let res = self.ai.api_evaluate_json(req);
                ctx.text(res);
            }
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WSActor::new(create_ai()), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on port 8080");
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(fs::Files::new("/static", "static"))
            .route("/eval", web::get().to(index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
