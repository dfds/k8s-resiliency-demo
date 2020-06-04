use actix_web::{HttpServer, App, get, web, Responder, Error, Scope, HttpResponse};
use actix_web::dev::{Transform, Service, ServiceRequest, ServiceResponse};
use futures::future::{Ready, ok};
use std::pin::Pin;
use futures::Future;
use futures::task::{Context, Poll};
use http::header::HeaderName;
use http::HeaderValue;
use crate::model::AppState;
use std::sync::Arc;

#[actix_rt::main]
pub async fn serve(state : AppState) {
    let web_state = state;
    HttpServer::new(move || App::new()
        .data(web_state.clone())
        .service(web::resource("/ws/").to(crate::websocket::ws_index).wrap(ServerDefaults))
        .service(web::resource("/").to(index).wrap(ServerDefaults)))
        .bind("0.0.0.0:3100").unwrap()
        .run()
        .await;
}

pub fn scope() -> Scope {
    web::scope("/")
        .service(web::resource("/").to(index))
}

async fn index(state : web::Data<AppState>) -> impl Responder {
    let mut stats_raw = crate::model::Stats::new();
    {
        let stats = state.stats.read().unwrap();
        stats_raw.http_200_count = stats.http_200_count;
        stats_raw.http_400_count = stats.http_400_count;
        stats_raw.http_500_count = stats.http_500_count;
    }
    let payload = serde_json::to_string(&stats_raw).unwrap();
    HttpResponse::Ok().body(payload)
}

//
// ServerDefaults middleware
//

pub struct ServerDefaults;

impl<S, B> Transform<S> for ServerDefaults
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ServerDefaultsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ServerDefaultsMiddleware { service })
    }
}

pub struct ServerDefaultsMiddleware<S> {
    service: S
}

impl<S, B> Service for ServerDefaultsMiddleware<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            res.headers_mut().insert(HeaderName::from_static("server"), HeaderValue::from_static("k8s-traefik-stats"));
            res.headers_mut().insert(HeaderName::from_static("access-control-allow-origin"), HeaderValue::from_static("*"));

            Ok(res)
        })
    }
}