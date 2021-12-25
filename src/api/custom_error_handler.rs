// MIT License
//
// Copyright (c) 2021 Ankur Srivastava
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
	tracing::debug!("Failed to parse incoming path component");

	let detail = err.to_string();
	let rm = ResponseMessage {
		code: StatusCode::BAD_REQUEST.as_u16(),
		message: detail,
		data: "",
	};
	let resp = HttpResponse::build(StatusCode::BAD_REQUEST).json(&rm);
	error::InternalError::from_response(err, resp).into()
}

// Custom error handler in case there is json payload decoding error
pub fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
	use actix_web::error::JsonPayloadError;
	tracing::debug!("Failed to parse incoming json component");

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
	error::InternalError::from_response(err, resp).into()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::api::handler::{get_bids, post_bid_new};
	use actix_web::{dev::Service, test::TestRequest};
	use actix_web::{http, test, web, App};

	#[actix_rt::test]
	async fn test_custom_error_handler() {
		let srv = test::init_service(
			App::new()
				// .app_data(bidmanagement)
				.app_data(web::PathConfig::default().error_handler(uuid_error_handler))
				.app_data(web::JsonConfig::default().error_handler(json_error_handler))
				.route("/", web::post().to(post_bid_new))
				.route("/{itemuuid}", web::get().to(get_bids)),
		)
		.await;

		let malformed_json = serde_json::json!({
			"something": "random"
		});
		let req = TestRequest::post().uri("/").set_json(&malformed_json).to_request();
		let response = srv.call(req).await.unwrap();
		assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);

		let malformed_uuid = "malformed_uuid";
		let req = TestRequest::get().uri(&format!("/{}", malformed_uuid)).to_request();
		let response = srv.call(req).await.unwrap();
		assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
	}
}
