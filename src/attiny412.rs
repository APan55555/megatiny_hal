use crate::attiny412pac as pac;
use crate::attiny412pac::Peripherals;
use core::ops::Deref;

#[macro_export]
macro_rules! init {
    () => {
        let PERIF = unsafe {Peripherals::steal()};
    };
}

#[macro_export]
macro_rules! init_timer {
    ($($p:expr), ms) => {
        {
            let rtc = $p.RTC.deref();
            //makes sure everything is synced
            while rtc.status.read().bits() > 0 {
                continue;
            }

            //selects 1kHz clock
            rtc.
                clksel.
                write(|w| {w.clksel().int1k()});

            //sets the top to max
            rtc.
                per.
                write(|w| unsafe {w.bits(0xFFFF)});

            //enables rtc
            rtc.
                ctrla.
                write(|w| unsafe {w.bits(1)});
        }
    }

    ($($p:expr), us) => {
        {
            let rtc = $p.RTC.deref();
            //makes sure everything is synced
            while rtc.status.read().bits() > 0 {
                continue;
            }

            //selects 1kHz clock
            rtc.
                clksel.
                write(|w| {w.clksel().int32k()});

            //sets the top to max
            rtc.
                per.
                write(|w| unsafe {w.bits(0xFFFF)});

            //enables rtc
            rtc.
                ctrla.
                write(|w| unsafe {w.bits(1)});
        }
    }
}

pub enum Pin<'a> {
    PA0,
    PA1,
    PA2,
    PA3,
    PA4,
    PA5,
    PA6,
    PA7,
}

#[macro_export]
macro_rules! Pin {
    (&($p:expr), PA0 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1) });
            Pin::PA0
        }
    }
    ($($p:expr), PA0 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1) });
            Pin::PA0
        }
    }
    ($($p:expr), PA1 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 1) });
            Pin::PA1
        }
    }
    ($($p:expr), PA1 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 1) });
            Pin::PA1
        }
    }
    ($($p:expr), PA2 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 2) });
            Pin::PA2
        }
    }
    ($($p:expr), PA2 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 2) });
            Pin::PA2
        }
    }
    ($($p:expr), PA3 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 3) });
            Pin::PA3
        }
    }
    ($($p:expr), PA3 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 3) });
            Pin::PA3
        }
    }
    ($($p:expr), PA4 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 4) });
            Pin::PA4
        }
    }
    ($($p:expr), PA4 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 4) });
            Pin::PA4
        }
    }
    ($($p:expr), PA5 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 5) });
            Pin::PA5
        }
    }
    ($($p:expr), PA5 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 5) });
            Pin::PA5
        }
    }
    ($($p:expr), PA6 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 6) });
            Pin::PA6
        }
    }
    ($($p:expr), PA6 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 6) });
            Pin::PA6
        }
    }
    ($($p:expr), PA7 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 7) });
            Pin::PA7
        }
    }
    ($($p:expr), PA7 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 7) });
            Pin::PA7
        }
    }
}

impl Pin {
    #[inline(always)]
    pub fn write_high(&self, &p: Peripherals) {
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
            Self::PA4 => {
                let porta = p.PORTA.deref();
                porta.outset.write(|w| unsafe { w.bits(0b1 << 4) })
            },
            Self::PA5 => {
                let porta = p.PORTA.deref();
                porta.outset.write(|w| unsafe { w.bits(0b1 << 5) })
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
    pub fn write_low(&self, &p: Peripherals) {
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
            Self::PA4 => {
                let porta = p.PORTA.deref();
                porta.outclr.write(|w| unsafe { w.bits(0b1 << 4) })
            },
            Self::PA5 => {
                let porta = p.PORTA.deref();
                porta.outclr.write(|w| unsafe { w.bits(0b1 << 5) })
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
    pub fn toggle(&self, &p: Peripherals) {
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
            Self::PA4 => {
                let porta = p.PORTA.deref();
                porta.outtgl.write(|w| unsafe { w.bits(0b1 << 4) })
            },
            Self::PA5 => {
                let porta = p.PORTA.deref();
                porta.outtgl.write(|w| unsafe { w.bits(0b1 << 5) })
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
    pub fn read(&self, &p: Peripherals) -> bool {
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
            Self::PA4 => {
                let porta = p.PORTA.deref();
                porta.in_.read().pa4().bit()
            },
            Self::PA5 => {
                let porta = p.PORTA.deref();
                porta.in_.read().pa5().bit()
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
pub fn delay(p: &Peripherals, del: u16) {
    let rtc = p.RTC.deref();
    rtc.reset();
    while rtc.cnt.read().bits() < ((del as u32) * 1024 / 1000) as u16 {
        continue;
    }
}
