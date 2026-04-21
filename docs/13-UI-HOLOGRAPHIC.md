# Holographic UI & Graphics

Smoothie Elite completely discards web-based UIs (React, HTML) for the plugin interface (Standalone apps still use Tauri/React). The in-DAW interface is powered by `smoothie-ui` (egui) and `smoothie-graphics` (wgpu).

## 1. Zero-DOM Architecture

The UI is strictly Immediate Mode (egui). There is no DOM, no HTML, and no garbage collection pauses. The UI is re-evaluated 60 to 144 times a second.

```rust
pub fn build_ui(cx: &mut UiContext, params: &Arc<PluginParams>) {
    cx.panel("Main", |ui| {
        ui.knob("Cutoff", &params.cutoff);
        ui.knob("Resonance", &params.resonance);
    });
}
```

## 2. GPU Rendering (wgpu)

- **Backends:** The UI compiles down to native graphics APIs: Metal (macOS), DirectX 12 (Windows), and Vulkan (Linux).
- **Performance:** CPU usage for the UI is near 0%. All rendering is offloaded to the GPU.

## 3. Holographic SDFs

Traditional plugins use PNG or SVG assets for knobs and dials. Smoothie Elite uses mathematical **Signed Distance Fields (SDFs)** calculated in the fragment shader.

- **Distributed Resolution:** A knob rendered at 10x scale will be perfectly crisp.
- **Dynamic Lighting:** Because SDFs describe geometry mathematically, the GPU can calculate real-time lighting, shadows, and reflections on the UI elements without 3D models.

## 4. The Seraphic Theme

The framework includes a built-in design system (`smoothie-graphics::theme::SeraphicTheme`).
- Defaults to a dark-mode, high-contrast aesthetic (Seraphic Orange on deep charcoal).
- Completely customizable via `cx.set_theme()`.
