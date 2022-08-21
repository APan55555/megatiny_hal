#![no_std]

pub mod attiny412pac;
pub mod attiny412;

#[macro_export]
macro_rules! Pin {
    ($p:expr, PA0 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1) });
            Pin::PA0
        }
    };
    ($p:expr, PA0 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1) });
            Pin::PA0
        }
    };
    ($p:expr, PA1 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 1) });
            Pin::PA1
        }
    };
    ($p:expr, PA1 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 1) });
            Pin::PA1
        }
    };
    ($p:expr, PA2 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 2) });
            Pin::PA2
        }
    };
    ($p:expr, PA2 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 2) });
            Pin::PA2
        }
    };
    ($p:expr, PA3 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 3) });
            Pin::PA3
        }
    };
    ($p:expr, PA3 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 3) });
            Pin::PA3
        }
    };
    ($p:expr, PA4 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 4) });
            Pin::PA4
        }
    };
    ($p:expr, PA4 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 4) });
            Pin::PA4
        }
    };
    ($p:expr, PA5 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 5) });
            Pin::PA5
        }
    };
    ($p:expr, PA5 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 5) });
            Pin::PA5
        }
    };
    ($p:expr, PA6 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 6) });
            Pin::PA6
        }
    };
    ($p:expr, PA6 input) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 6) });
            Pin::PA6
        }
    };
    ($p:expr, PA7 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirset.write(|w| unsafe { w.bits(0b1 << 7) });
            Pin::PA7
        }
    };
    ($p:expr, PA7 output) => {
        {
            let porta = $p.PORTA.deref();
            porta.dirclr.write(|w| unsafe { w.bits(0b1 << 7) });
            Pin::PA7
        }
    };
}
