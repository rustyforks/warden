//! Warden is a small service that acts as a utility/assistant for CI/CD
//! workflows by listening to and responding to Webhooks. Warden loads
//! scripts for you and executes them as tasks either sequentially or in
//! parallel.
pub mod config;