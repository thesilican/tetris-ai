mod ai;

use actix::prelude::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use ai::*;

struct MyWsHandler {
    ai: WsAi,
}
impl MyWsHandler {
    fn new() -> Self {
        Self { ai: get_ai() }
    }
}
impl Actor for MyWsHandler {
    type Context = ws::WebsocketContext<Self>;
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWsHandler {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_) | ws::Message::Binary(_)) => {}
            Ok(ws::Message::Text(req)) => {
                // println!("Request: {}", req);
                match self.ai.evaluate(&req) {
                    Ok(res) => {
                        // println!("Response: {}", res);
                        ctx.text(res);
                    }
                    Err(err) => {
                        println!("Error processing request: {}", err);
                    }
                }
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let res = ws::start(MyWsHandler::new(), &r, stream);
    res
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on port 8080");
    HttpServer::new(|| App::new().service(web::resource("/").route(web::get().to(ws_index))))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
