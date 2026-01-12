//! Templates module for outreach functionality
//!
//! This module provides a comprehensive template system for client communications,
//! including prebuilt templates, variable replacement, and UI components.

pub mod components;
pub mod engine;
pub mod prebuilt;

pub use components::*;
pub use engine::*;
pub use prebuilt::*;