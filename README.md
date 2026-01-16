---
license: mit
tags:
  - spatial-database
  - memory
  - embeddings
  - ai
  - vector-search
  - rust
library_name: arms-core
pipeline_tag: feature-extraction
---

# ARMS - Attention Reasoning Memory Store
[![DOI](https://zenodo.org/badge/1132385062.svg)](https://doi.org/10.5281/zenodo.18263613) [![crates.io](https://img.shields.io/crates/v/arms-core.svg)](https://crates.io/crates/arms-core) [![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE) [![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
> **Position IS Relationship** - A Spatial Memory Fabric for AI Systems

ARMS is a spatial memory fabric that enables AI systems to store and retrieve computed states by their native dimensional coordinates. Unlike traditional databases that require explicit relationships through foreign keys or learned topology through approximate nearest neighbor algorithms, ARMS operates on a fundamental principle: **proximity defines connection**.

![ARMS Architecture](paper/figures/fig01_architecture.jpg)

## The Problem

Current AI memory approaches all lose information:

- **Extended context**: Expensive, doesn't scale beyond training length
- **RAG retrieval**: Retrieves text, requires recomputation of attention
- **Vector databases**: Treat all data as unstructured point clouds
- **External memory**: Key-value stores with explicit indexing

![Traditional vs ARMS](paper/figures/fig06_traditional_vs_arms.jpg)

## The ARMS Insight

```
Traditional:  State → Project → Index → Retrieve → Reconstruct
              (lossy at each step)

ARMS:         State → Store AT coordinates → Retrieve → Inject directly
              (native representation preserved)
```

## The Five Primitives

Everything in ARMS reduces to five operations:

![Five Primitives](paper/figures/fig03_primitives.jpg)

| Primitive | Type | Purpose |
|-----------|------|---------|
| **Point** | `Vec<f32>` | Any dimensionality |
| **Proximity** | `fn(a, b) -> f32` | How related? |
| **Merge** | `fn(points) -> point` | Compose together |
| **Place** | `fn(point, data) -> id` | Exist in space |
| **Near** | `fn(point, k) -> ids` | What's related? |

## Quick Start

```rust
use arms_core::{Arms, ArmsConfig, Point};

// Create ARMS with default config
let mut arms = Arms::new(ArmsConfig::new(768));

// Place a point in the space
let point = Point::new(vec![0.1; 768]);
let id = arms.place(point, b"my data".to_vec()).unwrap();

// Find nearby points
let query = Point::new(vec![0.1; 768]);
let neighbors = arms.near(&query, 5).unwrap();
```

## Hexagonal Architecture

ARMS follows a hexagonal (ports-and-adapters) architecture. The core domain contains pure math with no I/O. Ports define trait contracts. Adapters provide swappable implementations.

![Hexagonal Architecture](paper/figures/fig02_hexagonal.jpg)

```
┌─────────────────────────────────────────────────────────────┐
│                         ARMS                                │
├─────────────────────────────────────────────────────────────┤
│  CORE (pure math, no I/O)                                   │
│    Point, Id, Blob, Proximity, Merge                        │
│                                                             │
│  PORTS (trait contracts)                                    │
│    Place, Near, Latency                                     │
│                                                             │
│  ADAPTERS (swappable implementations)                       │
│    Storage: Memory, NVMe (planned)                          │
│    Index: Flat, HAT (see arms-hat crate)                    │
│                                                             │
│  ENGINE (orchestration)                                     │
│    Arms - the main entry point                              │
└─────────────────────────────────────────────────────────────┘
```

## The Hippocampus Analogy

ARMS functions as an artificial hippocampus for AI systems:

![Hippocampus Analogy](paper/figures/fig05_hippocampus.jpg)

| Hippocampus | ARMS |
|-------------|------|
| Encodes episodic memories | Stores attention states |
| Spatial navigation | High-dimensional proximity |
| Pattern completion | Near queries |
| Memory consolidation | Merge operations |
| Place cells | Points at coordinates |

## Ecosystem

![ARMS Ecosystem](paper/figures/fig07_ecosystem.jpg)

### Related Crates

- [`arms-hat`](https://crates.io/crates/arms-hat) - Hierarchical Attention Tree index adapter (100% recall, 70x faster than HNSW)

### Planned Adapters

- `arms-nvme` - Persistent storage via memory-mapped files
- `arms-distributed` - Sharded storage across machines
- `arms-gpu` - CUDA-accelerated similarity search
- `arms-py` - Python bindings

## Proximity Functions

Built-in proximity measures:

- **Cosine** - Angle between vectors (semantic similarity)
- **Euclidean** - Straight-line distance
- **DotProduct** - Raw dot product
- **Manhattan** - L1 distance

## Installation

```toml
[dependencies]
arms-core = "0.1"
```

## Paper

The research paper is available in the [`paper/`](paper/) directory.

**ARMS: A Spatial Memory Fabric for AI Systems**
Andrew Young, 2026

## License

MIT License - see [LICENSE](LICENSE)

## Citation

If you use ARMS in research, please cite:

```bibtex
@article{young2026arms,
  author = {Young, Andrew},
  title = {ARMS: A Spatial Memory Fabric for AI Systems},
  journal = {arXiv preprint},
  year = {2026},
  url = {https://github.com/automate-capture/arms}
}
```

## Author

Andrew Young - [andrew@automate-capture.com](mailto:andrew@automate-capture.com)
