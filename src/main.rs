mod endpoints;
mod entry;
mod env;
mod log;
mod node;
mod procedures;
mod server;
mod tree;

use std::net::{SocketAddr, ToSocketAddrs};

use crate::{
    env::{EnvVars, SuggestionNumber},
    tree::Tree,
};

#[tokio::main]
async fn main() {
    let env_vars = match EnvVars::new() {
        Ok(value) => value,
        Err(err) => panic!("Environment Variables Error: {}", err),
    };

    let host: String = env_vars.host;
    let port: String = env_vars.port;
    let suggestions: SuggestionNumber = env_vars.suggestion_number;

    let tree = match Tree::new(suggestions).await {
        Ok(value) => value,
        Err(err) => panic!("Error Loading Tree: {}", err),
    };

    let host_port = format!("{host}:{port}");
    let default_socket: SocketAddr = format!("0.0.0.0:{port}")
        .parse()
        .expect("Failed to parse the default socket");

    // If it fails to parse the socket Address, will use unindentified address 0.0.0.0
    let socket_addr: SocketAddr = match host_port.parse() {
        Ok(value) => value,
        Err(err) => {
            println!("Failed to parse {host}:{port} as IPV4 or IPV6 address. err: {err} .");
            println!("Warp and Rust standard libraries currently have no method of recognizing URI addresses.");
            println!("Therefore utilizing an unindentified address instead 0.0.0.0 .");

            match host_port.to_socket_addrs() {
                Err(_) => default_socket,
                Ok(mut iterator) => match iterator.next() {
                    None => default_socket,
                    Some(value) => value,
                },
            }
        }
    };

    server::start(socket_addr, tree).await;
}
