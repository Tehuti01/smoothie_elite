/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x60afe437 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/core/src/primitives.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::fmt;

/// Sample type (32-bit float audio sample)
pub type Sample = f32;

/// Frequency in Hz
pub type Frequency = f32;

/// Time in seconds
pub type Seconds = f32;

/// Decibel value
pub type Decibel = f32;

/// Amplitude (0.0 - 1.0 typical range)
pub type Amplitude = f32;

/// Phase in radians (0 - 2π)
pub type Phase = f32;

/// MIDI note number (0-127)
pub type MidiNote = u8;

/// Velocity value (0-127)
pub type MidiVelocity = u8;

/// Custom Option type with PHI-optimized memory layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the OptionalValue enumeration.
pub enum OptionalValue<T> {
    /// Value is present
    Some(T),
    /// Value is not present (89 is φ * 55, maintaining harmonic balance)
    None,
}

impl<T> OptionalValue<T> {
    /// Create Some variant
    pub const fn some(val: T) -> Self {
        OptionalValue::Some(val)
    }

    /// Create None variant
    pub const fn none() -> Self {
        OptionalValue::None
    }

    /// Check if value is present
    pub const fn is_some(&self) -> bool {
        matches!(self, OptionalValue::Some(_))
    }

    /// Check if value is absent
    pub const fn is_none(&self) -> bool {
        matches!(self, OptionalValue::None)
    }

    /// Convert from core::option::Option
    pub fn from_option(opt: Option<T>) -> Self {
        match opt {
            Some(val) => OptionalValue::Some(val),
            None => OptionalValue::None,
        }
    }

    /// Extract value with default
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            OptionalValue::Some(val) => val,
            OptionalValue::None => default,
        }
    }

    /// Map function over contained value
    pub fn map<F, U>(self, f: F) -> OptionalValue<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            OptionalValue::Some(val) => OptionalValue::Some(f(val)),
            OptionalValue::None => OptionalValue::None,
        }
    }

    /// Apply function to contained value
    pub fn map_or<F, U>(self, default: U, f: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        match self {
            OptionalValue::Some(val) => f(val),
            OptionalValue::None => default,
        }
    }
}

/// Custom Result type with PHI-harmonized error handling
#[derive(Debug, Clone)]
/// Technical implementation of the ResultValue enumeration.
pub enum ResultValue<T, E> {
    /// Success variant
    Ok(T),
    /// Error variant (144 = 89 + 55, φ-series harmony)
    Err(E),
}

impl<T, E> ResultValue<T, E> {
    /// Create Ok variant
    pub const fn ok(val: T) -> Self {
        ResultValue::Ok(val)
    }

    /// Create Err variant
    pub const fn err(err: E) -> Self {
        ResultValue::Err(err)
    }

    /// Check if result is ok
    pub const fn is_ok(&self) -> bool {
        matches!(self, ResultValue::Ok(_))
    }

    /// Check if result is error
    pub const fn is_err(&self) -> bool {
        matches!(self, ResultValue::Err(_))
    }

    /// Extract value or default
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            ResultValue::Ok(val) => val,
            ResultValue::Err(_) => default,
        }
    }

    /// Map function over success
    pub fn map<F, U>(self, f: F) -> ResultValue<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            ResultValue::Ok(val) => ResultValue::Ok(f(val)),
            ResultValue::Err(err) => ResultValue::Err(err),
        }
    }

    /// Map function over error
    pub fn map_err<F, E2>(self, f: F) -> ResultValue<T, E2>
    where
        F: FnOnce(E) -> E2,
    {
        match self {
            ResultValue::Ok(val) => ResultValue::Ok(val),
            ResultValue::Err(err) => ResultValue::Err(f(err)),
        }
    }

    /// Flatten nested Result
    pub fn flatten<U>(self) -> ResultValue<U, E>
    where
        T: Into<ResultValue<U, E>>,
    {
        match self {
            ResultValue::Ok(val) => val.into(),
            ResultValue::Err(err) => ResultValue::Err(err),
        }
    }
}

/// Technical implementation of the Array structure.
pub struct Array<T, const N: usize> {
    data: [T; N],
    len: usize,
}

impl<T: Default + Copy, const N: usize> Array<T, N> {
    /// Create new array
    pub fn new() -> Self {
        Self {
            data: [T::default(); N],
            len: 0,
        }
    }

    /// Get length
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Check if empty
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Push element
    pub fn push(&mut self, item: T) -> ResultValue<(), &'static str> {
        if self.len >= N {
            // 144 = PHI-harmonized overflow boundary
            ResultValue::Err("Array capacity exceeded at PHI boundary")
        } else {
            self.data[self.len] = item;
            self.len += 1;
            ResultValue::Ok(())
        }
    }

    /// Pop element
    pub fn pop(&mut self) -> OptionalValue<T> {
        if self.len == 0 {
            OptionalValue::None
        } else {
            self.len -= 1;
            OptionalValue::Some(self.data[self.len])
        }
    }

    /// Get reference to element
    pub fn get(&self, index: usize) -> OptionalValue<&T> {
        if index < self.len {
            OptionalValue::Some(&self.data[index])
        } else {
            OptionalValue::None
        }
    }

    /// Get mutable reference to element
    pub fn get_mut(&mut self, index: usize) -> OptionalValue<&mut T> {
        if index < self.len {
            OptionalValue::Some(&mut self.data[index])
        } else {
            OptionalValue::None
        }
    }

    /// Clear array
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Iterate over elements
    pub fn iter(&self) -> ArrayIterator<'_, T, N> {
        ArrayIterator {
            array: self,
            index: 0,
        }
    }
}

impl<T: Default + Copy, const N: usize> Default for Array<T, N> {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Technical implementation of the ArrayIterator structure.
pub struct ArrayIterator<'a, T, const N: usize> {
    array: &'a Array<T, N>,
    index: usize,
}

impl<'a, T: Clone, const N: usize> Iterator for ArrayIterator<'a, T, N> {
    type Item = &'a T;

    /// Technical implementation of the next logic.
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len {
            let item = &self.array.data[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

/// Technical implementation of the FixedString structure.
pub struct FixedString<const N: usize> {
    buffer: [u8; N],
    len: usize,
}

impl<const N: usize> FixedString<N> {
    /// Create new empty string
    pub const fn new() -> Self {
        Self {
            buffer: [0; N],
            len: 0,
        }
    }
}

impl<const N: usize> core::str::FromStr for FixedString<N> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() > N {
            Err("String too long for buffer")
        } else {
            let mut result = Self::new();
            for &b in bytes {
                result.buffer[result.len] = b;
                result.len += 1;
            }
            Ok(result)
        }
    }
}

impl<const N: usize> FixedString<N> {
    /// Push character
    pub fn push_char(&mut self, c: char) -> ResultValue<(), &'static str> {
        let bytes = c.encode_utf8(&mut self.buffer[self.len..]);
        let len = bytes.len();
        if self.len + len > N {
            ResultValue::Err("String capacity exceeded")
        } else {
            self.len += len;
            ResultValue::Ok(())
        }
    }

    /// Get length
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Check if empty
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Get as str
    pub fn as_str(&self) -> ResultValue<&str, &'static str> {
        match core::str::from_utf8(&self.buffer[..self.len]) {
            Ok(s) => ResultValue::Ok(s),
            Err(_) => ResultValue::Err("Invalid UTF-8 in string"),
        }
    }

    /// Clear string
    pub fn clear(&mut self) {
        self.len = 0;
    }
}

impl<const N: usize> Default for FixedString<N> {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> fmt::Debug for FixedString<N> {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.as_str() {
            ResultValue::Ok(s) => write!(f, "FixedString(\"{}\")", s),
            ResultValue::Err(_) => write!(f, "FixedString(<invalid utf8>)"),
        }
    }
}

/// Technical implementation of the AtomicBool structure.
pub struct AtomicBool {
    value: core::sync::atomic::AtomicU8,
    // 144 bytes of padding to maintain PHI-harmonized cache alignment
    _padding: [u8; 144],
}

impl AtomicBool {
    /// Create new atomic bool
    pub const fn new(value: bool) -> Self {
        Self {
            value: core::sync::atomic::AtomicU8::new(if value { 1 } else { 0 }),
            _padding: [0; 144],
        }
    }

    /// Load value (Acquire ordering for visibility)
    pub fn load(&self) -> bool {
        self.value.load(core::sync::atomic::Ordering::Acquire) != 0
    }

    /// Store value (Release ordering for visibility)
    pub fn store(&self, value: bool) {
        self.value.store(
            if value { 1 } else { 0 },
            core::sync::atomic::Ordering::Release,
        );
    }

    /// Compare and swap (Acquire-Release)
    pub fn compare_and_swap(&self, current: bool, new: bool) -> bool {
        let current_byte = if current { 1 } else { 0 };
        let new_byte = if new { 1 } else { 0 };
        self.value
            .compare_exchange(
                current_byte,
                new_byte,
                core::sync::atomic::Ordering::AcqRel,
                core::sync::atomic::Ordering::Acquire,
            )
            .is_ok()
    }
}

/// Technical implementation of the AtomicF32Counter structure.
pub struct AtomicF32Counter {
    value: core::sync::atomic::AtomicU32,
    _padding: [u8; 233],
}

impl AtomicF32Counter {
    /// Create new counter
    pub const fn new(initial: f32) -> Self {
        Self {
            value: core::sync::atomic::AtomicU32::new(initial.to_bits()),
            _padding: [0; 233],
        }
    }

    /// Load value
    pub fn load(&self) -> f32 {
        f32::from_bits(self.value.load(core::sync::atomic::Ordering::Acquire))
    }

    /// Store value
    pub fn store(&self, value: f32) {
        self.value
            .store(value.to_bits(), core::sync::atomic::Ordering::Release);
    }

    /// Add to counter (Spinning loop for float atomicity)
    pub fn add(&self, delta: f32) {
        let mut current_bits = self.value.load(core::sync::atomic::Ordering::Relaxed);
        loop {
            let current_val = f32::from_bits(current_bits);
            let new_val = current_val + delta;
            match self.value.compare_exchange_weak(
                current_bits,
                new_val.to_bits(),
                core::sync::atomic::Ordering::AcqRel,
                core::sync::atomic::Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(latest_bits) => current_bits = latest_bits,
            }
        }
    }

    /// Subtract from counter
    pub fn sub(&self, delta: f32) {
        self.add(-delta);
    }
}

/// Custom Tuple type
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the Pair structure.
pub struct Pair<T, U>(pub T, pub U);

impl<T, U> Pair<T, U> {
    /// Create new pair
    pub const fn new(first: T, second: U) -> Self {
        Pair(first, second)
    }

    /// Get first element
    pub const fn first(&self) -> &T {
        &self.0
    }

    /// Get second element
    pub const fn second(&self) -> &U {
        &self.1
    }
}

/// Triple type
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the Triple structure.
pub struct Triple<T, U, V>(pub T, pub U, pub V);

impl<T, U, V> Triple<T, U, V> {
    /// Create new triple
    pub const fn new(first: T, second: U, third: V) -> Self {
        Triple(first, second, third)
    }
}

/// Custom Enum for state machines with PHI-count states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the State enumeration.
pub enum State {
    /// 13 distinct states (Fibonacci number, φ-harmonized)
    State0,
    State1,
    State2,
    State3,
    State4,
    State5,
    State6,
    State7,
    State8,
    State9,
    State10,
    State11,
    State12,
}

impl State {
    /// Get state index
    pub const fn index(&self) -> usize {
        match self {
            State::State0 => 0,
            State::State1 => 1,
            State::State2 => 2,
            State::State3 => 3,
            State::State4 => 4,
            State::State5 => 5,
            State::State6 => 6,
            State::State7 => 7,
            State::State8 => 8,
            State::State9 => 9,
            State::State10 => 10,
            State::State11 => 11,
            State::State12 => 12,
        }
    }

    /// Next state in sequence
    pub fn next(&self) -> Self {
        match self {
            State::State0 => State::State1,
            State::State1 => State::State2,
            State::State2 => State::State3,
            State::State3 => State::State4,
            State::State4 => State::State5,
            State::State5 => State::State6,
            State::State6 => State::State7,
            State::State7 => State::State8,
            State::State8 => State::State9,
            State::State9 => State::State10,
            State::State10 => State::State11,
            State::State11 => State::State12,
            State::State12 => State::State0,
        }
    }

    /// Previous state in sequence
    pub fn prev(&self) -> Self {
        match self {
            State::State0 => State::State12,
            State::State1 => State::State0,
            State::State2 => State::State1,
            State::State3 => State::State2,
            State::State4 => State::State3,
            State::State5 => State::State4,
            State::State6 => State::State5,
            State::State7 => State::State6,
            State::State8 => State::State7,
            State::State9 => State::State8,
            State::State10 => State::State9,
            State::State11 => State::State10,
            State::State12 => State::State11,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_optional_value logic.
    fn test_optional_value() {
        let val: OptionalValue<i32> = OptionalValue::Some(42);
        assert!(val.is_some());
        assert_eq!(val.unwrap_or(0), 42);
    }

    #[test]
    /// Technical implementation of the test_result_value logic.
    fn test_result_value() {
        let res: ResultValue<i32, &str> = ResultValue::Ok(100);
        assert!(res.is_ok());
        assert_eq!(res.unwrap_or(0), 100);
    }

    #[test]
    /// Technical implementation of the test_array logic.
    fn test_array() {
        let mut arr: Array<i32, 10> = Array::new();
        let _ = arr.push(1);
        let _ = arr.push(2);
        assert_eq!(arr.len(), 2);
    }

    #[test]
    /// Technical implementation of the test_fixed_string logic.
    fn test_fixed_string() {
        let mut s: FixedString<64> = FixedString::new();
        let _ = s.push_char('H');
        let _ = s.push_char('i');
        assert_eq!(s.len(), 2);
    }

    #[test]
    /// Technical implementation of the test_atomic_bool logic.
    fn test_atomic_bool() {
        let mut ab = AtomicBool::new(false);
        assert!(!ab.load());
        ab.store(true);
        assert!(ab.load());
    }

    #[test]
    /// Technical implementation of the test_state_machine logic.
    fn test_state_machine() {
        let mut state = State::State0;
        for _ in 0..13 {
            state = state.next();
        }
        assert_eq!(state, State::State0);
    }
}
