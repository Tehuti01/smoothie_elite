/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xe6770e8c | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/core/src/error.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::fmt;

/// Core error type (replaces std::error::Error and anyhow::Error)
#[derive(Debug, Clone)]
/// Technical implementation of the SmoothieError enumeration.
pub enum SmoothieError {
    /// Buffer overflow or capacity exceeded
    BufferOverflow { capacity: usize, requested: usize },

    /// Invalid argument passed to function
    InvalidArgument { message: &'static str },

    /// Index out of bounds
    IndexOutOfBounds { index: usize, length: usize },

    /// Configuration error
    ConfigError { message: &'static str },

    /// Audio format mismatch
    FormatMismatch {
        expected: &'static str,
        got: &'static str,
    },

    /// Sample rate mismatch
    SampleRateMismatch { expected: f32, got: f32 },

    /// Channel count mismatch
    ChannelMismatch { expected: usize, got: usize },

    /// Not initialized
    NotInitialized { module: &'static str },

    /// Already initialized (double init)
    AlreadyInitialized { module: &'static str },

    /// Resource not available
    ResourceUnavailable { resource: &'static str },

    /// Operation timed out
    Timeout { operation: &'static str },

    /// Synchronization error
    SyncError { message: &'static str },

    /// MIDI error
    MidiError { message: &'static str },

    /// DSP algorithm error
    DSPError { message: &'static str },

    /// Serialization error
    SerializationError { message: &'static str },

    /// I/O error
    IOError { message: &'static str },

    /// Generic error with message
    Generic { message: &'static str },
}

impl SmoothieError {
    /// Create buffer overflow error
    pub fn buffer_overflow(capacity: usize, requested: usize) -> Self {
        Self::BufferOverflow {
            capacity,
            requested,
        }
    }

    /// Create invalid argument error
    pub fn invalid_argument(msg: &'static str) -> Self {
        Self::InvalidArgument { message: msg }
    }

    /// Create index out of bounds error
    pub fn index_out_of_bounds(index: usize, length: usize) -> Self {
        Self::IndexOutOfBounds { index, length }
    }

    /// Create config error
    pub fn config_error(msg: &'static str) -> Self {
        Self::ConfigError { message: msg }
    }

    /// Create format mismatch error
    pub fn format_mismatch(expected: &'static str, got: &'static str) -> Self {
        Self::FormatMismatch { expected, got }
    }

    /// Create sample rate mismatch error
    pub fn sample_rate_mismatch(expected: f32, got: f32) -> Self {
        Self::SampleRateMismatch { expected, got }
    }

    /// Create channel mismatch error
    pub fn channel_mismatch(expected: usize, got: usize) -> Self {
        Self::ChannelMismatch { expected, got }
    }

    /// Create not initialized error
    pub fn not_initialized(module: &'static str) -> Self {
        Self::NotInitialized { module }
    }

    /// Create already initialized error
    pub fn already_initialized(module: &'static str) -> Self {
        Self::AlreadyInitialized { module }
    }

    /// Create resource unavailable error
    pub fn resource_unavailable(resource: &'static str) -> Self {
        Self::ResourceUnavailable { resource }
    }

    /// Create timeout error
    pub fn timeout(operation: &'static str) -> Self {
        Self::Timeout { operation }
    }

    /// Create sync error
    pub fn sync_error(msg: &'static str) -> Self {
        Self::SyncError { message: msg }
    }

    /// Create MIDI error
    pub fn midi_error(msg: &'static str) -> Self {
        Self::MidiError { message: msg }
    }

    /// Create DSP error
    pub fn dsp_error(msg: &'static str) -> Self {
        Self::DSPError { message: msg }
    }

    /// Create serialization error
    pub fn serialization_error(msg: &'static str) -> Self {
        Self::SerializationError { message: msg }
    }

    /// Create I/O error
    pub fn io_error(msg: &'static str) -> Self {
        Self::IOError { message: msg }
    }

    /// Create generic error
    pub fn generic(msg: &'static str) -> Self {
        Self::Generic { message: msg }
    }

    /// Get error description
    pub fn description(&self) -> &'static str {
        match self {
            SmoothieError::BufferOverflow { .. } => "Buffer overflow",
            SmoothieError::InvalidArgument { .. } => "Invalid argument",
            SmoothieError::IndexOutOfBounds { .. } => "Index out of bounds",
            SmoothieError::ConfigError { .. } => "Configuration error",
            SmoothieError::FormatMismatch { .. } => "Format mismatch",
            SmoothieError::SampleRateMismatch { .. } => "Sample rate mismatch",
            SmoothieError::ChannelMismatch { .. } => "Channel count mismatch",
            SmoothieError::NotInitialized { .. } => "Not initialized",
            SmoothieError::AlreadyInitialized { .. } => "Already initialized",
            SmoothieError::ResourceUnavailable { .. } => "Resource unavailable",
            SmoothieError::Timeout { .. } => "Operation timed out",
            SmoothieError::SyncError { .. } => "Synchronization error",
            SmoothieError::MidiError { .. } => "MIDI error",
            SmoothieError::DSPError { .. } => "DSP algorithm error",
            SmoothieError::SerializationError { .. } => "Serialization error",
            SmoothieError::IOError { .. } => "I/O error",
            SmoothieError::Generic { .. } => "Generic error",
        }
    }
}

impl fmt::Display for SmoothieError {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmoothieError::BufferOverflow {
                capacity,
                requested,
            } => {
                write!(
                    f,
                    "Buffer overflow: capacity={}, requested={}",
                    capacity, requested
                )
            }
            SmoothieError::InvalidArgument { message } => {
                write!(f, "Invalid argument: {}", message)
            }
            SmoothieError::IndexOutOfBounds { index, length } => {
                write!(f, "Index out of bounds: index={}, length={}", index, length)
            }
            SmoothieError::ConfigError { message } => {
                write!(f, "Configuration error: {}", message)
            }
            SmoothieError::FormatMismatch { expected, got } => {
                write!(f, "Format mismatch: expected={}, got={}", expected, got)
            }
            SmoothieError::SampleRateMismatch { expected, got } => {
                write!(
                    f,
                    "Sample rate mismatch: expected={} Hz, got={} Hz",
                    expected, got
                )
            }
            SmoothieError::ChannelMismatch { expected, got } => {
                write!(f, "Channel mismatch: expected={}, got={}", expected, got)
            }
            SmoothieError::NotInitialized { module } => {
                write!(f, "Not initialized: {}", module)
            }
            SmoothieError::AlreadyInitialized { module } => {
                write!(f, "Already initialized: {}", module)
            }
            SmoothieError::ResourceUnavailable { resource } => {
                write!(f, "Resource unavailable: {}", resource)
            }
            SmoothieError::Timeout { operation } => {
                write!(f, "Operation timed out: {}", operation)
            }
            SmoothieError::SyncError { message } => {
                write!(f, "Synchronization error: {}", message)
            }
            SmoothieError::MidiError { message } => {
                write!(f, "MIDI error: {}", message)
            }
            SmoothieError::DSPError { message } => {
                write!(f, "DSP algorithm error: {}", message)
            }
            SmoothieError::SerializationError { message } => {
                write!(f, "Serialization error: {}", message)
            }
            SmoothieError::IOError { message } => {
                write!(f, "I/O error: {}", message)
            }
            SmoothieError::Generic { message } => {
                write!(f, "Error: {}", message)
            }
        }
    }
}

/// Simple Result alias using custom error type
pub type Result<T> = core::result::Result<T, SmoothieError>;

/// Technical implementation of the ErrorContext structure.
pub struct ErrorContext {
    error: SmoothieError,
    context: &'static str,
}

impl ErrorContext {
    /// Wrap error with context
    pub fn new(error: SmoothieError, context: &'static str) -> Self {
        Self { error, context }
    }

    /// Get wrapped error
    pub fn error(&self) -> &SmoothieError {
        &self.error
    }

    /// Get context
    pub fn context(&self) -> &'static str {
        self.context
    }
}

impl fmt::Display for ErrorContext {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.context, self.error)
    }
}

impl fmt::Debug for ErrorContext {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ErrorContext")
            .field("error", &self.error)
            .field("context", &self.context)
            .finish()
    }
}

/// Technical implementation of the ErrorAccumulator structure.
pub struct ErrorAccumulator {
    errors: [Option<SmoothieError>; 89], // 89 = Fibonacci, good for error collection
    count: usize,
}

impl ErrorAccumulator {
    /// Create new error accumulator (Silicon safe init)
    pub fn new() -> Self {
        let errors: [Option<SmoothieError>; 89] = unsafe {
            let mut data: core::mem::MaybeUninit<[Option<SmoothieError>; 89]> =
                core::mem::MaybeUninit::uninit();
            for i in 0..89 {
                core::ptr::write(&mut (*data.as_mut_ptr())[i], None);
            }
            data.assume_init()
        };

        Self { errors, count: 0 }
    }

    /// Add error to accumulator
    pub fn push(&mut self, error: SmoothieError) -> Result<()> {
        if self.count >= 89 {
            Err(SmoothieError::BufferOverflow {
                capacity: 89,
                requested: self.count + 1,
            })
        } else {
            self.errors[self.count] = Some(error);
            self.count += 1;
            Ok(())
        }
    }

    /// Check if any errors
    pub fn has_errors(&self) -> bool {
        self.count > 0
    }

    /// Get error count
    pub fn error_count(&self) -> usize {
        self.count
    }

    /// Get first error
    pub fn first_error(&self) -> Option<&SmoothieError> {
        self.errors[0].as_ref()
    }

    /// Get all errors
    pub fn errors(&self) -> &[Option<SmoothieError>] {
        &self.errors[..self.count]
    }

    /// Clear all errors
    pub fn clear(&mut self) {
        self.count = 0;
        for i in 0..89 {
            self.errors[i] = None;
        }
    }
}

impl Default for ErrorAccumulator {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for ErrorAccumulator {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ErrorAccumulator")
            .field("count", &self.count)
            .finish()
    }
}

/// Technical implementation of the Assertions structure.
pub struct Assertions;

impl Assertions {
    /// Assert condition, return error if false
    pub fn check(condition: bool, message: &'static str) -> Result<()> {
        if condition {
            Ok(())
        } else {
            Err(SmoothieError::invalid_argument(message))
        }
    }

    /// Assert equality
    pub fn equals<T: PartialEq + fmt::Debug>(
        left: T,
        right: T,
        message: &'static str,
    ) -> Result<()> {
        if left == right {
            Ok(())
        } else {
            Err(SmoothieError::invalid_argument(message))
        }
    }

    /// Assert not null (for pointers)
    pub fn not_null<T>(ptr: *const T, message: &'static str) -> Result<()> {
        if !ptr.is_null() {
            Ok(())
        } else {
            Err(SmoothieError::invalid_argument(message))
        }
    }

    /// Assert in range
    pub fn in_range(value: f32, min: f32, max: f32, message: &'static str) -> Result<()> {
        if value >= min && value <= max {
            Ok(())
        } else {
            Err(SmoothieError::invalid_argument(message))
        }
    }
}

/// Technical implementation of the RecoveryStrategy enumeration.
pub enum RecoveryStrategy {
    /// Use default value
    UseDefault,
    /// Skip operation
    Skip,
    /// Retry with backoff
    Retry,
    /// Propagate error
    Propagate,
    /// Try alternative
    UseAlternative,
}

impl RecoveryStrategy {
    /// Get recovery action description
    pub fn description(&self) -> &'static str {
        match self {
            RecoveryStrategy::UseDefault => "Using default value",
            RecoveryStrategy::Skip => "Skipping operation",
            RecoveryStrategy::Retry => "Retrying with backoff",
            RecoveryStrategy::Propagate => "Propagating error",
            RecoveryStrategy::UseAlternative => "Using alternative",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_error_creation logic.
    fn test_error_creation() {
        let err = SmoothieError::buffer_overflow(100, 150);
        assert_eq!(err.description(), "Buffer overflow");
    }

    #[test]
    /// Technical implementation of the test_error_context logic.
    fn test_error_context() {
        let err = SmoothieError::invalid_argument("test");
        let ctx = ErrorContext::new(err, "test_module");
        assert_eq!(ctx.context(), "test_module");
    }

    #[test]
    /// Technical implementation of the test_error_accumulator logic.
    fn test_error_accumulator() {
        let mut acc = ErrorAccumulator::new();
        let err = SmoothieError::generic("test error");
        let _ = acc.push(err);
        assert_eq!(acc.error_count(), 1);
        assert!(acc.has_errors());
    }

    #[test]
    /// Technical implementation of the test_assertions logic.
    fn test_assertions() {
        assert!(Assertions::check(true, "test").is_ok());
        assert!(Assertions::check(false, "test").is_err());
    }

    #[test]
    /// Technical implementation of the test_range_assertion logic.
    fn test_range_assertion() {
        assert!(Assertions::in_range(5.0, 0.0, 10.0, "in range").is_ok());
        assert!(Assertions::in_range(15.0, 0.0, 10.0, "out of range").is_err());
    }
}
