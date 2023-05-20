// use rocket::{response::{Responder, self}, Request, http::Status, Response as RocketResponse, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BizResp<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}


// impl<'r, 'o: 'r, T: Responder<'r, 'o>> Responder<'r, 'o> for Response<Json<T>> {
//     fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
//         let mut build = RocketResponse::build();
//         if let Some(responder) = self.data {
//             build.merge(responder.respond_to(req)?);
//         }

//         build.status(Status::Ok).ok()
//     }
// }
