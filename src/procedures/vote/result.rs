use std::sync::Weak;

use parking_lot::RwLock;

use crate::{entry::Entry, node::Node};

use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reply::Response, Rejection, Reply};

/// Result of a Vote Request.
///
/// - Next: Returned by a Node when the search is incomplete. Contains the next Node to access.
/// - Success: Entry Found.
/// - NotFound: Entry name was not found on Prefix Tree.
pub enum VoteResult {
    // Contains the next node it should travel to
    Next(Weak<RwLock<Node>>),
    Success(Entry),
    NotFound,
}

/// This will be deserialized into a response for the user.
///
/// Output::BadRequest will be an empty body.
/// Output::Created(entry) body will be entry deserialized as JSON.
#[derive(Deserialize, Serialize, Clone)]
#[serde(untagged)]
enum Output {
    BadRequest,
    Created(Entry),
}

// The macro below disables a lint from clippy
#[cfg_attr(feature = "cargo-clippy", allow(clippy::from_over_into))]
impl Into<Result<Response, Rejection>> for VoteResult {
    fn into(self) -> Result<Response, Rejection> {
        let output: Output = match self {
            VoteResult::Next(_lock) => {
                panic!("Unexpected VoteResult. Got Next.");
            }
            VoteResult::NotFound => Output::BadRequest,
            VoteResult::Success(entry) => Output::Created(entry),
        };

        let mut response = warp::reply::json(&output).into_response();
        let status: &mut StatusCode = response.status_mut();

        *status = match output {
            Output::BadRequest => StatusCode::BAD_REQUEST,
            Output::Created(_) => StatusCode::CREATED,
        };

        Ok(response)
    }
}
