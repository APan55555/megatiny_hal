//! Crate containing the hardware abstraction layers for attiny and atmega avr microcontrollers
//! 
//! # Contains support for
//! 
//! - attiny412

#![no_std]
#![feature(lang_items)]

pub mod attiny412pac;
pub mod attiny412;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

/// Returns a Pin enum with input formatted (\[peripherals\], \[pin number formatted \[port\]\[pin number\]\] input/output)
/// 
/// # Examples
/// 
/// ```
/// let periphs = get_periphs();
/// let led_pin = Pin!(periphs, PA3 output);
/// ```
/// 
/// ```
/// let periphs = get_periphs();
/// let button_pin = Pin!(periphs, PA1 input);
/// ```
#[macro_export]
macro_rules! Pin {
    ($p:expr, PA0 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1) });
            Pin::new($p, PinSelect::PA0)
        }
    };
    ($p:expr, PA0 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1) });
            Pin::new($p, PinSelect::PA0)
        }
    };
    ($p:expr, PA1 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 1) });
            Pin::new($p, PinSelect::PA1)
        }
    };
    ($p:expr, PA1 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 1) });
            Pin::new($p, PinSelect::PA1)
        }
    };
    ($p:expr, PA2 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 2) });
            Pin::new($p, PinSelect::PA2)
        }
    };
    ($p:expr, PA2 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 2) });
            Pin::new($p, PinSelect::PA2)
        }
    };
    ($p:expr, PA3 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 3) });
            Pin::new($p, PinSelect::PA3)
        }
    };
    ($p:expr, PA3 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 3) });
            Pin::new($p, PinSelect::PA3)
        }
    };
    ($p:expr, PA4 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 4) });
            Pin::new($p, PinSelect::PA4)
        }
    };
    ($p:expr, PA4 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 4) });
            Pin::new($p, PinSelect::PA4)
        }
    };
    ($p:expr, PA5 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 5) });
            Pin::new($p, PinSelect::PA5)
        }
    };
    ($p:expr, PA5 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 5) });
            Pin::new($p, PinSelect::PA5)
        }
    };
    ($p:expr, PA6 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 6) });
            Pin::new($p, PinSelect::PA6)
        }
    };
    ($p:expr, PA6 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 6) });
            Pin::new($p, PinSelect::PA6)
        }
    };
    ($p:expr, PA7 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 7) });
            Pin::new($p, PinSelect::PA7)
        }
    };
    ($p:expr, PA7 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 7) });
            Pin::new($p, PinSelect::PA7)
        }
    };
}

///A trait for GPIO pins
pub trait GPIO {
    fn write_high(&self);

    fn write_low(&self);

    fn toggle(&self);

    fn read(&self) -> bool;
}

/*
#[macro_export]
macro_rules! Interrupt {
    (attiny412, $p:expr, $i:expr, $f:expr) => {
        match $i {
            Interrupt::CRCSCAN_NMI => {
                $p.NVMCTRL.deref()
            }
        }
    }
}*/