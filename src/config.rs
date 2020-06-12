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

use serde::Deserialize;
use std::env;

const DEFAULT_CONFIG_ENV_KEY: &str = "BID_TRACKER_CONFIG_PATH";
const CONFIG_PREFIX: &str = "BID_TRACKER_";

struct ConfigFn {}

impl ConfigFn {
	#[allow(dead_code)]
	fn fn_false() -> bool {
		false
	}
	fn fn_true() -> bool {
		true
	}
	fn fn_default_address() -> String {
		"0.0.0.0:3000".into()
	}

	#[allow(dead_code)]
	fn fn_empty_string() -> String {
		"".into()
	}
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Config {
	// Run the app in debug mode
	#[serde(default = "ConfigFn::fn_true")]
	pub debug: bool,

	// Set the address to bind the webserver on
	// defaults to 0.0.0.0:8080
	#[serde(default = "ConfigFn::fn_default_address")]
	pub address: String,
}

impl Config {
	// Create a new Config instance by reading from
	// environment variables
	pub fn new() -> Config {
		// Check if there is an environment variable BID_TRACKER_CONFIG_PATH
		// then read it from there else fallback to .env
		match env::var(DEFAULT_CONFIG_ENV_KEY) {
			Ok(val) => dotenv::from_filename(val).ok(),
			Err(_) => dotenv::dotenv().ok(),
		};

		match envy::prefixed(CONFIG_PREFIX).from_env::<Config>() {
			Ok(config) => config,
			Err(error) => panic!("Failed to read config. Error: {error}", error = error),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn eq_with_nan_eq(a: &Config, b: &Config) -> bool {
		return (a.address == b.address) && (a.debug == b.debug);
	}

	fn vec_compare(va: &[Config], vb: &[Config]) -> bool {
		// zip stops at the shortest
		(va.len() == vb.len()) && va.iter().zip(vb).all(|(a, b)| eq_with_nan_eq(a, b))
	}

	#[test]
	fn test_config_parsing() {
		let json = r#"
	[
	  {
		"debug": true
	  },
	  {
		  "address": "0.0.0.0:9080",
		  "debug": false
	  }
	]
"#;
		let config: Vec<Config> = serde_json::from_str(json).unwrap();

		let expected_config: Vec<Config> = vec![
			Config {
				debug: true,
				address: "0.0.0.0:3000".into(),
			},
			Config {
				debug: false,
				address: "0.0.0.0:9080".into(),
			},
		];
		assert_eq!(
			vec_compare(&config, &expected_config),
			true,
			"Parsing failed !!! {:?}",
			config
		);
	}

	#[test]
	fn test_config_reading() {
		// better_panic::Settings::debug()
		// 	.most_recent_first(false)
		// 	.install();
		let path = env::var("CARGO_MANIFEST_DIR");
		let env_file = format!("{}/tests/data/test.env", path.unwrap());
		env::set_var(DEFAULT_CONFIG_ENV_KEY, env_file);
		let config: Config = Config::new();
		assert!(config.address == "0.0.0.0:9091");
		assert!(config.debug);
	}
}
