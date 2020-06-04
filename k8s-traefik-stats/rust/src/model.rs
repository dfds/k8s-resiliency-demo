use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use actix::Addr;
use crate::websocket::IndexWs;
use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct AppState {
    pub ws : Arc<Websocket>,
    pub stats : Arc<RwLock<Stats>>
}

impl AppState {
    pub fn new() -> Self {
        Self {
            ws: Arc::new(Websocket::new()),
            stats: Arc::new(RwLock::new(Stats::new()))
        }
    }

    pub fn update_stats(&self, bm_latency : BmLatency) {
        let data = Stats::from_bm_latency(bm_latency);
        let mut stats = self.stats.write().unwrap();
            stats.http_200_count = stats.http_200_count + data.http_200_count;
            stats.http_300_count = stats.http_300_count + data.http_300_count;
            stats.http_400_count = stats.http_400_count + data.http_400_count;
            stats.http_500_count = stats.http_500_count + data.http_500_count;
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Stats {
    pub http_200_count : u64,
    pub http_300_count : u64,
    pub http_400_count : u64,
    pub http_500_count : u64,
    pub average_latency: u64
}

impl Stats {
    pub fn new() -> Self {
        Self {
            http_200_count: 0,
            http_300_count: 0,
            http_400_count: 0,
            http_500_count: 0,
            average_latency: 0,
        }
    }

    pub fn from_bm_latency(bm_latency : BmLatency) -> Self {
        let status = bm_latency.status.unwrap();
        let mut stats = Self::new();
        if status >= 200 && status <= 299 {
            stats.http_200_count = 1;
        }
        if status >= 300 && status <= 399 {
            stats.http_300_count = 1;
        }
        if status >= 400 && status <= 499 {
            stats.http_400_count = 1;
        }
        if status >= 500 && status <= 599 {
            stats.http_500_count = 1;
        }
        stats
    }
}

#[derive(Clone)]
pub struct Websocket {
    pub clients : Arc<RwLock<HashMap<u64, Addr<IndexWs>>>>
}


impl Websocket {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn register_client(&self, addr : Addr<IndexWs>) {
        let mut clients = self.clients.clone();
        let mut lock = clients.write().unwrap();
        let mut rng = rand::thread_rng();
        (*lock).insert(rng.gen::<u64>(), addr);
    }

    pub fn remove_client(&self, id : u64) {
        let mut clients = self.clients.clone();
        let mut lock = clients.write().unwrap();
        (*lock).remove(&id);
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BroadcastMessage<T> {
    pub bm_type : String,
    pub payload : T
}

pub trait Bm {
    fn type_name() -> String;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BmLatency {
    pub host : Option<String>,
    pub path : Option<String>,
    pub method : Option<String>,
    pub duration : Option<i64>,
    pub time : Option<String>,
    pub status : Option<i64>,
}

impl Bm for BmLatency {
    fn type_name() -> String {
        "latency".to_owned()
    }
}


impl BmLatency {
    pub fn from_log(val : TraefikLog) -> Self {
        Self {
            host: val.request_host,
            path: val.request_path,
            method: val.request_method,
            duration: val.duration,
            time: val.time,
            status: val.downstream_status
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BmStatusCodeCount {
    pub count_200 : u64,
    pub count_300 : u64,
    pub count_400 : u64,
    pub count_500 : u64
}

impl BmStatusCodeCount {
    pub fn new() -> Self {
        Self {
            count_200: 0,
            count_300: 0,
            count_400: 0,
            count_500: 0,
        }
    }
}

impl Bm for BmStatusCodeCount {
    fn type_name() -> String {
        "statusCodeCount".to_owned()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TraefikLog {
    #[serde(rename = "BackendAddr")]
    pub backend_addr: Option<String>,
    #[serde(rename = "BackendName")]
    pub backend_name: Option<String>,
    #[serde(rename = "BackendURL")]
    pub backend_url: Option<BackendUrl>,
    #[serde(rename = "ClientAddr")]
    pub client_addr: Option<String>,
    #[serde(rename = "ClientHost")]
    pub client_host: Option<String>,
    #[serde(rename = "ClientPort")]
    pub client_port: Option<String>,
    #[serde(rename = "ClientUsername")]
    pub client_username: Option<String>,
    #[serde(rename = "DownstreamContentSize")]
    pub downstream_content_size: Option<i64>,
    #[serde(rename = "DownstreamStatus")]
    pub downstream_status: Option<i64>,
    #[serde(rename = "DownstreamStatusLine")]
    pub downstream_status_line: Option<String>,
    #[serde(rename = "Duration")]
    pub duration: Option<i64>,
    #[serde(rename = "FrontendName")]
    pub frontend_name: Option<String>,
    #[serde(rename = "OriginContentSize")]
    pub origin_content_size: Option<i64>,
    #[serde(rename = "OriginDuration")]
    pub origin_duration: Option<i64>,
    #[serde(rename = "OriginStatus")]
    pub origin_status: Option<i64>,
    #[serde(rename = "OriginStatusLine")]
    pub origin_status_line: Option<String>,
    #[serde(rename = "Overhead")]
    pub overhead: Option<i64>,
    #[serde(rename = "RequestAddr")]
    pub request_addr: Option<String>,
    #[serde(rename = "RequestContentSize")]
    pub request_content_size: Option<i64>,
    #[serde(rename = "RequestCount")]
    pub request_count: Option<i64>,
    #[serde(rename = "RequestHost")]
    pub request_host: Option<String>,
    #[serde(rename = "RequestLine")]
    pub request_line: Option<String>,
    #[serde(rename = "RequestMethod")]
    pub request_method: Option<String>,
    #[serde(rename = "RequestPath")]
    pub request_path: Option<String>,
    #[serde(rename = "RequestPort")]
    pub request_port: Option<String>,
    #[serde(rename = "RequestProtocol")]
    pub request_protocol: Option<String>,
    #[serde(rename = "RetryAttempts")]
    pub retry_attempts: Option<i64>,
    #[serde(rename = "StartLocal")]
    pub start_local: Option<String>,
    #[serde(rename = "StartUTC")]
    pub start_utc: Option<String>,
    #[serde(rename = "downstream_Content-Type")]
    pub downstream_content_type: Option<String>,
    #[serde(rename = "downstream_Vary")]
    pub downstream_vary: Option<String>,
    #[serde(rename = "downstream_X-Content-Type-Options")]
    pub downstream_x_content_type_options: Option<String>,
    pub level: Option<String>,
    pub msg: Option<String>,
    #[serde(rename = "origin_Content-Type")]
    pub origin_content_type: Option<String>,
    #[serde(rename = "origin_Vary")]
    pub origin_vary: Option<String>,
    #[serde(rename = "origin_X-Content-Type-Options")]
    pub origin_x_content_type_options: Option<String>,
    #[serde(rename = "request_Connection")]
    pub request_connection: Option<String>,
    #[serde(rename = "request_Content-Length")]
    pub request_content_length: Option<String>,
    #[serde(rename = "request_Content-Type")]
    pub request_content_type: Option<String>,
    #[serde(rename = "request_X-Forwarded-For")]
    pub request_x_forwarded_for: Option<String>,
    #[serde(rename = "request_X-Forwarded-Port")]
    pub request_x_forwarded_port: Option<String>,
    #[serde(rename = "request_X-Forwarded-Proto")]
    pub request_x_forwarded_proto: Option<String>,
    pub time: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BackendUrl {
    #[serde(rename = "Scheme")]
    pub scheme: Option<String>,
    #[serde(rename = "Opaque")]
    pub opaque: Option<String>,
    #[serde(rename = "User")]
    pub user: Option<serde_json::Value>,
    #[serde(rename = "Host")]
    pub host: Option<String>,
    #[serde(rename = "Path")]
    pub path: Option<String>,
    #[serde(rename = "RawPath")]
    pub raw_path: Option<String>,
    #[serde(rename = "ForceQuery")]
    pub force_query: Option<bool>,
    #[serde(rename = "RawQuery")]
    pub raw_query: Option<String>,
    #[serde(rename = "Fragment")]
    pub fragment: Option<String>,
}
