use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use ai_api::TetrisAI;
use ai_api::{parse, stringify};
use c4w_ai::ai::ai::AI as C4WAI;
use rusty_ai::ai::ai::{AIWeights, AI as RustyAI};

struct WSActor {
    // ai: RustyAI,
    ai: C4WAI,
}
impl WSActor {
    fn new() -> Self {
        // Rusty AI
        //  Downstacker:
        //      "PuDw3r2oNtK9TeZhPpwa3Lvq4G++x7VAvs9SFb8YPAI+P3qy"
        //  Points:
        //      "Pl3vz78Jv2G+FHNmvU3rWD6tNxu9ws5aO6jcXr8v4am+V8l9"

        // let rusty_weights = "Pl3vz78Jv2G+FHNmvU3rWD6tNxu9ws5aO6jcXr8v4am+V8l9";
        // let ai = RustyAI::new(&AIWeights::from_string(rusty_weights).unwrap(), false);
        // WSActor { ai }
        let ai = C4WAI::new(true);
        WSActor { ai }
    }
    fn handle_req(&mut self, req: String) -> Result<String, ()> {
        let req = parse(req).map_err(|_| ())?;
        let res = self.ai.evaluate(req).map_err(|_| ())?;
        let res = stringify(res).map_err(|_| ())?;
        Ok(res)
    }
}
impl Actor for WSActor {
    type Context = ws::WebsocketContext<Self>;
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WSActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(req)) => {
                let res = self.handle_req(req);
                if let Ok(res) = res {
                    ctx.text(res);
                } else {
                    ctx.text("null");
                }
            }
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WSActor::new(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on port 8080");
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
