//! Warden is a configurable service that helps streamline your continuous
//! integration and deployment workflows by listening to webhook events.
//!
//! Warden's built-in runtime automatically loads your executable scripts and
//! is capable of running them sequentially or in parallel.
pub mod config;
pub mod environment;
pub mod state;

pub type BoxedError = Box<dyn std::error::Error + Send + Sync + 'static>;
