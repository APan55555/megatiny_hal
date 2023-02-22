//! The hardware abstraction layer for the attiny412

use crate::attiny412pac::Peripherals;
use crate::GPIO;
use core::ops::Deref;

/// Gets the peripherals and returns them
/// 
/// # Example
/// 
/// ```
/// let periphs = get_periphs();
/// ```
#[inline(always)]
pub fn get_periphs() -> Peripherals {
    unsafe { Peripherals::steal() }
}

/// Enum to specify time units (Ms for milliseconds, Us for microseconds)
/// 
/// # Examples
/// 
/// ```
/// let periphs = get_periphs();
/// let milliseconds_unit = TimeUnit::Ms;
/// init_timer(&periphs, milliseconds_unit);
/// ```
/// 
/// ```
/// let periphs = get_periphs();
/// let microseconds_unit = TimeUnit::Us;
/// init_timer(&periphs, microseconds_unit);
/// ```
pub enum TimeUnit {
    Ms,
    Us,
}

/// Initializes the timer using the peripherals based on the specified time unit and returns a reference to it
/// 
/// # Examples
/// 
/// ```
/// let periphs = get_periphs();
/// init_timer(&periphs, TimeUnit::Ms);  // initialize timer for milliseconds
/// ```
/// 
/// ```
/// let periphs = get_periphs();
/// init_timer(&periphs, TimeUnit::Us);  // initialize timer for microseconds
/// ```
#[inline(always)]
pub fn init_timer(p: &Peripherals, unit: TimeUnit) {
    let rtc = p.RTC.deref();
    //makes sure everything is synced
    while rtc.status.read().bits() > 0 {
        continue;
    }
    //sets the top to max
    rtc.
        per.
        write(|w| unsafe {w.bits(0xFFFF)});
    match unit {
        TimeUnit::Ms => {
            //selects 1kHz clock
            rtc.
                clksel.
                write(|w| {w.clksel().int1k()});
        },
        TimeUnit::Us => {
            //selects 32kHz clock
            rtc.
                clksel.
                write(|w| {w.clksel().int32k()});
        }
    }
    //enables rtc
    rtc.
        ctrla.
        write(|w| unsafe {w.bits(1)});
}

/// Enum that specifies a pin
pub enum PinSelect {
    PA0,
    PA1,
    PA2,
    PA3,
    PA6,
    PA7,
}

/// Struct that specifies a pin initialized with the Pin! macro in lib.rs
/// 
/// # Example
/// 
/// ```
/// let periphs = get_periphs();
/// let led_pin = Pin!(periphs, PA3 output);
/// ```
pub struct Pin<'a> {
    p: &'a Peripherals,
    pin: PinSelect,
}

impl Pin<'_> {
    /// Constructor
    #[inline(always)]
    pub fn new(p: &Peripherals, pin: PinSelect) -> Pin {
        Pin {p, pin}
    }
}

/// Gives support for reading, writing, and toggling the pin
impl GPIO for Pin<'_> {
    /// Writes the pin high
    /// 
    /// # Example
    /// 
    /// ```
    /// let periphs = get_periphs();
    /// let led_pin = Pin!(periphs, PA3 output);
    /// led_pin.write_high();
    /// ```
    #[inline(always)]
    fn write_high(&self) {
        match self.pin {
            PinSelect::PA0 => {
                self.p.PORTA.deref().outset.write(|w| unsafe { w.bits(0b1) });
            },
            PinSelect::PA1 => {
                self.p.PORTA.deref().outset.write(|w| unsafe { w.bits(0b1 << 1) })
            },
            PinSelect::PA2 => {
                self.p.PORTA.deref().outset.write(|w| unsafe { w.bits(0b1 << 2) })
            },
            PinSelect::PA3 => {
                self.p.PORTA.deref().outset.write(|w| unsafe { w.bits(0b1 << 3) })
            },
            PinSelect::PA6 => {
                self.p.PORTA.deref().outset.write(|w| unsafe { w.bits(0b1 << 6) })
            },
            PinSelect::PA7 => {
                self.p.PORTA.deref().outset.write(|w| unsafe { w.bits(0b1 << 7) })
            },
        }
    }

    /// Writes the pin low
    /// 
    /// # Example
    /// 
    /// ```
    /// let periphs = get_periphs();
    /// let led_pin = Pin!(periphs, PA3 output);
    /// led_pin.write_low();
    /// ```
    #[inline(always)]
    fn write_low(&self) {
        match self.pin {
            PinSelect::PA0 => {
                self.p.PORTA.deref().outclr.write(|w| unsafe { w.bits(0b1) });
            },
            PinSelect::PA1 => {
                self.p.PORTA.deref().outclr.write(|w| unsafe { w.bits(0b1 << 1) })
            },
            PinSelect::PA2 => {
                self.p.PORTA.deref().outclr.write(|w| unsafe { w.bits(0b1 << 2) })
            },
            PinSelect::PA3 => {
                self.p.PORTA.deref().outclr.write(|w| unsafe { w.bits(0b1 << 3) })
            },
            PinSelect::PA6 => {
                self.p.PORTA.deref().outclr.write(|w| unsafe { w.bits(0b1 << 6) })
            },
            PinSelect::PA7 => {
                self.p.PORTA.deref().outclr.write(|w| unsafe { w.bits(0b1 << 7) })
            },
        }
    }

    /// Toggles the pin
    /// 
    /// # Example
    /// 
    /// ```
    /// let periphs = get_periphs();
    /// let led_pin = Pin!(periphs, PA3 output);
    /// led_pin.toggle();
    /// ```
    #[inline(always)]
    fn toggle(&self) {
        match self.pin {
            PinSelect::PA0 => {
                self.p.PORTA.deref().outtgl.write(|w| unsafe { w.bits(0b1) });
            },
            PinSelect::PA1 => {
                self.p.PORTA.deref().outtgl.write(|w| unsafe { w.bits(0b1 << 1) })
            },
            PinSelect::PA2 => {
                self.p.PORTA.deref().outtgl.write(|w| unsafe { w.bits(0b1 << 2) })
            },
            PinSelect::PA3 => {
                self.p.PORTA.deref().outtgl.write(|w| unsafe { w.bits(0b1 << 3) })
            },
            PinSelect::PA6 => {
                self.p.PORTA.deref().outtgl.write(|w| unsafe { w.bits(0b1 << 6) })
            },
            PinSelect::PA7 => {
                self.p.PORTA.deref().outtgl.write(|w| unsafe { w.bits(0b1 << 7) })
            },
        }
    }

    /// Reads the pin and returns a boolean
    /// 
    /// # Example
    /// 
    /// ```
    /// let periphs = get_periphs();
    /// let button_pin = Pin!(periphs, PA3 input);
    /// let button_state = button_pin.read();
    /// ```
    #[inline(always)]
    fn read(&self) -> bool {
        match self.pin {
            PinSelect::PA0 => {
                self.p.PORTA.deref().in_.read().pa0().bit()
            },
            PinSelect::PA1 => {
                self.p.PORTA.deref().in_.read().pa1().bit()
            },
            PinSelect::PA2 => {
                self.p.PORTA.deref().in_.read().pa2().bit()
            },
            PinSelect::PA3 => {
                self.p.PORTA.deref().in_.read().pa3().bit()
            },
            PinSelect::PA6 => {
                self.p.PORTA.deref().in_.read().pa6().bit()
            },
            PinSelect::PA7 => {
                self.p.PORTA.deref().in_.read().pa7().bit()
            },
        }
    }
}

/// Delays with timer input and time in milliseconds
/// 
/// Note: Max delay is 64 seconds (del = 64000), if a delay greater than the max is entered, it will go into an infinite loop
/// 
/// Note: Will compile but will not work expectedly if the timer is initialized in microseconds
/// 
/// # Example
/// 
/// ```
/// let periphs = get_periphs();
/// init_timer(&periphs, TimeUnit::Ms);
/// delay(&periphs, 1000);
/// ```
#[inline(always)]
pub fn delay(p: &Peripherals, del: u32) {
    let timer = p.RTC.deref();
    timer.cnt.reset();
    while (timer.cnt.read().bits() as u32) < (del * 1024) / 1000 {
        continue;
    }
}

/// Delays with timer input and time in microseconds
/// 
/// Note: Max delay is 2 seconds (del = 2E6), if a delay greater than the max is entered, it will go into an infinite loop
/// 
/// Note: The resolution is ~30.5 microseconds (more precisely 1/32768 seconds) and always rounds down
/// 
/// Note: Will compile but will not work expectedly if the timer is initialized in milliseconds
/// 
/// # Example
/// 
/// ```
/// let periphs = get_periphs();
/// init_timer(&periphs, TimeUnit::Us);
/// delay_us(&periphs, 1000000);
/// ```
#[inline(always)]
pub fn delay_us(p: &Peripherals, del: u64) {
    let timer = p.RTC.deref();
    timer.cnt.reset();
    while (timer.cnt.read().bits() as u64) < (del * 512) / 15625 {
        continue;
    }
}