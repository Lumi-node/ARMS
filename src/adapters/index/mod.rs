//! # Index Adapters
//!
//! Implementations of the Near port for different index backends.
//!
//! Available adapters:
//! - `FlatIndex` - Brute force search (exact, slow for large N)
//! - `HnswIndex` - Hierarchical Navigable Small World (approximate, fast) [TODO]

mod flat;

pub use flat::FlatIndex;

// TODO: Add HNSW adapter
// mod hnsw;
// pub use hnsw::HnswIndex;
