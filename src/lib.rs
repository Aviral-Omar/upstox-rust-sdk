//! A Rust client for communicating with the [Upstox API](<https://upstox.com/uplink/>).

//! Upstox API is a set of rest APIs that provide data required to build a complete investment and trading platform. Execute orders in real time, manage user portfolio, stream live market data (using Websockets), and a lot more with this crate.

//! Refer to [`client`] for usage guides.
mod apis;
pub mod client;
pub mod constants;
pub mod models;
pub mod protos;
mod utils;
pub mod ws_client;
