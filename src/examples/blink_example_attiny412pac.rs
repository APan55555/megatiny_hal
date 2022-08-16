#![no_std]
#![no_main]

extern crate panic_halt;

use core::ops::Deref;
use megatiny_hal::Peripherals;

#[no_mangle]
pub extern "C" fn main() {
    let peripherals = unsafe { Peripherals::steal() };
    let vporta = peripherals.VPORTA.deref();
    let rtc = peripherals.RTC.deref();

    vporta.dir.write(|w| unsafe { w.bits(0b1 << 3) });
    loop {
        vporta
            .out
            .modify(|r, w| unsafe { w.bits(r.bits() | 0b1 << 3) });
        //delays 1 second
        rtc.cnt.reset();
        let mut start = rtc.cnt.read().bits();
        while rtc.cnt.read().bits() - start < 1000 {
            continue;
        }

        vporta
            .out
            .modify(|r, w| unsafe { w.bits(r.bits() & !(0b1 << 3)) });
        //delays 1 second
        rtc.cnt.reset();
        start = rtc.cnt.read().bits();
        while rtc.cnt.read().bits() - start < 1000 {
            continue;
        }
    }
}
