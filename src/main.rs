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

mod api;
mod bidtracker;
mod config;
mod errors;

use actix_web::{middleware, web, App, HttpServer};
use api::custom_error_handler;

use anyhow::{self, Context};
use bidtracker::BidTracker;
use config::Config;
use std::{env, sync::Mutex};
use tracing_subscriber::{self, EnvFilter};

async fn spawn_server(
	config: &Config,
	bidtracker: web::Data<Mutex<bidtracker::BidManagement>>,
) -> Result<(), std::io::Error> {
	HttpServer::new(move || {
		App::new()
			.app_data(bidtracker.clone())
			.app_data(web::PathConfig::default().error_handler(custom_error_handler::uuid_error_handler))
			.app_data(web::JsonConfig::default().error_handler(custom_error_handler::json_error_handler))
			.wrap(middleware::Logger::default())
			.wrap(middleware::Compress::default())
			.service(web::resource("/healthz").route(web::get().to(|| async { "Healthy bruh" })))
			.service(
				web::scope("/api/v1")
					.route(api::routes::URL_BID_ITEM, web::post().to(api::post_bid_new))
					.route(api::routes::URL_BID_GET_ALL, web::get().to(api::get_bids))
					.route(
						api::routes::URL_BID_GET_WINNING,
						web::get().to(api::get_current_winning_bid),
					)
					.route(api::routes::URL_USER_GET_ALL_BIDS, web::get().to(api::get_user_bids)),
			)
	})
	.bind(&config.address)?
	.run()
	.await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	ctrlc::set_handler(move || {
		use std::process::exit;
		exit(0);
	})
	.expect("Error setting Ctrl-C handler");

	if env::var_os("RUST_LOG").is_none() {
		env::set_var("RUST_LOG", "bid_tracker_rs=info");
	}
	tracing_subscriber::fmt()
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	let config = Config::new();

	let biddable_items = vec![
		uuid::Uuid::parse_str("b2f9ee6d-79fe-4b14-9c19-35a69a89219a").unwrap(),
		uuid::Uuid::parse_str("b16ab43e-aa13-4079-b8c5-592e81312c01").unwrap(),
	];
	let bidmanagement = web::Data::new(Mutex::new(bidtracker::BidManagement::new(biddable_items)));
	tracing::info!("Spawning server on {}", &config.address);
	spawn_server(&config, bidmanagement)
		.await
		.context(format!("Failed to launch the server on {}", &config.address))?;

	Ok(())
}
