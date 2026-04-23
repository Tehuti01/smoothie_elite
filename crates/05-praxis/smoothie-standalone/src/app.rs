/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x55f5121e | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-standalone/src/app.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

/// Technical implementation of the AutonomousApp structure.
pub struct AutonomousApp {
    title: &'static str,
    width: u32,
    height: u32,
    window: Option<Window>,
}

impl AutonomousApp {
    /// Initializes a new instance of the associated type.
    pub fn new(title: &'static str, width: u32, height: u32) -> Self {
        Self {
            title,
            width,
            height,
            window: None,
        }
    }

    /// Technical implementation of the run logic.
    pub fn run(mut self) -> anyhow::Result<()> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self)?;
        Ok(())
    }
}

impl ApplicationHandler for AutonomousApp {
    /// Technical implementation of the resumed logic.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title(self.title)
                .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height));

            let window = event_loop
                .create_window(window_attributes)
                .expect("Failed to create window");
            self.window = Some(window);
        }
    }

    /// Technical implementation of the window_event logic.
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}
