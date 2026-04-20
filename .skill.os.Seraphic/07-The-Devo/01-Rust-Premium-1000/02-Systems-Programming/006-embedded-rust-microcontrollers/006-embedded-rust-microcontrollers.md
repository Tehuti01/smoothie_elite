# SKILL 006: EMBEDDED RUST & MICROCONTROLLERS - NO-STD MASTERY

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        EMBEDDED RUST & MICROCONTROLLERS
                     The Sovereign Guide to No-Std Systems Programming
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

Comprehensive mastery of embedded Rust编程 for ARM Cortex-M, ESP32, AVR microcontrollers.
Covers no_std environments, embeddedhal traits, interrupt handling, peripheral access,
RTOS integration, and building reliable embedded systems.

## TABLE OF CONTENTS

1. [No-Std Fundamentals](#no-std-fundamentals)
2. [ARM Cortex-M Deep Dive](#arm-cortex-m-deep-dive)
3. [ESP32 Programming](#esp32-programming)
4. [Interrupt Handling](#interrupt-handling)
5. [Peripheral Access](#peripheral-access)
6. [Embedded HAL](#embedded-hal)
7. [RTOS Integration](#rtos-integration)
8. [Communication Protocols](#communication-protocols)
9. [Power Management](#power-management)
10. [Testing Embedded](#testing-embedded)

---

## NO-STD FUNDAMENTALS

### 1.1 Core No-Std

```rust
#![no_std]
#![feature(abi_efiapi)]

use core::arch::asm;

/// Global allocator for no_std
#[global_allocator]
static ALLOCATOR: HEAP = HEAP { start: 0, end: 0 };

pub struct HEAP {
    start: usize,
    end: usize,
}

impl HEAP {
    pub const fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
    }
}

unsafe impl GlobalAllocator for HEAP {
    fn allocate(&self, layout: Layout) -> Result<*mut u8, AllocError> {
        let size = layout.size();
        let align = layout.align();
        
        // Simple bump allocator
        let ptr = self.start;
        self.start = (self.start + size + align - 1) & !(align - 1);
        
        if self.start > self.end {
            Err(AllocError)
        } else {
            Ok(ptr as *mut u8)
        }
    }

    fn deallocate(&self, _ptr: *mut u8, _layout: Layout) {
        // No-op for bump allocator
    }
}

/// Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {
        unsafe { asm!("wfi") }
    }
}

/// Entry point
#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {
        unsafe { asm!("wfi") }
    }
}
```

### 1.2 Startup Code

```rust
/// Vector table
#[repr(C)]
pub struct VectorTable {
    pub initial_sp: Option<unsafe fn()>,
    pub reset: unsafe fn(),
    pub nmi: unsafe fn(),
    pub hard_fault: unsafe fn(),
    pub mem_manage: unsafe fn(),
    pub bus_fault: unsafe fn(),
    pub usage_fault: unsafe fn(),
    pub reserved: [unsafe fn(); 4],
    pub sv_call: unsafe fn(),
    pub debug_monitor: unsafe fn(),
    pub pend_sv: unsafe fn(),
    pub systick: unsafe fn(),
    pub irq: [unsafe fn(); 64],
}

extern "C" {
    fn reset() -> !;
    fn nmi();
    fn hard_fault();
}

/// Cortex-M startup
#[link_section = ".vectors"]
#[no_mangle]
pub static __VECTORS: VectorTable = VectorTable {
    initial_sp: None,
    reset,
    nmi,
    hard_fault,
    mem_manage: cortex_m_default_handler,
    bus_fault: cortex_m_default_handler,
    usage_fault: cortex_m_default_handler,
    reserved: [cortex_m_default_handler; 4],
    sv_call: cortex_m_default_handler,
    debug_monitor: cortex_m_default_handler,
    pend_sv: cortex_m_default_handler,
    systick: cortex_m_default_handler,
    irq: [cortex_m_default_handler; 64],
};

fn cortex_m_default_handler() {
    loop {
        unsafe { asm!("wfi") }
    }
}
```

---

## ARM CORTEX-M DEEP DIVE

### 2.1 GPIO Manipulation

```rust
use volatile_register::{RO, RW, WO};

/// GPIO Port
#[repr(C)]
pub struct GpioPort {
    moder: RW<u32>,
    otyper: RW<u32>,
    ospeedr: RW<u32>,
    pupdr: RW<u32>,
    idr: RO<u32>,
    odr: RW<u32>,
    bsrr: WO<u32>,
    lckr: RW<u32>,
    afrl: RW<u32>,
    afrh: RW<u32>,
}

/// Pin modes
#[repr(u32)]
pub enum PinMode {
    Input = 0b00,
    Output = 0b01,
    Alternate = 0b10,
    Analog = 0b11,
}

/// GPIO pins
pub struct Pin {
    port: &'static mut GpioPort,
    pin: u8,
}

impl Pin {
    pub fn new(port: &'static mut GpioPort, pin: u8) -> Self {
        Pin { port, pin }
    }

    pub fn set_mode(&mut self, mode: PinMode) {
        let shift = self.pin * 2;
        self.port.moder.modify(|v| {
            let mask = 0b11 << shift;
            (v & !mask) | ((mode as u32) << shift)
        });
    }

    pub fn set(&self) {
        self.port.bsrr.write(1 << self.pin);
    }

    pub fn clear(&self) {
        self.port.bsrr.write(1 << (self.pin + 16));
    }

    pub fn read(&self) -> bool {
        self.port.idr.read() & (1 << self.pin) != 0
    }

    pub fn toggle(&self) {
        self.port.odr.modify(|v| v ^ (1 << self.pin));
    }
}
```

### 2.2 System Timer

```rust
/// ARM System Timer (SysTick)
#[repr(C)]
pub struct SysTick {
    pub csr: RW<u32>,
    pub rvr: RW<u32>,
    pub cvr: RW<u32>,
    pub calib: RO<u32>,
}

impl SysTick {
    pub fn new(base: usize) -> &'static mut SysTick {
        unsafe { &mut *(base as *mut SysTick) }
    }

    pub fn enable(&mut self, reload: u32) {
        self.rvr.write(reload);
        self.cvr.write(0);
        self.csr.write(0x7); // Enable, tick internal, clock source
    }

    pub fn disable(&mut self) {
        self.csr.write(0);
    }

    pub fn countflag(&self) -> bool {
        self.csr.read() & (1 << 16) != 0
    }
}

///延时
pub fn delay_ms(ms: u32) {
    let freq = 48_000_000_u32;
    let reload = (freq / 1000 * ms) - 1;
    
    let systick = unsafe { &mut *(0xE000_E010 as *mut SysTick) };
    systick.enable(reload);
    
    while !systick.countflag() {}
    
    systick.disable();
}
```

---

## ESP32 PROGRAMMING

### 3.1 ESP-IDF Integration

```rust
use esp_idf_sys as _;

/// ESP32 GPIO
pub fn configure_gpio(pin: u8, mode: GpioMode) {
    unsafe {
        let mut conf = esp_gpioConf_t::__bindgen_anon_1::default();
        
        conf.set_pin_bit = 1;
        conf.set_mode = match mode {
            GpioMode::Input => 0,
            GpioMode::Output => 1,
            GpioMode::OutputOD => 2,
            GpioMode::InputIO => 3,
        };
        conf.set_pull_up_en = 1;
        
        gpio_config(&conf);
    }
}

/// WiFi Station
pub fn wifi_join(ssid: &str, password: &str) -> Result<(), EspError> {
    let config = wifi_config_t {
        sta: wifi_sta_config_t {
            ssid: [0u8; 32],
            password: [0u8; 64],
            ..Default::default()
        },
    };
    
    // Copy SSID and password
    unsafe {
        let ssid_bytes = ssid.as_bytes();
        let password_bytes = password.as_bytes();
        
        // ...
    }
    
    wifi_sta_set_config(&config)?;
    wifi_sta_connect()?;
    Ok(())
}
```

### 3.2 FreeRTOS Tasks

```rust
use esp_idf_hal::task::thread_fspawn;

pub fn create_task(
    name: &'static str,
    stack_size: usize,
    priority: u8,
    main: unsafe extern "C" fn(*mut core::ffi::c_void),
) -> Result<TaskHandle, ()> {
    let handle: TaskHandle = core::ptr::null_mut();
    
    let result = unsafe {
        xTaskCreate(
            Some(main),
            name.as_ptr() as *const i8,
            stack_size as u32,
            core::ptr::null_mut(),
            priority as u32,
            &handle,
        )
    };
    
    if result == pdTRUE {
        Ok(handle)
    } else {
        Err(())
    }
}
```

---

## INTERRUPT HANDLING

### 4.1 Interrupt Attributes

```rust
use cortex_m::asm;

/// Interrupt handler attribute
#[interrupt]
fn UART0() {
    unsafe {
        // Read from UART
        let data = (*UART0_BASE).dr.read();
        
        if data & 0x100 != 0 {
            // Error
        } else {
            // Data
            let _ = data as u8;
        }
    }
    
    // Clear interrupt
    unsafe { (*UART0_BASE).icr.write(1) };
}

/// Critical sections
pub fn critical_section<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    cortex_m::interrupt::free(|_| f())
}

/// Atomic operations
pub fn atomic_test_and_set(flag: &core::sync::atomic::AtomicBool) -> bool {
    cortex_m::interrupt::free(|_| flag.load(core::sync::atomic::Ordering::Acquire))
}
```

### 4.2 Hardware Interrupts

```rust
use cortex_m::peripheral::NVIC;

/// Enable interrupt
pub fn enable_interrupt(irq: IRQ) {
    unsafe {
        NVIC::unmask(irq);
    }
}

/// Disable interrupt
pub fn disable_interrupt(irq: IRQ) {
    NVIC::mask(irq);
}

/// Set priority
pub fn set_priority(irq: IRQ, priority: u8) {
    unsafe {
        NVIC::set_priority(irq, priority);
    }
}

/// Pending interrupt
pub fn pend_interrupt(irq: IRQ) {
    NVIC::pend(irq);
}
```

---

## PERIPHERAL ACCESS

### 5.1 UART

```rust
/// UART registers
#[repr(C)]
pub struct UartRegs {
    dr: RW<u32>,
    sr: RO<u32>,
    brr: RW<u32>,
    cr1: RW<u32>,
    cr2: RW<u32>,
    cr3: RW<u32>,
    gtpr: RW<u32>,
}

/// UART configuration
pub fn uart_init(baud: u32) {
    let uart = unsafe { &mut *(UART1_BASE as *mut UartRegs) };
    
    // Disable UART
    uart.cr1.write(0);
    
    // Set baud rate
    let brr = 48_000_000 / baud;
    uart.brr.write(brr);
    
    // Enable RX/TX, 8-bit
    uart.cr1.write(0x202C);
}

pub fn uart_putc(c: u8) {
    let uart = unsafe { &mut *(UART1_BASE as *mut UartRegs) };
    
    while uart.sr.read() & 0x80 == 0 {} // Wait TX empty
    
    uart.dr.write(c as u32);
}

pub fn uart_getc() -> Option<u8> {
    let uart = unsafe { &mut *(UART1_BASE as *mut UartRegs) };
    
    if uart.sr.read() & 0x20 != 0 { // RX not empty
        Some(uart.dr.read() as u8)
    } else {
        None
    }
}
```

### 5.2 SPI

```rust
/// SPI configuration
pub fn spi_init(baud: u32) {
    let spi = unsafe { &mut *(SPI1_BASE as *mut SpiRegs) };
    
    // Clock: fPCLK / 2^((br >> 3) + 1)
    let br = (48_000_000 / baud).trailing_zeros() as u32;
    let br = (br - 1).clamp(0, 6) << 3;
    
    spi.cr1.write(0x0300 | br | 0x0040); // Master, software CS
}

pub fn spi_transfer(data: &[u8]) -> Vec<u8> {
    let spi = unsafe { &mut *(SPI1_BASE as *mut SpiRegs) };
    let mut result = Vec::with_capacity(data.len());
    
    for byte in data {
        while spi.sr.read() & 0x02 == 0 {} // Wait TX empty
        spi.dr.write(*byte as u32);
        
        while spi.sr.read() & 0x01 == 0 {} // Wait RX not empty
        result.push(spi.dr.read() as u8);
    }
    
    result
}
```

---

## EMBEDDED HAL

### 6.1 Embedded-HAL Traits

```rust
use embedded_hal::{
    digital::v2::{InputPin, OutputPin, StatefulOutputPin},
    serial::Serial,
    spi::FullDuplex,
};

/// Custom GPIO implementation
pub struct GpioPin<P> {
    pin: P,
}

impl<P: OutputPin> OutputPin for GpioPin<P> {
    type Error = P::Error;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()
    }
}

/// UART HAL
pub struct SerialUart<T> {
    uart: T,
    _phantom: core::marker::PhantomData<()>,
}

impl<T: Serial> Serial for SerialUart<T> {
    type Tx = T;
    type Rx = T;
    type Error = T::Error;

    fn try_split(
        self,
    ) -> Result<(Self::Tx, Self::Rx), Self::Error> {
        todo!()
    }

    fn use_spi(self) -> Self::Error {
        todo!()
    }
}
```

### 6.2 Driver Implementation

```rust
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

/// HD44780 LCD driver
pub struct Lcd<I2C> {
    i2c: I2C,
    backlight: bool,
}

impl<I2C: Write> Lcd<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Lcd { i2c, backlight: true }
    }

    pub fn init(&mut self) {
        self.send(0x30, CommandMode);
        self.delay_us(4100);
        self.send(0x30, CommandMode);
        self.delay_us(100);
        self.send(0x30, CommandMode);
        self.send(0x20, CommandMode); // 4-bit mode
        self.send(0x28, CommandMode); // 2 lines, 5x8
        self.send(0x08, CommandMode); // Display off
        self.send(0x01, CommandMode); // Clear
        self.delay_us(2000);
        self.send(0x06, CommandMode); // Entry mode
    }
}
```

---

## RTOS INTEGRATION

### 7.1 FreeRTOS Tasks

```rust
use freertos_sys::*;

pub fn create_freertos_task() {
    let mut task = TaskHandle_t::default();
    let stack = Box::into_raw(Box::new([0u8; 2048])) as *mut u8;
    
    unsafe {
        xTaskCreate(
            Some(task_entry),
            c_str!("my_task".as_ptr()),
            512,
            core::ptr::null_mut(),
            1,
            &mut task,
        );
    }
}

unsafe extern "C" fn task_entry(_params: *mut core::ffi::c_void) {
    loop {
        // Task work
        vTaskDelay(pdMS_TO_TICKS(1000));
    }
}

/// Queue management
pub struct MessageQueue<T> {
    queue: QueueHandle_t,
    _phantom: core::marker::PhantomData<T>,
}

impl<T: Sized + Copy> MessageQueue<T> {
    pub fn new(size: u32) -> Result<Self, ()> {
        let mut queue: QueueHandle_t = core::ptr::null_mut();
        
        let result = unsafe {
            xQueueCreate(size, core::mem::size_of::<T>() as u32, &mut queue)
        };
        
        if result == pdTRUE {
            Ok(MessageQueue {
                queue,
                _phantom: core::marker::PhantomData,
            })
        } else {
            Err(())
        }
    }

    pub fn send(&self, msg: &T, timeout: u32) -> Result<(), ()> {
        let result = unsafe {
            xQueueSend(self.queue, msg as *const T as *const _, timeout)
        };
        
        if result == pdTRUE {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn receive(&self, msg: &mut T, timeout: u32) -> Result<(), ()> {
        let result = unsafe {
            xQueueReceive(self.queue, msg as *mut T as *mut _, timeout)
        };
        
        if result == pdTRUE {
            Ok(())
        } else {
            Err(())
        }
    }
}
```

---

## COMMUNICATION PROTOCOLS

### 8.1 I2C

```rust
/// I2C master
pub struct I2cMaster<SCL, SDA> {
    scl: SCL,
    sda: SDA,
}

impl<SCL, SDA> I2cMaster<SCL, SDA> {
    pub fn write(&mut self, addr: u8, data: &[u8]) -> Result<(), Error> {
        // Start
        self.sda.set_low();
        self.scl.set_low();
        
        // Address
        self.write_byte(addr << 1)?;
        
        // Data
        for byte in data {
            self.write_byte(*byte)?;
        }
        
        // Stop
        self.stop();
        
        Ok(())
    }

    pub fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Error> {
        // Start
        self.start();
        
        // Address with read bit
        self.write_byte((addr << 1) | 1)?;
        
        // Data
        for byte in buffer.iter_mut() {
            *byte = self.read_byte()?;
        }
        
        // NACK and stop
        self.stop();
        
        Ok(())
    }
}
```

### 8.2 CAN Bus

```rust
/// CAN message
#[repr(C)]
pub struct CanMessage {
    pub id: u32,
    pub dlc: u8,
    pub data: [u8; 8],
}

pub struct CanBus {
    can: &'static mut CanRegisters,
}

impl CanBus {
    pub fn init(&mut self) {
        // Initialize CAN controller
    }

    pub fn transmit(&mut self, msg: &CanMessage) -> Result<(), CanError> {
        // Check tx mailbox
        while self.can.tsr.read() & 0x1 == 0 {}
        
        // Write ID
        self.can.txmailbox0
            .idr
            .write(if msg.id > 0x7FF { msg.id } else { msg.id << 18 });
        
        // Write data
        self.can.txmailbox0.tdlr.write(u32::from_le_bytes([
            msg.data[0],
            msg.data[1],
            msg.data[2],
            msg.data[3],
        ]));
        
        // Request transmit
        self.can.txmailbox0.tir.write(0x01);
        
        Ok(())
    }
}
```

---

## POWER MANAGEMENT

### 9.1 Sleep Modes

```rust
/// Enter sleep mode
pub fn sleep() {
    unsafe {
        // Set sleep bit
        SCB::sleeper_write(0x01);
        
        // Wait for interrupt
        asm!("wfi");
    }
}

/// Deep sleep
pub fn deep_sleep(duration_ms: u32) {
    // Configure RTC alarm
    unsafe {
        // Set wakeup timer
        RTC.alarm0.write(duration_ms);
        
        // Enable wakeup
        RTC.cr.modify(|v| v | 0x01);
        
        // Enter deep sleep
        SCB::sleeper_write(0x04);
        asm!("wfi");
    }
}

/// Stop mode
pub fn stop_mode() {
    unsafe {
        // Configure voltage regulator for low power
        PWR.cr.modify(|v| {
            (v & 0xFFFFFFF3) | 0x01
        });
        
        // Enter stop mode
        SCB::sleeper_write(0x02);
        asm!("wfi");
    }
}
```

### 9.2 Dynamic Voltage

```rust
/// Scale frequency
pub fn scale_frequency(mhz: u32) {
    // Configure PLL for new frequency
    let pllm = 8u8;
    let plln = (mhz * 2) as u8;
    let pllp = 0u8; // /2
    
    RCC.pllcfgr.modify(|v| {
        (v & 0xFFFFC000) |
        ((pllm as u32) << 0) |
        ((plln as u32) << 6) |
        ((pllp as u32) << 16)
    });
    
    // Wait for PLL lock
    while !RCC.cr.read() & 0x02000000 != 0 {}
}
```

---

## TESTING EMBEDDED

### 10.1 Test Infrastructure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pin_mode() {
        // Test pin mode configuration
        let mut pin = Pin::new(unsafe { &mut *(0x4002_1C00 as *mut GpioPort) }, 5);
        
        pin.set_mode(PinMode::Output);
        pin.set();
        assert!(pin.read());
        
        pin.clear();
        assert!(!pin.read());
    }
    
    #[test]
    fn test_delay() {
        // Test delay function timing
        let start = Systick::now();
        delay_ms(100);
        let elapsed = start.elapsed();
        
        assert!(elapsed >= 99 && elapsed <= 110);
    }
}
```

---

## RECAP

1. **Start minimal** - Enable only what you need
2. **Use embedded-hal** - Portable drivers
3. **Handle interrupts carefully** - Keep them short
4. **Test on hardware** - QEMU isn't enough
5. **Watch power** - Sleep modes matter
6. **Use RTOS when needed** - For complex apps

---

*Skill ID: 006 | Category: Systems-Programming | Complexity: Expert*
*Version: 1.0.0 | Last Updated: 2024*