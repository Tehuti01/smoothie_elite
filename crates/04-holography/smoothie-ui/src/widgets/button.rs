/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x3ea29107 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/04-holography/smoothie-ui/src/widgets/button.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;

use core::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone, Copy, PartialEq)]
/// Technical implementation of the ButtonStyle enumeration.
pub enum ButtonStyle {
    Toggle,
    Momentary,
    Radio,
}

/// Technical implementation of the ButtonState structure.
pub struct ButtonState {
    pressed: AtomicBool,
    toggled: AtomicBool,
}

impl ButtonState {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            pressed: AtomicBool::new(false),
            toggled: AtomicBool::new(false),
        }
    }

    #[inline]
    /// Technical implementation of the is_pressed logic.
    pub fn is_pressed(&self) -> bool {
        self.pressed.load(Ordering::Relaxed)
    }

    #[inline]
    /// Technical implementation of the is_toggled logic.
    pub fn is_toggled(&self) -> bool {
        self.toggled.load(Ordering::Relaxed)
    }

    /// Technical implementation of the press logic.
    pub fn press(&self) {
        self.pressed.store(true, Ordering::Relaxed);
    }

    /// Technical implementation of the release logic.
    pub fn release(&self) {
        self.pressed.store(false, Ordering::Relaxed);
    }

    /// Technical implementation of the toggle logic.
    pub fn toggle(&self) {
        let current = self.toggled.load(Ordering::Relaxed);
        self.toggled.store(!current, Ordering::Relaxed);
    }

    /// Technical implementation of the set_toggled logic.
    pub fn set_toggled(&self, value: bool) {
        self.toggled.store(value, Ordering::Relaxed);
    }
}

impl Default for ButtonState {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Technical implementation of the Button structure.
pub struct Button {
    state: ButtonState,
    style: ButtonStyle,
    label: &'static str,
    group_id: u32,
}

impl Button {
    /// Initializes a new instance of the associated type.
    pub const fn new(label: &'static str) -> Self {
        Self {
            state: ButtonState::new(),
            style: ButtonStyle::Momentary,
            label,
            group_id: 0,
        }
    }

    /// Technical implementation of the toggle logic.
    pub const fn toggle(label: &'static str) -> Self {
        Self {
            state: ButtonState::new(),
            style: ButtonStyle::Toggle,
            label,
            group_id: 0,
        }
    }

    /// Technical implementation of the radio logic.
    pub const fn radio(label: &'static str, group_id: u32) -> Self {
        Self {
            state: ButtonState::new(),
            style: ButtonStyle::Radio,
            label,
            group_id,
        }
    }

    /// Technical implementation of the on_mouse_down logic.
    pub fn on_mouse_down(&self) {
        match self.style {
            ButtonStyle::Momentary => self.state.press(),
            ButtonStyle::Toggle => self.state.toggle(),
            ButtonStyle::Radio => {
                self.state.set_toggled(true);
            }
        }
    }

    /// Technical implementation of the on_mouse_up logic.
    pub fn on_mouse_up(&self) {
        match self.style {
            ButtonStyle::Momentary => self.state.release(),
            ButtonStyle::Toggle | ButtonStyle::Radio => {}
        }
    }

    /// Technical implementation of the is_active logic.
    pub fn is_active(&self) -> bool {
        match self.style {
            ButtonStyle::Momentary => self.state.is_pressed(),
            ButtonStyle::Toggle | ButtonStyle::Radio => self.state.is_toggled(),
        }
    }

    /// Technical implementation of the label logic.
    pub fn label(&self) -> &'static str {
        self.label
    }

    /// Technical implementation of the group_id logic.
    pub fn group_id(&self) -> u32 {
        self.group_id
    }
}

impl Default for Button {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new("")
    }
}

pub const MAX_BUTTONS: usize = 32;

/// Technical implementation of the ButtonGroup structure.
pub struct ButtonGroup {
    buttons: [*const Button; MAX_BUTTONS],
    active_index: usize,
    count: usize,
}

impl ButtonGroup {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            buttons: [core::ptr::null(); MAX_BUTTONS],
            active_index: 0,
            count: 0,
        }
    }

    /// Performs vector addition logic.
    pub fn add(&mut self, button: *const Button) {
        if self.count < MAX_BUTTONS {
            self.buttons[self.count] = button;
            self.count += 1;
        }
    }

    /// Technical implementation of the activate logic.
    pub fn activate(&mut self, index: usize) {
        if index < self.count {
            for i in 0..self.count {
                unsafe {
                    if !self.buttons[i].is_null() {
                        (*self.buttons[i]).state.set_toggled(i == index);
                    }
                }
            }
            self.active_index = index;
        }
    }

    /// Technical implementation of the active logic.
    pub fn active(&self) -> usize {
        self.active_index
    }
}

impl Default for ButtonGroup {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_toggle_button logic.
    fn test_toggle_button() {
        let btn = Button::toggle("Test");
        assert!(!btn.is_active());
    }

    #[test]
    /// Technical implementation of the test_momentary_button logic.
    fn test_momentary_button() {
        let btn = Button::new("Test");
        btn.on_mouse_down();
        assert!(btn.is_active());
        btn.on_mouse_up();
        assert!(!btn.is_active());
    }
}
