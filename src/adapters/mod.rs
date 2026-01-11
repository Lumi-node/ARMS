//! # Adapters
//!
//! Swappable implementations of port traits.
//!
//! This is where the hexagonal architecture meets reality:
//! - Storage adapters: Memory, NVMe
//! - Index adapters: Flat (brute force)
//!
//! Each adapter implements one or more port traits.
//! Adapters can be swapped without changing core logic.
//!
//! For advanced index adapters like HAT (Hierarchical Attention Tree),
//! see the `arms-hat` crate.

pub mod storage;
pub mod index;
