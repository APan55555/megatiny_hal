use crate::attiny412pac;
pub use crate::attiny412pac::Peripherals;
use core::ops::Deref;

/// Gets the peripherals and returns them
/// 
/// # Example
/// 
/// ```
/// let periphs = get_periphs();
/// ```
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
/// let timer = init_timer(&periphs, milliseconds_unit);
/// ```
/// 
/// ```
/// let periphs = get_periphs();
/// let microseconds_unit = TimeUnit::Us;
/// let timer = init_timer(&periphs, microseconds_unit);
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
/// let milliseconds_timer = init_timer(&periphs, TimeUnit::Ms);
/// ```
/// 
/// ```
/// let periphs = get_periphs();
/// let microseconds_timer = init_timer(&periphs, TimeUnit::Us);
#[inline(always)]
pub fn init_timer(p: &Peripherals, unit: TimeUnit) -> &attiny412pac::rtc::RegisterBlock {
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
            //selects 1kHz clock
            rtc.
                clksel.
                write(|w| {w.clksel().int32k()});
        }
    }
    //enables rtc
    rtc.
        ctrla.
        write(|w| unsafe {w.bits(1)});

    rtc
}

/// Enum that specifies a pin initialized with the Pin! macro in lib.rs
/// 
/// # Example
/// 
/// ```
/// let periphs = get_periphs();
/// let led_pin = Pin!(periphs, PA3 output);
/// ```
pub enum Pin {
    PA0,
    PA1,
    PA2,
    PA3,
    PA6,
    PA7,
}

/// Gives support for reading, writing, and toggling the pin
impl Pin {
    /// Writes the pin high
    /// 
    /// # Example
    /// 
    /// ```
    /// let periphs = get_periphs();
    /// let led_pin = Pin!(periphs, PA3 output);
    /// led_pin.write_high(&periphs);
    /// ```
    #[inline(always)]
    pub fn write_high(&self, p: &Peripherals) {
        match self {
            Self::PA0 => {
                let porta = p.PORTA.deref();
                porta.outset.write(|w| unsafe { w.bits(0b1) });
            },
            Self::PA1 => {
                let porta = p.PORTA.deref();
                porta.outset.write(|w| unsafe { w.bits(0b1 << 1) })
            },
            Self::PA2 => {
                let porta = p.PORTA.deref();
                porta.outset.write(|w| unsafe { w.bits(0b1 << 2) })
            },
            Self::PA3 => {
                let porta = p.PORTA.deref();
                porta.outset.write(|w| unsafe { w.bits(0b1 << 3) })
            },
            Self::PA6 => {
                let porta = p.PORTA.deref();
                porta.outset.write(|w| unsafe { w.bits(0b1 << 6) })
            },
            Self::PA7 => {
                let porta = p.PORTA.deref();
                porta.outset.write(|w| unsafe { w.bits(0b1 << 7) })
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
    /// led_pin.write_low(&periphs);
    /// ```
    #[inline(always)]
    pub fn write_low(&self, p: &Peripherals) {
        match self {
            Self::PA0 => {
                let porta = p.PORTA.deref();
                porta.outclr.write(|w| unsafe { w.bits(0b1) });
            },
            Self::PA1 => {
                let porta = p.PORTA.deref();
                porta.outclr.write(|w| unsafe { w.bits(0b1 << 1) })
            },
            Self::PA2 => {
                let porta = p.PORTA.deref();
                porta.outclr.write(|w| unsafe { w.bits(0b1 << 2) })
            },
            Self::PA3 => {
                let porta = p.PORTA.deref();
                porta.outclr.write(|w| unsafe { w.bits(0b1 << 3) })
            },
            Self::PA6 => {
                let porta = p.PORTA.deref();
                porta.outclr.write(|w| unsafe { w.bits(0b1 << 6) })
            },
            Self::PA7 => {
                let porta = p.PORTA.deref();
                porta.outclr.write(|w| unsafe { w.bits(0b1 << 7) })
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
    /// led_pin.write_high(&periphs);
    /// ```
    #[inline(always)]
    pub fn toggle(&self, p: &Peripherals) {
        match self {
            Self::PA0 => {
                let porta = p.PORTA.deref();
                porta.outtgl.write(|w| unsafe { w.bits(0b1) });
            },
            Self::PA1 => {
                let porta = p.PORTA.deref();
                porta.outtgl.write(|w| unsafe { w.bits(0b1 << 1) })
            },
            Self::PA2 => {
                let porta = p.PORTA.deref();
                porta.outtgl.write(|w| unsafe { w.bits(0b1 << 2) })
            },
            Self::PA3 => {
                let porta = p.PORTA.deref();
                porta.outtgl.write(|w| unsafe { w.bits(0b1 << 3) })
            },
            Self::PA6 => {
                let porta = p.PORTA.deref();
                porta.outtgl.write(|w| unsafe { w.bits(0b1 << 6) })
            },
            Self::PA7 => {
                let porta = p.PORTA.deref();
                porta.outtgl.write(|w| unsafe { w.bits(0b1 << 7) })
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
    /// let button_state = button_pin.read(&periphs);
    /// ```
    #[inline(always)]
    pub fn read(&self, p: &Peripherals) -> bool {
        match self {
            Self::PA0 => {
                let porta = p.PORTA.deref();
                porta.in_.read().pa0().bit()
            },
            Self::PA1 => {
                let porta = p.PORTA.deref();
                porta.in_.read().pa1().bit()
            },
            Self::PA2 => {
                let porta = p.PORTA.deref();
                porta.in_.read().pa2().bit()
            },
            Self::PA3 => {
                let porta = p.PORTA.deref();
                porta.in_.read().pa3().bit()
            },
            Self::PA6 => {
                let porta = p.PORTA.deref();
                porta.in_.read().pa6().bit()
            },
            Self::PA7 => {
                let porta = p.PORTA.deref();
                porta.in_.read().pa7().bit()
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
/// let timer = init_timer(&periphs, TimeUnit::Ms);
/// delay(timer, 1000);
/// ```
#[inline(always)]
pub fn delay(timer: &attiny412pac::rtc::RegisterBlock, del: u32) {
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
/// let timer = init_timer(&periphs, TimeUnit::Us);
/// delay_us(timer, 1000000);
/// ```
pub fn delay_us(timer: &attiny412pac::rtc::RegisterBlock, del: u64) {
    timer.cnt.reset();
    while (timer.cnt.read().bits() as u64) < (del * 512) / 15625 {
        continue;
    }
}