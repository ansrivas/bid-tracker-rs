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
use super::response::send_json;
use crate::{
	bidtracker::{Bid, BidManagement, BidTracker},
	errors::BidTrackerError,
};
use std::sync::Mutex;

use actix_web::{error, http::StatusCode, web, Error as ActixErr, HttpResponse};

pub async fn post_bid_new(
	bid: web::Json<Bid>,
	bidtracker: web::Data<Mutex<BidManagement>>,
) -> Result<HttpResponse, ActixErr> {
	let mut bdm = bidtracker.lock().unwrap();

	let bbid = bid.into_inner();
	bdm.insert_bid(&bbid).map_err(|source| match source {
		BidTrackerError::ItemNotBiddable(_e) => {
			error::ErrorUnprocessableEntity(format!("Failed to insert bid into the db. {:?}", _e))
		}
		_ => error::ErrorInternalServerError(format!("Failed to insert bid into the db. {:?}", source.to_string())),
	})?;

	send_json(StatusCode::OK, "Returning from post_bid_new bids", &bbid)
}

/// Get all the bids for the given itemuuid
pub async fn get_bids(
	item_uuid: web::Path<uuid::Uuid>,
	bidtracker: web::Data<Mutex<BidManagement>>,
) -> Result<HttpResponse, ActixErr> {
	let bdm = bidtracker.lock().unwrap();
	let bids = bdm.get_bids(&item_uuid).map_err(|source| match source {
		BidTrackerError::ItemNotBiddable(_e) => {
			error::ErrorUnprocessableEntity(format!("Failed to get the bids. {:?}", _e))
		}
		_ => error::ErrorInternalServerError(format!("Failed to get the bids. {:?}", source.to_string())),
	})?;

	send_json(StatusCode::OK, "Returning from get_handler bids", &bids)
}

/// Get the current winning bid for a given itemuuid
pub async fn get_current_winning_bid(
	item_uuid: web::Path<uuid::Uuid>,
	bidtracker: web::Data<Mutex<BidManagement>>,
) -> Result<HttpResponse, ActixErr> {
	let bdm = bidtracker.lock().unwrap();
	let bids = bdm.current_winning_bid(&item_uuid).map_err(|source| match source {
		BidTrackerError::ItemNotBiddable(_e) => {
			error::ErrorUnprocessableEntity(format!("Failed to get current winning bids for. {:?}", _e))
		}
		_ => error::ErrorInternalServerError(format!(
			"Failed to get current winning bids for. {:?}",
			source.to_string()
		)),
	})?;

	send_json(StatusCode::OK, "Returning from get_current_winning_bid", &bids)
}

/// Get all the bids from a given user uuid
pub async fn get_user_bids(
	user_uuid: web::Path<uuid::Uuid>,
	bidtracker: web::Data<Mutex<BidManagement>>,
) -> Result<HttpResponse, ActixErr> {
	let bdm = bidtracker.lock().unwrap();
	let bids = bdm.get_bids_by_user(&user_uuid).map_err(|source| match source {
		BidTrackerError::ItemNotBiddable(_e) => {
			error::ErrorUnprocessableEntity(format!("Failed to get current winning bids {:?}", _e))
		}
		_ => error::ErrorInternalServerError(format!("Failed to get current winning bids {:?}", source.to_string())),
	})?;
	send_json(StatusCode::OK, "Returning from get_user_bids", &bids)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		api::{ResponseMessageBid, ResponseMessageBids},
		bidtracker,
	};
	use actix_web::{http, test, App};

	#[actix_rt::test]
	async fn test_post_bid_new() {
		let bid = Bid {
			item_uuid: uuid::Uuid::parse_str("b2f9ee6d-79fe-4b14-9c19-35a69a89219a").unwrap(),
			user_uuid: uuid::Uuid::parse_str("b16ab43e-aa13-4079-b8c5-592e81312c01").unwrap(),
			timestamp: 12312321321,
			amount: 30f64,
		};

		let non_existant_bid = Bid {
			item_uuid: uuid::Uuid::parse_str("7f272d43-0ff2-4e0f-9ebc-589eae48e3ad").unwrap(),
			user_uuid: uuid::Uuid::parse_str("b16ab43e-aa13-4079-b8c5-592e81312c01").unwrap(),
			timestamp: 12312321321,
			amount: 30f64,
		};

		let srv = test::start(move || {
			let biddable_items = vec![
				uuid::Uuid::parse_str("b2f9ee6d-79fe-4b14-9c19-35a69a89219a").unwrap(),
				uuid::Uuid::parse_str("b16ab43e-aa13-4079-b8c5-592e81312c01").unwrap(),
			];
			let bidmanagement = web::Data::new(Mutex::new(bidtracker::BidManagement::new(biddable_items)));
			App::new()
				.app_data(bidmanagement)
				.route("/", web::post().to(post_bid_new))
		});

		let response = srv.post("/").send_json(&non_existant_bid).await.unwrap();
		assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);

		let response = srv.post("/").send_json(&bid).await.unwrap();
		assert_eq!(response.status(), http::StatusCode::OK);
	}

	#[actix_rt::test]
	async fn test_get_bids() {
		let bid1 = Bid {
			item_uuid: uuid::Uuid::parse_str("b2f9ee6d-79fe-4b14-9c19-35a69a89219a").unwrap(),
			user_uuid: uuid::Uuid::parse_str("b16ab43e-aa13-4079-b8c5-592e81312c01").unwrap(),
			timestamp: 12312321321,
			amount: 30f64,
		};

		let bid2 = Bid {
			item_uuid: uuid::Uuid::parse_str("b2f9ee6d-79fe-4b14-9c19-35a69a89219a").unwrap(),
			user_uuid: uuid::Uuid::parse_str("b16ab43e-aa13-4079-b8c5-592e81312c01").unwrap(),
			timestamp: 12312321321,
			amount: 30f64,
		};

		// let srv = test::start(move || {
		let srv = test::start(move || {
			let biddable_items = vec![
				uuid::Uuid::parse_str("b2f9ee6d-79fe-4b14-9c19-35a69a89219a").unwrap(),
				uuid::Uuid::parse_str("b16ab43e-aa13-4079-b8c5-592e81312c01").unwrap(),
			];
			let bidmanagement = web::Data::new(Mutex::new(bidtracker::BidManagement::new(biddable_items)));
			App::new()
				.app_data(bidmanagement)
				.route("/", web::post().to(post_bid_new))
				.route("/{itemuuid}", web::get().to(get_bids))
		});

		let _response = srv.post("/").send_json(&bid1).await.unwrap();
		let _response = srv.post("/").send_json(&bid2).await.unwrap();

		let mut response = srv.get("/b2f9ee6d-79fe-4b14-9c19-35a69a89219a").send().await.unwrap();
		let res: ResponseMessageBids = response.json().await.unwrap();
		assert_eq!(res.data.len(), 2);

		// Missing uuid case
		let response = srv.get("/d60da647-9b9b-43db-97af-56760afa6d93").send().await.unwrap();
		assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
	}

	#[actix_rt::test]
	async fn test_get_current_winning_bid() {
		let bid1 = Bid {
			item_uuid: uuid::Uuid::parse_str("b2f9ee6d-79fe-4b14-9c19-35a69a89219a").unwrap(),
			user_uuid: uuid::Uuid::parse_str("b16ab43e-aa13-4079-b8c5-592e81312c01").unwrap(),
			timestamp: 12312321321,
			amount: 30f64,
		};

		let bid2 = Bid {
			item_uuid: uuid::Uuid::parse_str("b2f9ee6d-79fe-4b14-9c19-35a69a89219a").unwrap(),
			user_uuid: uuid::Uuid::parse_str("b16ab43e-aa13-4079-b8c5-592e81312c01").unwrap(),
			timestamp: 12312321321,
			amount: 32.5f64,
		};

		// let srv = test::start(move || {
		let srv = test::start(move || {
			let biddable_items = vec![
				uuid::Uuid::parse_str("b2f9ee6d-79fe-4b14-9c19-35a69a89219a").unwrap(),
				uuid::Uuid::parse_str("b16ab43e-aa13-4079-b8c5-592e81312c01").unwrap(),
			];
			let bidmanagement = web::Data::new(Mutex::new(bidtracker::BidManagement::new(biddable_items)));
			App::new()
				.app_data(bidmanagement)
				.route("/", web::post().to(post_bid_new))
				.route("/{itemuuid}/winning", web::get().to(get_current_winning_bid))
		});

		let _response = srv.post("/").send_json(&bid1).await.unwrap();
		let _response = srv.post("/").send_json(&bid2).await.unwrap();

		let mut response = srv
			.get("/b2f9ee6d-79fe-4b14-9c19-35a69a89219a/winning")
			.send()
			.await
			.unwrap();
		let res: ResponseMessageBid = response.json().await.unwrap();
		assert_eq!(res.data.amount, 32.5);

		// Missing uuid case
		let response = srv
			.get("/d60da647-9b9b-43db-97af-56760afa6d93/winning")
			.send()
			.await
			.unwrap();
		assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
	}

	#[actix_rt::test]
	async fn test_get_user_bids() {
		let bid1 = Bid {
			item_uuid: uuid::Uuid::parse_str("b2f9ee6d-79fe-4b14-9c19-35a69a89219a").unwrap(),
			user_uuid: uuid::Uuid::parse_str("1c916ab6-255b-4a36-9574-e456e0f774c9").unwrap(),
			timestamp: 12312321321,
			amount: 30f64,
		};

		let bid2 = Bid {
			item_uuid: uuid::Uuid::parse_str("b16ab43e-aa13-4079-b8c5-592e81312c01").unwrap(),
			user_uuid: uuid::Uuid::parse_str("1c916ab6-255b-4a36-9574-e456e0f774c9").unwrap(),
			timestamp: 12312321321,
			amount: 32.5f64,
		};

		// let srv = test::start(move || {
		let srv = test::start(move || {
			let biddable_items = vec![
				uuid::Uuid::parse_str("b2f9ee6d-79fe-4b14-9c19-35a69a89219a").unwrap(),
				uuid::Uuid::parse_str("b16ab43e-aa13-4079-b8c5-592e81312c01").unwrap(),
			];
			let bidmanagement = web::Data::new(Mutex::new(bidtracker::BidManagement::new(biddable_items)));
			App::new()
				.app_data(bidmanagement)
				.route("/", web::post().to(post_bid_new))
				.route("/{useruuid}/bids", web::get().to(get_user_bids))
		});

		let _response = srv.post("/").send_json(&bid1).await.unwrap();
		let _response = srv.post("/").send_json(&bid2).await.unwrap();

		let mut response = srv
			.get("/1c916ab6-255b-4a36-9574-e456e0f774c9/bids")
			.send()
			.await
			.unwrap();
		let res: ResponseMessageBids = response.json().await.unwrap();
		assert_eq!(res.data.len(), 2);

		// Missing uuid case
		let response = srv
			.get("/17ec66e3-4971-4912-824e-f8533a285857/bids")
			.send()
			.await
			.unwrap();
		assert_eq!(response.status(), http::StatusCode::UNPROCESSABLE_ENTITY);
	}
}
