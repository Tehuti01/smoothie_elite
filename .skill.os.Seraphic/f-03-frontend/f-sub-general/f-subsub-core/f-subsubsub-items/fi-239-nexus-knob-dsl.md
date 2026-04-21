---
id: fi-239-nexus-knob-dsl.md
category: f-03-frontend
---

# 🛠️ NEXUS KNOB DSL (EXAMPLE)

A 12x Quality implementation of a signature knob using the Smoothie UI framework.

### 1. The Declarative Node
```rust
use smoothie_ui::prelude::*;

pub fn build_nexus_knob(id: &str, label: &str) -> Node {
    Knob::new(id)
        .label(label)
        // [Strophe 29]: Assigning Real-Life Material
        .material(Material::BrushedMetal {
            anisotropy: 0.8,
            roughness: 0.2,
        })
        .radius(40.0)
        // [Strophe 23]: PHI-resonant depth
        .depth(PHI * 2.0) 
        .shadow(Shadow::Soft {
            diffusion: 1.618,
            offset: [2.0, 2.0],
        })
        .interaction(Interaction::Radial)
        .build()
}
```

### 2. Parameter Integration (Wait-Free)
```rust
// Wires the front-end knob to the Tier 1 DSP parameter
nexus_knob.bind(params.get_ref("cutoff"));
```

---
*Example 12x UI Implementation: CONFIRMED.*
