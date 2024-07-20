use std::{future::{ready, Ready}, time::Instant};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

pub struct Timing;

impl<S, B> Transform<S, ServiceRequest> for Timing
where
    // S 必须实现 Service trait，其响应类型为 ServiceResponse<B>，错误类型为 Error
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TimingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        // 接收一个服务 service，并返回一个 future，这个 future 在解析时会产生一个 TimingMiddleware 实例或者一个初始化错误。
        ready(Ok(TimingMiddleware { service }))
    }
}

pub struct TimingMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for TimingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    // 将 poll_ready 方法的调用转发到内部服务。
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // 记录开始时间
        let start_time = Instant::now();
        let path = req.path().to_string();
        let method = req.method().to_string();
        let remote_addr = req.connection_info().peer_addr().unwrap_or("Undefined Addr").to_string();
        let version = format!("{:?}", req.version());
        let headers = req.headers().clone();

        // 调用内部服务
        let fut = self.service.call(req);

        // 返回一个 future，这个 future 在解析时会产生一个 ServiceResponse 实例或者一个错误。
        let fut = async move {
            let res = fut.await?;
            // 记录结束时间
            let elapsed = start_time.elapsed();
            let status_code = res.status();
            let content_length = res
                .headers()
                .get(actix_web::http::header::CONTENT_LENGTH)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("-");
            let user_agent = headers
                .get(actix_web::http::header::USER_AGENT)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("-");

            // 将时间差转换为毫秒
            let elapsed_ms = elapsed.as_millis();
            println!(
                "{} {} {} {} {} {} {} {}",
                remote_addr, method, path, version, status_code, content_length, elapsed_ms, user_agent
            );

            Ok(res)
        };

        Box::pin(fut)
    }
}
