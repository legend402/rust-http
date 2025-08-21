use std::{future::{ready, Ready, Future}, pin::Pin};

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, error, web, Error, HttpResponse};
use crate::utils::Response;

pub struct Authentication;

// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    /// The next service to call
    service: S,
}

// This future doesn't have the requirement of being `Send`.
// See: futures_util::future::LocalBoxFuture
type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

// `S`: type of the wrapped service
// `B`: type of the body - try to be generic over the body where possible
impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    // This service is ready when its next service is ready
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let header = req.headers().to_owned();
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await;
            match header.get("token") {
                Some(token) => {
                    println!("token:{}", token.to_str().unwrap());
                },
                None => {
                    println!("token is None");
                    return Err(error::ErrorUnauthorized(Response::<()>::unauthorized("token is None".to_string()).to_serialize()));
                },
            }
            res
        })
    }
}
