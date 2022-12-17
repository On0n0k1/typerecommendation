use std::sync::Weak;

use parking_lot::RwLock;

use crate::{entry::Entry, node::Node};

use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reply::Response, Rejection, Reply};

pub enum VoteResult {
    // Contains the next node it should travel to
    Next(Weak<RwLock<Node>>),
    // Represent that the operation is finished and the correct node was found
    // Return the found entry and current node reference to start updating top backwards.
    // The reference is an option for ease of testing.
    Success(Entry, Option<Weak<RwLock<Node>>>),
    // Represent that the operation is finished and the correct node was not found
    NotFound,
}

/// This will be deserialized into a response for the user.
/// 
/// Output::BadRequest will be an empty body.
/// Output::Created(entry) body will be entry deserialized as JSON.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
enum Output {
    BadRequest,
    Created(Entry),
}

impl Into<Result<Response, Rejection>> for VoteResult {
    fn into(self) -> Result<Response, Rejection> {
        let output: Output = match self {
            VoteResult::Next(_lock) => {
                panic!("Unexpected VoteResult. Got Next.");
            }
            VoteResult::NotFound => Output::BadRequest,
            VoteResult::Success(entry, lock) => match lock {
                Some(_) => panic!("Unexpected VoteResult. Got Success with a lock."),
                None => Output::Created(entry),
            },
        };

        let mut response = warp::reply::json(&output).into_response();
        let status: &mut StatusCode = response.status_mut();

        match output {
            Output::BadRequest => *status = StatusCode::BAD_REQUEST,
            Output::Created(_) => *status = StatusCode::CREATED,
        };

        Ok(response)
    }
}
