# ARMS - Attention Reasoning Memory Store

> "The hippocampus of artificial minds"

ARMS is a spatial memory fabric for AI systems. It stores computed attention states at their native dimensional coordinates, enabling instant retrieval by proximity rather than traditional indexing.

## Core Philosophy

**Position IS relationship** - No foreign keys, proximity defines connection.

```
Traditional:  State → Project → Index → Retrieve → Reconstruct
              (lossy at each step)

ARMS:         State → Store AT coordinates → Retrieve → Inject directly
              (native representation preserved)
```

## The Five Primitives

Everything in ARMS reduces to five operations:

| Primitive | Type | Purpose |
|-----------|------|---------|
| **Point** | `Vec<f32>` | Any dimensionality |
| **Proximity** | `fn(a, b) -> f32` | How related? |
| **Merge** | `fn(points) -> point` | Compose together |
| **Place** | `fn(point, data) -> id` | Exist in space |
| **Near** | `fn(point, k) -> ids` | What's related? |

## Quick Start

```rust
use arms::{Arms, ArmsConfig, Point};

// Create ARMS with default config
let mut arms = Arms::new(ArmsConfig::new(768));

// Place a point in the space
let point = Point::new(vec![0.1; 768]);
let id = arms.place(point, b"my data".to_vec()).unwrap();

// Find nearby points
let query = Point::new(vec![0.1; 768]);
let neighbors = arms.near(&query, 5).unwrap();
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         ARMS                                 │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  CORE (pure math, no I/O)                                   │
│    Point, Id, Blob, Proximity, Merge                        │
│                                                              │
│  PORTS (trait contracts)                                     │
│    Place, Near, Latency                                     │
│                                                              │
│  ADAPTERS (swappable implementations)                       │
│    Storage: Memory, NVMe (coming)                           │
│    Index: Flat, HAT (see arms-hat crate)                    │
│                                                              │
│  ENGINE (orchestration)                                      │
│    Arms - the main entry point                              │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Proximity Functions

Built-in proximity measures:

- **Cosine** - Angle between vectors (semantic similarity)
- **Euclidean** - Straight-line distance
- **DotProduct** - Raw dot product
- **Manhattan** - L1 distance

## Related Crates

- [`arms-hat`](https://crates.io/crates/arms-hat) - Hierarchical Attention Tree index adapter for ARMS

## Installation

```toml
[dependencies]
arms = "0.1"
```

## License

MIT License - see [LICENSE](LICENSE)

## Citation

If you use ARMS in research, please cite:

```bibtex
@software{arms2026,
  author = {Young, Andrew},
  title = {ARMS: Attention Reasoning Memory Store},
  year = {2026},
  url = {https://github.com/automate-capture/arms}
}
```

## Author

Andrew Young - [andrew@automate-capture.com](mailto:andrew@automate-capture.com)
