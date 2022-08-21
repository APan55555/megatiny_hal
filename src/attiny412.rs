use crate::attiny412pac;
pub use crate::attiny412pac::Peripherals;
use core::ops::Deref;

pub fn get_periphs() -> Peripherals {
    unsafe { Peripherals::steal() }
}

pub enum TimeUnit {
    Ms,
    Us,
}

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

pub enum Pin {
    PA0,
    PA1,
    PA2,
    PA3,
    PA6,
    PA7,
}

impl Pin {
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

#[inline(always)]
pub fn delay(clock: &attiny412pac::rtc::RegisterBlock, del: u16) {
    clock.cnt.reset();
    while (clock.cnt.read().bits() as u32) < (del as u32) * 1024 / 1000 {
        continue;
    }
}
