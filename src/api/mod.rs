mod handler;
mod response;

pub use response::{send_json, ResponseMessage, ResponseMessageBid, ResponseMessageBids};

pub mod routes;
pub use handler::{get_bids, get_current_winning_bid, get_user_bids, post_bid_new};
pub mod custom_error_handler;
