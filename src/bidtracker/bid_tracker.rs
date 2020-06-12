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

use super::Bid;
use crate::errors::BidTrackerError;
use anyhow::Result;

pub trait BidTracker {
	fn new(allowed_bids: Vec<uuid::Uuid>) -> Self;
	fn insert_bid(&mut self, bid: &Bid) -> Result<(), BidTrackerError>;
	fn current_winning_bid(&self, item_uuid: &uuid::Uuid) -> Result<Bid, BidTrackerError>;
	fn get_bids(&self, item_uuid: &uuid::Uuid) -> Result<Vec<Bid>, BidTrackerError>;
	fn get_bids_by_user(&self, user_uuid: &uuid::Uuid) -> Result<Vec<Bid>, BidTrackerError>;
}
