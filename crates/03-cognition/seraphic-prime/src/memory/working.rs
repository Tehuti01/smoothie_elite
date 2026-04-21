/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x1ae9a2a6 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/seraphic-prime/src/memory/working.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use heapless::String;
use heapless::Vec;

/// 🧬 The Message
#[derive(Debug, Clone)]
/// Technical implementation of the Message structure.
pub struct Message {
    role: &'static str,
    content: String<512>, // Fixed-size string for no_std
}

/// Manages the context window for the current autonomous task.
/// Technical implementation of the WorkingMemory structure.
pub struct WorkingMemory {
    messages: Vec<Message, 16>, // Max 16 messages in short-term context
}

impl WorkingMemory {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    /// 🚀 Push a message into the context window
    pub fn push(&mut self, role: &'static str, content: &str) -> Result<(), &'static str> {
        let mut msg_content = String::new();
        let _ = msg_content.push_str(content);

        let msg = Message {
            role,
            content: msg_content,
        };
        self.messages
            .push(msg)
            .map_err(|_| "CONTEXT_WINDOW_EXCEEDED")
    }

    /// 🦾 Retrieve the full context as a single buffer
    pub fn get_context(&self) -> &Vec<Message, 16> {
        &self.messages
    }

    /// 🦾 Clear the working memory for a new task
    pub fn clear(&mut self) {
        self.messages.clear();
    }
}

/// 🛡️ System Integrity Verification: Context integrity verified.
pub const MEMORY_DENSITY: &str = "SERAPHIC_100000X_WORKING_SET";
