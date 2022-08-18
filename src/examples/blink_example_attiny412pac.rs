#![no_std]
#![no_main]

extern crate panic_halt;

use core::ops::Deref;
use megatiny_hal::attiny412pac::Peripherals;

#[no_mangle]
pub extern "C" fn main() {
    let peripherals = unsafe { Peripherals::steal() };
    let porta = peripherals.PORTA.deref();
    let rtc = peripherals.RTC.deref();

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

    porta.dirset.write(|w| unsafe { w.bits(0b1 << 3) }); //sets PA3 as output
    loop {
        porta
            .outtgl
            .write(|w| unsafe { w.bits(0b1 << 3) });
        //delays 1 second
        rtc.cnt.reset();
        while rtc.cnt.read().bits() < 2000 {
            continue;
        }
    }
}
