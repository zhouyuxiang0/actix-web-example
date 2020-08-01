#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::test;

  #[test]
  fn test_static_handler() {
    let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
    let resp = test::block_on()
  }
}