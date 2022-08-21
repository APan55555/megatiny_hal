#![no_std]
#![no_main]

extern crate panic_halt;

use core::ops::Deref;
use::megatiny_hal::attiny412;
use megatiny_hal::attiny412::{TimeUnit, Pin};
use megatiny_hal::Pin;

#[no_mangle]
pub extern "C" fn main() {
    let periphs = attiny412::get_periphs();
    let clock = attiny412::init_timer(&periphs, TimeUnit::Ms);
    let led_pin = Pin!(periphs, PA3 output);
    loop {
        led_pin.toggle(&periphs);
        attiny412::delay(clock, 1000);
    }
}
