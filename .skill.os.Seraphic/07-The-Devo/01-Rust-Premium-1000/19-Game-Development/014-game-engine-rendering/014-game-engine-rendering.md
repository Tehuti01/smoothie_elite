# SKILL 014: GAME ENGINE & RENDERING

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        GAME ENGINE & RENDERING IN RUST
                     Graphics & Game Development
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Comprehensive mastery of game engine architecture and rendering in Rust
including ECS, physics integration, shader development, and GPU pipelines.

## TABLE OF CONTENTS

1. [ECS Architecture](#ecs-architecture)
2. [Game Loop](#game-loop)
3. [Rendering Pipeline](#rendering-pipeline)
4. [Shaders](#shaders)
5. [Physics Integration](#physics-integration)

---

## ECS ARCHITECTURE

### 1.1 Component-Entity-System

```rust
pub struct World {
    entities: EntityMap,
    components: ComponentStore,
    systems: Vec<Box<dyn System>>,
}

pub struct Entity(u64);

pub trait Component: Send + Sync {
    fn component_id() -> ComponentId;
}

pub trait System: Send + Sync {
    fn update(&mut self, world: &mut World, dt: f64);
}

// Example component
#[derive(Clone)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quaternion,
    pub scale: Vec3,
}

impl Component for Transform {
    fn component_id() -> ComponentId {
        ComponentId::of::<Transform>()
    }
}

// Example system
pub struct MovementSystem;

impl System for MovementSystem {
    fn update(&mut self, world: &mut World, dt: f64) {
        for (entity, transform) in world.query::<&mut Transform>() {
            if let Some(velocity) = world.get_component::<Velocity>(entity) {
                transform.position += velocity.0 * dt;
            }
        }
    }
}
```

---

## GAME LOOP

### 2.1 Main Loop

```rust
pub struct Game {
    world: World,
    renderer: Renderer,
    input: InputManager,
}

impl Game {
    pub fn run(&mut self) {
        let mut last_time = std::time::Instant::now();
        
        loop {
            let current_time = std::time::Instant::now();
            let dt = (current_time - last_time).as_secs_f64();
            last_time = current_time;
            
            // Process input
            self.input.process_events();
            if self.input.quit_requested() {
                break;
            }
            
            // Update
            self.world.update(dt);
            
            // Render
            self.renderer.render(&self.world);
            
            // Wait for vsync
            self.renderer.wait_vsync();
        }
    }
}
```

---

## RENDERING PIPELINE

### 3.1 GPU Pipeline

```rust
pub struct Renderer {
    device: GpuDevice,
    swap_chain: SwapChain,
    render_pass: RenderPass,
    pipeline: GraphicsPipeline,
}

impl Renderer {
    pub fn render(&mut self, world: &World) {
        // Begin frame
        let frame = self.swap_chain.acquire_frame();
        
        // Update uniforms
        let camera = world.query::<&Camera>().next().unwrap();
        self.update_camera_buffers(camera);
        
        // Record commands
        let mut encoder = self.device.create_command_encoder();
        
        // Render shadow maps
        self.render_shadows(&mut encoder, world);
        
        // Main render pass
        encoder.begin_render_pass(&self.render_pass);
        
        for (entity, mesh) in world.query::<&Renderable>() {
            let transform = world.get_component::<Transform>(entity).unwrap();
            self.set_transform(&mut encoder, transform);
            self.draw_mesh(&mut encoder, &mesh);
        }
        
        encoder.end_render_pass();
        
        // Submit
        self.device.submit(encoder.finish());
        self.swap_chain.present(frame);
    }
}
```

---

## SHADERS

### 4.1 Vertex Shader

```glsl
// vertex shader
#version 450

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec2 a_uv;

layout(set = 0, binding = 0) uniform Uniforms {
    mat4 model;
    mat4 view;
    mat4 projection;
};

layout(location = 0) out vec3 v_position;
layout(location = 1) out vec3 v_normal;
layout(location = 2) out vec2 v_uv;

void main() {
    v_position = (model * vec4(a_position, 1.0)).xyz;
    v_normal = mat3(model) * a_normal;
    v_uv = a_uv;
    gl_Position = projection * view * model * vec4(a_position, 1.0);
}
```

---

## RECAP

1. **ECS scales well** - Components over inheritance
2. **Frame pacing fixed** - Consistent dt for simulation
3. **Batching essential** - Draw call optimization
4. **LOD for distant objects** - Level of detail
5. **Shader cache** - Avoid recompilation

---

*Skill ID: 014 | Category: Game-Development | Complexity: Expert*
*Version: 1.0.0 | Last Updated: 2024*