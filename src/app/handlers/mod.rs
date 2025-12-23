//! Domain-specific action handlers.
//!
//! This module splits the large `update` function into smaller, domain-specific handlers.
//! Each handler returns `Option<Command>` - `None` if the action is not handled by that module.

pub mod brokers;
pub mod connection;
pub mod consumer_groups;
pub mod logs;
pub mod messages;
pub mod navigation;
pub mod system;
pub mod topics;
pub mod ui;
