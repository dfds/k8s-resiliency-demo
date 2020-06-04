use tokio::stream::StreamExt;
use linemux::MuxedLines;
use std::fs::File;
use log::{info, error};
use byte_unit::Byte;
use crate::model::AppState;
use actix_web_actors::ws;
use crate::model::Bm;
use std::time::{Duration, Instant};

mod model;
mod websocket;
mod server;

fn main() {
    setup();

    let file_path = match std::env::var("KTS_FILEPATH") {
        Ok(val) => val,
        Err(_) => "access.log".to_owned()
    };

    let state = AppState::new();
    {
        let watcher_state = state.clone();
        std::thread::spawn(move || {
            loop {
                watcher(watcher_state.clone(), file_path.clone());
                error!("Watcher died, restarting.");
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            error!("Watcher thread is going down");
        });
    }

    server::serve(state);
}

#[tokio::main]
async fn watcher(state : AppState, file_path: String) {
    let mut lines = MuxedLines::new().unwrap();

    lines.add_file(&file_path).await.unwrap();
    let mut current_run_time = Instant::now();
    let one_second = Duration::from_secs(1);
    let mut current_http_count = model::BmStatusCodeCount::new();

    // New log line handling
    while let Some(Ok(line)) = lines.next().await {
        println!("{}", line.line());

        // Data
        let unserialized_msg : model::TraefikLog = match serde_json::from_str(&line.line()) {
            Ok(val) => val,
            Err(_) => return
        };
        let latency = model::BmLatency::from_log(unserialized_msg);
        let dto = model::BroadcastMessage { bm_type: model::BmLatency::type_name(), payload: latency.clone() };

        state.update_stats(latency.clone());
        let stats = model::Stats::from_bm_latency(latency.clone());
        current_http_count.count_200 = current_http_count.count_200 + stats.http_200_count;
        current_http_count.count_300 = current_http_count.count_300 + stats.http_300_count;
        current_http_count.count_400 = current_http_count.count_400 + stats.http_400_count;
        current_http_count.count_500 = current_http_count.count_500 + stats.http_500_count;

        let dto_msg = serde_json::to_string(&dto).unwrap();

        broadcast_message(state.clone(), dto_msg);
        truncate(file_path.clone());

        if current_run_time.elapsed() >= one_second {
            let dto_wrapper = model::BroadcastMessage {bm_type: model::BmStatusCodeCount::type_name(), payload: current_http_count.clone()};
            broadcast_message(state.clone(), serde_json::to_string(&dto_wrapper).unwrap());
            current_http_count = model::BmStatusCodeCount::new();
            current_run_time = Instant::now();
        }
    }
}

fn broadcast_message(state : AppState, msg : String) {
    // TODO: Clear clients that is no longer connected
    let clients = state.ws.clients.read().unwrap();
    for (key, client) in &(*clients) {
        if client.connected() {
            client.do_send(websocket::TextMessage::new(msg.clone()));
        }
    }
}

fn truncate(filename : String) {
    match File::open(filename.clone()) {
        Err(_) => {
            error!("Unable to find file {}", filename)
        },
        Ok(file) => {
            let length = file.metadata().unwrap().len();
            let bytes = Byte::from_str("1 GB").unwrap();
            let max_size = bytes.get_bytes();

            if length > max_size {
                info!("Truncating file {}", &filename);
                File::create(filename.clone()).unwrap();
            }
        }
    };
}

fn setup() {
    std::env::set_var("RUST_LOG", "trace,mio=info,hyper=info,actix_server=debug,actix_http=debug");
    pretty_env_logger::init();
}