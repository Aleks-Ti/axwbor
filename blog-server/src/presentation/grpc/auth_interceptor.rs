// use tonic::{Request, Status, service::Interceptor};

// use crate::infrastructure::jwt::JwtKeys;
// use crate::presentation::auth::extract_identity_from_token;

// #[derive(Clone)]
// pub struct JwtAuthInterceptor {
//     keys: JwtKeys,
// }

// impl JwtAuthInterceptor {
//     pub fn new(keys: JwtKeys) -> Self {
//         Self { keys }
//     }
// }

// impl Interceptor for JwtAuthInterceptor {
//     fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
//         let auth_header = match req.metadata().get("authorization") {
//             Some(v) => v.to_str().ok(),
//             None => None,
//         };

//         let Some(auth_header) = auth_header else {
//             return Ok(req);
//         };

//         let token = auth_header
//             .strip_prefix("Bearer ")
//             .ok_or_else(|| Status::unauthenticated("invalid authorization"))?;

//         let identity = extract_identity_from_token(token, &self.keys)
//             .map_err(|_| Status::unauthenticated("invalid token"))?;

//         req.extensions_mut().insert(identity);
//         Ok(req)
//     }
// }
