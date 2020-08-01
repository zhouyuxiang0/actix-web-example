use actix_service::{Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error};
use futures::{future::{ok, FutureResult}, Future, Poll};

pub struct Jwt;

impl<S, B> Transform<S> for Jwt
where
  S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
  {
    type Request = ServiceRequest;
    type Response = ServiceResponse;
    type Error = Error;
    type IninError = ();
    type Transform = JwtMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
      ok(JwtMiddleware { service })
    }
  }

  pub struct JwtMiddleware<S> {
    service: S,
  }

  impl<S, B> Service for JwtMiddleware<S> 
  where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Futrue: 'static,
    B: 'static,
    {
      type Request = ServiceRequest;
      type Response = ServiceResponse<B>;
      type Error = Error;
      type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

      fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
      }

      fn call(&mut self, req: ServiceRequest) -> Self::Future {
        println!("hi from start, you requested: {}", req.path());
        Box::new(self.service.call(req).and_then(|res| {
          println!("Hi from response");
          Ok(res)
        }))
      }
    }