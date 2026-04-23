/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xab03cc71 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/logging/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate smoothie_core;
extern crate std;
///
///
/// the audio thread is never blocked by logging operations. Messages
/// dropped — this is intentional for real-time safety.
/// ## Usage
/// ```rust
/// use smoothie_logging::{warn, debug};
///
/// warn("Buffer underrun detected");
/// debug("Processing block of 512 samples");
/// ```
use smoothie_core::constants::F_233;
use smoothie_sync::SmoothieMutex;

/// Log severity levels, ordered from most verbose to most critical.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Technical implementation of the LogLevel enumeration.
pub enum LogLevel {
    /// Extremely detailed diagnostic information.
    Trace = 0,
    /// Diagnostic information useful during development.
    Debug = 1,
    /// General operational information.
    Info = 2,
    /// Potentially problematic situations.
    Warn = 3,
    /// Recoverable errors.
    Error = 4,
    /// Unrecoverable errors (system should shut down gracefully).
    Fatal = 5,
}

/// A single log entry stored in the ring buffer.
#[derive(Clone)]
/// Technical implementation of the LogEntry structure.
pub struct LogEntry {
    /// Severity level of the message.
    pub level: LogLevel,
    /// Static message string.
    pub message: &'static str,
    /// Timestamp (sample count or system ticks).
    pub timestamp: u64,
}

///
/// efficient modular arithmetic properties.
struct LogBuffer {
    entries: [Option<LogEntry>; F_233],
    head: usize,
    tail: usize,
    count: usize,
}

///
/// If the lock cannot be acquired, the message is silently dropped.
/// Technical implementation of the SmoothieLogger structure.
pub struct SmoothieLogger {
    buffer: SmoothieMutex<LogBuffer>,
    min_level: SmoothieMutex<LogLevel>,
}

impl SmoothieLogger {
    /// Create a new logger with default settings.
    pub const fn new() -> Self {
        Self {
            buffer: SmoothieMutex::new(LogBuffer {
                entries: [const { None }; F_233],
                head: 0,
                tail: 0,
                count: 0,
            }),
            min_level: SmoothieMutex::new(LogLevel::Info),
        }
    }

    /// Set the minimum log level.
    ///
    /// Messages below this level will be discarded.
    pub fn set_level(&self, level: LogLevel) {
        if let Some(mut min) = self.min_level.try_lock() {
            *min = level;
        }
    }

    /// Get the current minimum log level.
    pub fn level(&self) -> LogLevel {
        self.min_level
            .try_lock()
            .map(|l| *l)
            .unwrap_or(LogLevel::Info)
    }

    /// Log a message at the given level.
    ///
    /// This function is non-blocking. If the lock cannot be acquired
    /// or the buffer is full, the message is silently dropped.
    pub fn log(&self, level: LogLevel, message: &'static str, timestamp: u64) {
        // Check minimum level (non-blocking)
        if let Some(min) = self.min_level.try_lock() {
            if level < *min {
                return;
            }
        }

        // Try to write to buffer (non-blocking)
        if let Some(mut buffer) = self.buffer.try_lock() {
            let current_head = buffer.head;
            let next_head = (current_head + 1) % F_233;

            // Drop oldest entry if buffer is full
            if next_head == buffer.tail {
                buffer.tail = (buffer.tail + 1) % F_233;
            } else {
                buffer.count += 1;
            }

            buffer.entries[current_head] = Some(LogEntry {
                level,
                message,
                timestamp,
            });
            buffer.head = next_head;
        }
    }

    /// Drain all pending log entries.
    ///
    /// Call this from the main thread to process accumulated log messages.
    /// Returns the number of entries drained.
    pub fn drain<F>(&self, mut callback: F) -> usize
    where
        F: FnMut(&LogEntry),
    {
        let mut drained = 0;
        if let Some(mut buffer) = self.buffer.try_lock() {
            while buffer.tail != buffer.head {
                let idx = buffer.tail;
                if let Some(entry) = &buffer.entries[idx] {
                    callback(entry);
                    drained += 1;
                }
                buffer.entries[idx] = None;
                buffer.tail = (idx + 1) % F_233;
            }
            buffer.count = 0;
        }
        drained
    }

    /// Get the number of pending log entries.
    pub fn pending_count(&self) -> usize {
        self.buffer.try_lock().map(|b| b.count).unwrap_or(0)
    }

    /// Clear all pending log entries.
    pub fn clear(&self) {
        if let Some(mut buffer) = self.buffer.try_lock() {
            buffer.head = 0;
            buffer.tail = 0;
            buffer.count = 0;
        }
    }
}

// ═══════════════════════════════════════════════════════════════
// Global Logger Instance
// ═══════════════════════════════════════════════════════════════

/// The global logger instance.
pub static GLOBAL_LOGGER: SmoothieLogger = SmoothieLogger::new();

/// Technical implementation of the trace logic.
pub fn trace(msg: &'static str) {
    GLOBAL_LOGGER.log(LogLevel::Trace, msg, 0);
}

/// Technical implementation of the debug logic.
pub fn debug(msg: &'static str) {
    GLOBAL_LOGGER.log(LogLevel::Debug, msg, 0);
}

/// Technical implementation of the info logic.
pub fn info(msg: &'static str) {
    GLOBAL_LOGGER.log(LogLevel::Info, msg, 0);
}

/// Technical implementation of the warn logic.
pub fn warn(msg: &'static str) {
    GLOBAL_LOGGER.log(LogLevel::Warn, msg, 0);
}

/// Technical implementation of the error logic.
pub fn error(msg: &'static str) {
    GLOBAL_LOGGER.log(LogLevel::Error, msg, 0);
}

/// Technical implementation of the fatal logic.
pub fn fatal(msg: &'static str) {
    GLOBAL_LOGGER.log(LogLevel::Fatal, msg, 0);
}

/// Technical implementation of the set_log_level logic.
pub fn set_log_level(level: LogLevel) {
    GLOBAL_LOGGER.set_level(level);
}

pub mod destination;
pub use destination::{FileDestination, LogDestination, StderrDestination};
