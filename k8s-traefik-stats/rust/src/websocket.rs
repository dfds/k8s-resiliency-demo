use std::time::{Duration, Instant};
use actix::prelude::*;
use actix_web_actors::ws;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use log::info;
use crate::model::AppState;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn ws_index(r: HttpRequest, stream: web::Payload, state : web::Data<AppState>) -> Result<HttpResponse, Error> {
    let res = ws::start(IndexWs::new(state.clone()), &r, stream);
    res
}

pub struct IndexWs {
    hb: Instant,
    state: web::Data<AppState>
}

impl Actor for IndexWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        self.state.ws.register_client(ctx.address());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for IndexWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            },
            Ok(ws::Message::Pong(msg)) => {
                self.hb = Instant::now();
            },
            Ok(ws::Message::Text(msg)) => {
                info!("WS MSG: {}", msg);
            },
            _ => {}
        }
    }
}

pub struct TextMessage {
    msg : String
}

impl TextMessage {
    pub fn new(msg : String) -> Self {
        Self {
            msg
        }
    }
}

impl Handler<TextMessage> for IndexWs {
    type Result = usize;

    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.msg.clone());
        0
    }
}

impl actix::Message for TextMessage {
    type Result = usize;
}


impl IndexWs {
    fn new(state : web::Data<AppState>) -> Self {
        Self {
            hb: Instant::now(),
            state: state.clone()
        }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}