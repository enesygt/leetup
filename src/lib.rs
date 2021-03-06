#![allow(unused_variables, unused_imports, dead_code)]
pub use config::*;
pub use error::{LeetUpError, Result};

pub mod client;
pub mod cmd;
mod config;
mod error;
pub mod icon;
pub mod service;
pub mod template;
