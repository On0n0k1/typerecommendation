use serde::{Deserialize, Serialize};
use warp::Filter;

use crate::{
    log::log_debug,
    procedures::vote::{tree::Vote, VoteResult},
    tree::Tree,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Input {
    pub name: String,
}

/// Configure the path to require a json body, and deny a large body.
pub fn vote_json() -> impl Filter<Extract = (Input,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn vote(request: Input, tree: Tree) -> Result<impl warp::Reply, warp::Rejection> {
    log_debug("---------------------------------------------------------------------");
    println!("Post Vote Name {}", &request.name);

    let response: VoteResult = tree.vote(&request.name);

    response.into()
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Weak};

    use parking_lot::RwLock;
    use warp::http::StatusCode;

    use crate::{entry::Entry, node::Node, procedures::vote::VoteResult};

    fn new_empty_next() -> (Arc<RwLock<Node>>, VoteResult) {
        let node: Arc<RwLock<Node>> = Node::new(None, "".into(), 0, 0);
        let lock: Weak<RwLock<Node>> = Arc::downgrade(&node);
        let next: VoteResult = VoteResult::Next(lock);

        (node, next)
    }

    fn new_success() -> VoteResult {
        let entry: Entry = Entry::new("AName".into(), 42);
        let success: VoteResult = VoteResult::Success(entry);

        success
    }

    mod request_conversion {
        use super::*;

        #[test]
        #[should_panic(expected = "Unexpected VoteResult. Got Next.")]
        fn post_result_next_error() {
            let (_node, next) = new_empty_next();

            // This is expected to panic because VoteResult::Next is not meant to be used by the endpoints
            let _warp_response: Result<warp::reply::Response, warp::Rejection> = next.into();
        }

        #[test]
        fn post_result_not_found() {
            let not_found: VoteResult = VoteResult::NotFound;

            let warp_response: Result<warp::reply::Response, warp::Rejection> = not_found.into();
            let warp_response = match warp_response {
                Err(err) => panic!("Warp Response is Err {:#?} .", err),
                Ok(value) => value,
            };

            // Status code 400
            let expected_status_code = StatusCode::BAD_REQUEST;
            assert_eq!(warp_response.status(), expected_status_code);
        }

        #[test]
        fn post_result_success() {
            let entry: Entry = Entry::new("AName".into(), 32);
            let success: VoteResult = VoteResult::Success(entry);

            let warp_response: Result<warp::reply::Response, warp::Rejection> = success.into();
            let warp_response = match warp_response {
                Err(err) => panic!("Warp Response is Err {:#?} .", err),
                Ok(value) => value,
            };

            // Status code 201
            let expected_status_code = StatusCode::CREATED;
            assert_eq!(warp_response.status(), expected_status_code);
        }
    }

    mod status_codes {
        use super::*;
        use warp::reply::Response;
        use warp::Rejection;

        #[test]
        fn not_found_is_400() {
            let not_found: VoteResult = VoteResult::NotFound;
            let response: Result<Response, Rejection> = not_found.into();

            let response: Response = response.unwrap();

            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        }

        #[test]
        fn success_is_201() {
            let success = new_success();
            let response: Result<Response, Rejection> = success.into();

            let response: Response = response.unwrap();

            assert_eq!(response.status(), StatusCode::CREATED);
        }
    }
}
