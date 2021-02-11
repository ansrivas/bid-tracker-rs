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

use std::borrow::Borrow;

use crate::bidtracker::Bid;
use actix_web::{http::StatusCode, web, Error as ActixErr, HttpResponse};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseMessage<T: Serialize + ?Sized> {
	pub code: u16,
	pub message: String,
	pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseMessageBid {
	pub code: u16,
	pub message: String,
	pub data: Bid,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseMessageBids {
	pub code: u16,
	pub message: String,
	pub data: Vec<Bid>,
}

pub fn send_json<T>(status_code: StatusCode, message: &str, data: &T) -> Result<HttpResponse, ActixErr>
where
	T: Serialize + ?Sized,
{
	let json = ResponseMessage {
		code: status_code.as_u16(),
		message: message.into(),
		data: data,
	};
	Ok(web::HttpResponse::build(StatusCode::from_u16(status_code.as_u16()).unwrap()).json(json.borrow()))
}
