use std::net::SocketAddr;
use warp::Filter;

use crate::{
    endpoints::{get_entries, get_top_entries, vote, vote_json},
    tree::Tree,
};

/// Set all endpoints and start the server.
///
/// Will keep running until the system shuts down.
pub async fn start(socket_addr: SocketAddr, tree: Tree) {
    let tree_copy = tree.clone();

    let tree_filter = warp::any().map(move || tree_copy.clone());

    println!("\n");
    println!("Endpoint GET {}/rec/[prefix]", socket_addr);
    let get_names = warp::get()
        .and(warp::path!("rec" / String))
        .and(tree_filter.clone())
        .and_then(get_entries);

    println!("Endpoint GET {}/rec", socket_addr);
    let get_names = get_names.or(warp::get()
        .and(warp::path!("rec"))
        .and(tree_filter.clone())
        .and_then(get_top_entries));

    println!("Endpoint GET {}/rec/", socket_addr);
    let get_names = get_names.or(warp::get()
        .and(warp::path!("rec" / ..))
        .and(tree_filter.clone())
        .and_then(get_top_entries));

    println!("Endpoint POST Input:JSON {}/rec/", socket_addr);
    let rec_vote = warp::post()
        .and(warp::path("rec"))
        .and(warp::path::end())
        .and(vote_json())
        .and(tree_filter)
        .and_then(vote);

    let routes = get_names.or(rec_vote);

    println!("\nStarting server...");

    // A signal that happens when the user press ctrl+c
    let signal = async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen to shutdown signal.")
    };

    let (_addr, server) = warp::serve(routes).bind_with_graceful_shutdown(socket_addr, signal);

    server.await;

    println!("Shutting down");
}
