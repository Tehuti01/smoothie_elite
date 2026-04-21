/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x3d17c014 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-midi/src/rpn.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::MidiMessage;
use smoothie_core::math::FloatExt;

/// RPN parameter number
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the RpnParam enumeration.
pub enum RpnParam {
    PitchBendSensitivity,
    ChannelFineTuning,
    ChannelCoarseTuning,
    ModulationDepth,
    Volume,
    Pan,
    Custom(u16),
}

/// 14-bit value wrapper
#[derive(Debug, Clone, Copy, Default)]
/// Technical implementation of the RpnValue structure.
pub struct RpnValue(pub u16);

impl RpnValue {
    /// Initializes a new instance of the associated type.
    pub fn new(msb: u8, lsb: u8) -> Self {
        Self(((msb as u16 & 0x7F) << 7) | (lsb as u16 & 0x7F))
    }

    /// Technical implementation of the from_u7 logic.
    pub fn from_u7(value: u8) -> Self {
        Self((value as u16) << 7)
    }

    /// Technical implementation of the msb logic.
    pub fn msb(self) -> u8 {
        (self.0 >> 7) as u8
    }
    /// Technical implementation of the lsb logic.
    pub fn lsb(self) -> u8 {
        (self.0 & 0x7F) as u8
    }
    /// Technical implementation of the as_u7 logic.
    pub fn as_u7(self) -> u8 {
        (self.0 >> 7) as u8
    }
    /// Technical implementation of the as_f32 logic.
    pub fn as_f32(self) -> f32 {
        self.0 as f32 / 16383.0
    }
}

/// Technical implementation of the RpnProcessor structure.
pub struct RpnProcessor {
    pending_param_msb: Option<u8>,
    pending_param_lsb: Option<u8>,
    pending_value_msb: Option<u8>,
    current_param: Option<u16>,
    current_value: RpnValue,
}

impl RpnProcessor {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            pending_param_msb: None,
            pending_param_lsb: None,
            pending_value_msb: None,
            current_param: None,
            current_value: RpnValue::default(),
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, msg: &MidiMessage) -> Option<RpnEvent> {
        match msg {
            MidiMessage::ControlChange {
                channel: _,
                controller,
                value,
            } => match *controller {
                cc::RPN_MSB => {
                    self.pending_param_msb = Some(*value);
                    self.check_param_complete();
                    None
                }
                cc::RPN_LSB => {
                    self.pending_param_lsb = Some(*value);
                    self.check_param_complete();
                    None
                }
                cc::DATA_ENTRY_MSB => {
                    self.pending_value_msb = Some(*value);
                    self.check_value_complete();
                    None
                }
                cc::DATA_ENTRY_LSB if self.pending_value_msb.is_some() => {
                    let msb = self.pending_value_msb.take().unwrap();
                    self.current_value = RpnValue::new(msb, *value);
                    self.emit_event()
                }
                cc::RPN_INCREMENT => {
                    self.current_value =
                        RpnValue(self.current_value.0.saturating_add(1).min(16383));
                    self.emit_event()
                }
                cc::RPN_DECREMENT => {
                    self.current_value = RpnValue(self.current_value.0.saturating_sub(1));
                    self.emit_event()
                }
                _ => None,
            },
            _ => None,
        }
    }

    /// Technical implementation of the check_param_complete logic.
    fn check_param_complete(&mut self) {
        if let (Some(msb), Some(lsb)) = (self.pending_param_msb, self.pending_param_lsb) {
            self.current_param = Some(((msb as u16 & 0x7F) << 7) | (lsb as u16 & 0x7F));
            self.pending_param_msb = None;
            self.pending_param_lsb = None;
        }
    }

    /// Technical implementation of the check_value_complete logic.
    fn check_value_complete(&mut self) {
        // Value is complete when we have MSB (LSB comes separately)
    }

    /// Technical implementation of the emit_event logic.
    fn emit_event(&mut self) -> Option<RpnEvent> {
        if let Some(param) = self.current_param {
            let event = RpnEvent {
                param,
                value: self.current_value,
            };
            Some(event)
        } else {
            None
        }
    }

    /// Technical implementation of the current_param logic.
    pub fn current_param(&self) -> Option<u16> {
        self.current_param
    }
    /// Technical implementation of the current_value logic.
    pub fn current_value(&self) -> RpnValue {
        self.current_value
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

/// RPN event with parameter number and value
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the RpnEvent structure.
pub struct RpnEvent {
    pub param: u16,
    pub value: RpnValue,
}

/// Technical implementation of the NrpnProcessor structure.
pub struct NrpnProcessor {
    pending_param_msb: Option<u8>,
    pending_param_lsb: Option<u8>,
    pending_value_msb: Option<u8>,
    current_param: Option<u16>,
    current_value: RpnValue,
}

impl NrpnProcessor {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            pending_param_msb: None,
            pending_param_lsb: None,
            pending_value_msb: None,
            current_param: None,
            current_value: RpnValue::default(),
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, msg: &MidiMessage) -> Option<NrpnEvent> {
        match msg {
            MidiMessage::ControlChange {
                channel: _,
                controller,
                value,
            } => match *controller {
                cc::NRPN_MSB => {
                    self.pending_param_msb = Some(*value);
                    self.check_param_complete();
                    None
                }
                cc::NRPN_LSB => {
                    self.pending_param_lsb = Some(*value);
                    self.check_param_complete();
                    None
                }
                cc::DATA_ENTRY_MSB => {
                    self.pending_value_msb = Some(*value);
                    None
                }
                cc::DATA_ENTRY_LSB if self.pending_value_msb.is_some() => {
                    let msb = self.pending_value_msb.take().unwrap();
                    self.current_value = RpnValue::new(msb, *value);
                    self.emit_event()
                }
                _ => None,
            },
            _ => None,
        }
    }

    /// Technical implementation of the check_param_complete logic.
    fn check_param_complete(&mut self) {
        if let (Some(msb), Some(lsb)) = (self.pending_param_msb, self.pending_param_lsb) {
            self.current_param = Some(((msb as u16 & 0x7F) << 7) | (lsb as u16 & 0x7F));
            self.pending_param_msb = None;
            self.pending_param_lsb = None;
        }
    }

    /// Technical implementation of the emit_event logic.
    fn emit_event(&mut self) -> Option<NrpnEvent> {
        if let Some(param) = self.current_param {
            let event = NrpnEvent {
                param,
                value: self.current_value,
            };
            Some(event)
        } else {
            None
        }
    }

    /// Technical implementation of the current_param logic.
    pub fn current_param(&self) -> Option<u16> {
        self.current_param
    }
    /// Technical implementation of the current_value logic.
    pub fn current_value(&self) -> RpnValue {
        self.current_value
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        *self = Self::new();
    }
}

/// NRPN event
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the NrpnEvent structure.
pub struct NrpnEvent {
    pub param: u16,
    pub value: RpnValue,
}

/// Technical implementation of the RpnNrpnProcessor structure.
pub struct RpnNrpnProcessor {
    rpn: RpnProcessor,
    nrpn: NrpnProcessor,
    is_nrpn: bool,
}

impl RpnNrpnProcessor {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            rpn: RpnProcessor::new(),
            nrpn: NrpnProcessor::new(),
            is_nrpn: false,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, msg: &MidiMessage) -> Option<ParameterEvent> {
        match msg {
            MidiMessage::ControlChange { controller, .. } => {
                if *controller == cc::NRPN_MSB || *controller == cc::NRPN_LSB {
                    self.is_nrpn = true;
                    self.nrpn.process(msg).map(ParameterEvent::Nrpn)
                } else if *controller == cc::RPN_MSB || *controller == cc::RPN_LSB {
                    self.is_nrpn = false;
                    self.rpn.process(msg).map(ParameterEvent::Rpn)
                } else if self.is_nrpn {
                    self.nrpn.process(msg).map(ParameterEvent::Nrpn)
                } else {
                    self.rpn.process(msg).map(ParameterEvent::Rpn)
                }
            }
            _ => None,
        }
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.rpn.reset();
        self.nrpn.reset();
        self.is_nrpn = false;
    }
}

/// Unified parameter event
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the ParameterEvent enumeration.
pub enum ParameterEvent {
    Rpn(RpnEvent),
    Nrpn(NrpnEvent),
}

pub mod cc {
    pub const RPN_MSB: u8 = 101;
    pub const RPN_LSB: u8 = 100;
    pub const NRPN_MSB: u8 = 99;
    pub const NRPN_LSB: u8 = 98;
    pub const DATA_ENTRY_MSB: u8 = 6;
    pub const DATA_ENTRY_LSB: u8 = 38;
    pub const RPN_INCREMENT: u8 = 96;
    pub const RPN_DECREMENT: u8 = 97;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_rpn_value logic.
    fn test_rpn_value() {
        let val = RpnValue::new(0x40, 0x00);
        assert_eq!(val.0, 0x2000);
    }

    #[test]
    /// Technical implementation of the test_rpn_processor logic.
    fn test_rpn_processor() {
        let mut proc = RpnProcessor::new();
        // Set param MSB=0, LSB=0 (pitch bend sensitivity)
        proc.process(&MidiMessage::ControlChange {
            channel: 0,
            controller: cc::RPN_MSB,
            value: 0,
        });
        proc.process(&MidiMessage::ControlChange {
            channel: 0,
            controller: cc::RPN_LSB,
            value: 0,
        });
        assert_eq!(proc.current_param(), Some(0));

        // Set value MSB=2, LSB=0 (24 cents = 2 semitones)
        proc.process(&MidiMessage::ControlChange {
            channel: 0,
            controller: cc::DATA_ENTRY_MSB,
            value: 2,
        });
        let event = proc.process(&MidiMessage::ControlChange {
            channel: 0,
            controller: cc::DATA_ENTRY_LSB,
            value: 0,
        });
        assert!(event.is_some());
    }

    #[test]
    /// Technical implementation of the test_rpn_increment logic.
    fn test_rpn_increment() {
        let mut proc = RpnProcessor::new();
        proc.current_value = RpnValue(100);
        proc.process(&MidiMessage::ControlChange {
            channel: 0,
            controller: cc::RPN_INCREMENT,
            value: 0,
        });
        assert_eq!(proc.current_value.0, 101);
    }
}
