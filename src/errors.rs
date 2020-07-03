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

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BidTrackerError {
	#[error("Requested item is not present in bidding list: {0}")]
	ItemNotBiddable(String),
	#[error("IO error encountered")]
	Io { source: std::io::Error },
	#[error("IO error encountered")]
	Actix { source: actix_web::error::Error },
}

impl From<BidTrackerError> for actix_web::error::Error {
	fn from(e: BidTrackerError) -> actix_web::error::Error {
		match e {
			BidTrackerError::ItemNotBiddable(_e) => {
				actix_web::error::ErrorUnprocessableEntity(format!("Failed to process the bid the db. {:?}", _e))
			}
			_ => actix_web::error::ErrorInternalServerError(format!("Failed to get the bids. {:?}", e.to_string())),
		}
	}
}
