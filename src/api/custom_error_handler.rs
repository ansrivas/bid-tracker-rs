// MIT License
//
// Copyright (c) 2020 Ankur Srivastava
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use actix_web::{error, http::StatusCode, HttpRequest, HttpResponse};

use super::ResponseMessage;

// Custom error handler in case there is an uuid parse error
pub fn uuid_error_handler(err: error::PathError, _req: &HttpRequest) -> error::Error {
	log::debug!("Failed to parse incoming path component");

	let detail = err.to_string();
	let rm = ResponseMessage {
		code: StatusCode::BAD_REQUEST.as_u16(),
		message: detail,
		data: "",
	};
	let resp = HttpResponse::build(StatusCode::BAD_REQUEST).json(&rm);
	error::InternalError::from_response(err, resp.into()).into()
}

// Custom error handler in case there is json payload decoding error
pub fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
	use actix_web::error::JsonPayloadError;
	log::debug!("Failed to parse incoming json component");

	let detail = err.to_string();
	let resp = match &err {
		JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body(detail),
		JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
			// HttpResponse::UnprocessableEntity().body(detail)
			let rm = ResponseMessage {
				code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
				message: detail,
				data: "",
			};
			// HttpResponse::UnprocessableEntity().body(detail)}
			HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(&rm)
		}
		_ => HttpResponse::BadRequest().body(detail),
	};
	error::InternalError::from_response(err, resp.into()).into()
}
