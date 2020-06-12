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

use super::BidTracker;
use crate::errors::BidTrackerError;
use anyhow::{self, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Bid {
	#[serde(rename = "itemuuid")]
	pub item_uuid: uuid::Uuid,
	#[serde(rename = "useruuid")]
	pub user_uuid: uuid::Uuid,
	pub timestamp: i64,
	pub amount: f64,
}

// ItemBidState represents the current state of an item
#[derive(Deserialize, Serialize, Default, Debug)]
pub struct ItemBidState {
	pub item_uuid: uuid::Uuid,
	pub bids: Vec<Bid>,
	pub current_winning_bid: Option<Bid>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BidManagement {
	user_bids: HashMap<uuid::Uuid, Vec<Bid>>,
	items: HashMap<uuid::Uuid, ItemBidState>,
}

impl BidTracker for BidManagement {
	fn new(allowed_item_uuid: Vec<uuid::Uuid>) -> Self {
		let mut items = HashMap::with_capacity(allowed_item_uuid.len());
		for item_uuid in allowed_item_uuid {
			items.insert(item_uuid, ItemBidState::default());
		}
		BidManagement {
			items,
			user_bids: HashMap::new(),
		}
	}

	/// Insert a bid in the internal hashmap
	fn insert_bid(&mut self, bid: &Bid) -> Result<(), BidTrackerError> {
		if let Some(existing) = self.items.get_mut(&bid.item_uuid) {
			existing.bids.push(bid.clone());

			match existing.current_winning_bid.as_ref() {
				Some(cur_bid) => {
					// We update the bid status _only_ if the bid amount is larger
					// in case the bids are equal, the previous bid will be the winner
					if cur_bid.amount < bid.amount {
						existing.current_winning_bid = Some(bid.clone());
					}
				}
				None => existing.current_winning_bid = Some(bid.clone()),
			}
		} else {
			return Err(BidTrackerError::ItemNotBiddable(
				"Requested item_uuid is not available for bidding".into(),
			));
		}

		if let Some(existing) = self.user_bids.get_mut(&bid.user_uuid) {
			existing.push(bid.clone());
		} else {
			self.user_bids.insert(bid.user_uuid, vec![bid.clone()]);
		}

		Ok(())
	}

	/// Get the current winning bid for a given itemuuid.
	fn current_winning_bid(&self, item_uuid: &uuid::Uuid) -> Result<Bid, BidTrackerError> {
		if let Some(bid_state) = self.items.get(item_uuid) {
			bid_state.current_winning_bid.clone().ok_or_else(|| {
				BidTrackerError::ItemNotBiddable("Requested item_uuid is not available for bidding".into())
			})
		} else {
			Err(BidTrackerError::ItemNotBiddable(
				"Requested item_uuid is not available for bidding".into(),
			))
		}
	}

	/// Get all the bids associated with this item_uuid
	fn get_bids(&self, item_uuid: &uuid::Uuid) -> Result<Vec<Bid>, BidTrackerError> {
		if let Some(bid_state) = self.items.get(item_uuid) {
			Ok(bid_state.bids.clone())
		} else {
			Err(BidTrackerError::ItemNotBiddable(
				"Requested item_uuid is not available for bidding".into(),
			))
		}
	}

	/// Get all the bids associated with a user_uuid
	fn get_bids_by_user(&self, user_uuid: &uuid::Uuid) -> Result<Vec<Bid>, BidTrackerError> {
		if let Some(user_bids) = self.user_bids.get(user_uuid) {
			Ok(user_bids.clone())
		} else {
			return Err(BidTrackerError::ItemNotBiddable(
				"Requested user_uuid is not available for bidding".into(),
			));
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_insert_bid() {
		let item_uuid = uuid::Uuid::parse_str("1cb396fd-3242-40ce-aaa1-8e8337c70435").unwrap();
		let mut bm = BidManagement::new(vec![item_uuid.clone()]);

		let bid = Bid {
			user_uuid: uuid::Uuid::parse_str("e5129c2c-718e-4ce6-b327-e74855967ab8").unwrap(),
			item_uuid: item_uuid.clone(),
			timestamp: 1591915318,
			amount: 30f64,
		};
		bm.insert_bid(&bid).unwrap();

		assert!(bm.user_bids.len() > 0);
		assert!(bm.items.len() > 0);
		assert!(bm.items.get(&item_uuid).unwrap().bids.len() > 0);

		// When non-allowed item_uuid is inserted
		let non_allowed = uuid::Uuid::parse_str("a17b364f-0a5d-4e6e-91dd-0fa38c11b4ea").unwrap();
		let bid2 = Bid {
			item_uuid: non_allowed.clone(),
			..bid
		};
		assert!(bm.insert_bid(&bid2).is_err());
	}

	#[test]
	fn test_current_winning_bid() {
		let item_uuid = uuid::Uuid::parse_str("1cb396fd-3242-40ce-aaa1-8e8337c70435").unwrap();
		let mut bm = BidManagement::new(vec![item_uuid.clone()]);

		let bid = Bid {
			user_uuid: uuid::Uuid::parse_str("e5129c2c-718e-4ce6-b327-e74855967ab8").unwrap(),
			item_uuid: item_uuid.clone(),
			timestamp: 1591915318,
			amount: 30f64,
		};

		// When wrong item_uuid requested
		let non_existent_uuid = uuid::Uuid::parse_str("a17b364f-0a5d-4e6e-91dd-0fa38c11b4ea").unwrap();
		assert!(bm.current_winning_bid(&non_existent_uuid).is_err());

		// When correct item without any bids requested
		assert!(bm.current_winning_bid(&item_uuid).is_err());

		// When correct item with bid requested
		bm.insert_bid(&bid).unwrap();
		assert!(bm.current_winning_bid(&item_uuid).unwrap().item_uuid == item_uuid);
	}

	#[test]
	fn test_get_bids() {
		let item_uuid = uuid::Uuid::parse_str("1cb396fd-3242-40ce-aaa1-8e8337c70435").unwrap();
		let mut bm = BidManagement::new(vec![item_uuid.clone()]);

		let user_uuid1 = uuid::Uuid::parse_str("e5129c2c-718e-4ce6-b327-e74855967ab8").unwrap();
		let user_uuid2 = uuid::Uuid::parse_str("215248b5-8402-4211-93c0-9f71a93e69a9").unwrap();
		let bid1 = Bid {
			user_uuid: user_uuid1,
			item_uuid: item_uuid.clone(),
			timestamp: 1591915318,
			amount: 30f64,
		};

		let bid2 = Bid {
			user_uuid: user_uuid2,
			amount: 31f64,
			..bid1
		};

		// When no bids
		assert!(bm.get_bids(&item_uuid).is_ok());
		assert!(bm.get_bids(&item_uuid).unwrap().len() == 0);

		// When wrong item_uuid requested
		let non_existent_uuid = uuid::Uuid::parse_str("a17b364f-0a5d-4e6e-91dd-0fa38c11b4ea").unwrap();
		assert!(bm.get_bids(&non_existent_uuid).is_err());

		// When after insertion bids requested
		bm.insert_bid(&bid1).expect("Failed to insert first bid");
		bm.insert_bid(&bid2).expect("Failed to insert second bid");

		assert!(bm.get_bids(&item_uuid).unwrap().len() == 2);
	}

	#[test]
	fn test_get_bids_by_user() {
		let item_uuid1 = uuid::Uuid::parse_str("1cb396fd-3242-40ce-aaa1-8e8337c70435").unwrap();
		let item_uuid2 = uuid::Uuid::parse_str("e5129c2c-718e-4ce6-b327-e74855967ab8").unwrap();
		let mut bm = BidManagement::new(vec![item_uuid1.clone(), item_uuid2.clone()]);

		let user_uuid = uuid::Uuid::parse_str("215248b5-8402-4211-93c0-9f71a93e69a9").unwrap();
		let bid1 = Bid {
			user_uuid,
			item_uuid: item_uuid1.clone(),
			timestamp: 1591915318,
			amount: 30f64,
		};

		let bid2 = Bid {
			item_uuid: item_uuid2,
			amount: 31f64,
			..bid1
		};

		// When no bids, hence no user exist
		assert!(bm.get_bids_by_user(&user_uuid).is_err());

		// When wrong  non_existent user_uuid requested
		let non_existent_uuid = uuid::Uuid::parse_str("a17b364f-0a5d-4e6e-91dd-0fa38c11b4ea").unwrap();
		assert!(bm.get_bids_by_user(&non_existent_uuid).is_err());

		// When after insertion bids requested
		bm.insert_bid(&bid1).expect("Failed to insert first bid");
		bm.insert_bid(&bid2).expect("Failed to insert second bid");

		assert!(
			bm.get_bids_by_user(&user_uuid)
				.expect("Failed in getting bids by the user")
				.len() == 2
		);
	}
}
