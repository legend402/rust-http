use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub code: u16,
    pub msg: String,
    pub data: T,
}

impl<T: Serialize + Default> Response<T> {
    pub fn new(code: StatusCode, msg: String, data: T) -> Response<T> {
        Response {
            code: code.as_u16(),
            msg,
            data,
        }
    }
    pub fn success(data: T) -> Response<T> {
        Response::new(StatusCode::OK, "success".to_string(), data)
    }
    pub fn server_error(msg: String) -> Response<T> {
        Response::new(StatusCode::INTERNAL_SERVER_ERROR, msg, Default::default())
    }
    pub fn not_found(msg: String) -> Response<T> {
        Response::new(StatusCode::NOT_FOUND, msg, Default::default())
    }
    pub fn unauthorized(msg: String) -> Response<T> {
        Response::new(StatusCode::UNAUTHORIZED, msg, Default::default())
    }
    pub fn set_code(&mut self, code: StatusCode) {
        self.code = code.as_u16();
    }

    pub fn set_msg(&mut self, msg: String) {
        self.msg = msg;
    }

    pub fn set_data(&mut self, data: T) {
        self.data = data;
    }
    pub fn to_serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

// 为 Response<T> 实现 Responder 特征
impl<T: Serialize> Responder for Response<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        // 将 Response 结构体序列化为 JSON 字符串
        let body = serde_json::to_string(&self).unwrap();

        // 根据 Response 中的 code 字段设置 HTTP 状态码
        let status = StatusCode::from_u16(self.code)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR); // 无效状态码时默认 500

        // 构建 HTTP 响应：设置状态码、Content-Type 为 JSON、响应体为序列化后的 JSON
        HttpResponse::build(status)
            .content_type("application/json")
            .body(body)
    }

}