#![no_std]
#![no_main]

extern crate panic_halt;

use core::ops::Deref;
use megatiny_hal::attiny412;
use megatiny_hal::attiny412::{TimeUnit, Pin, PinSelect};
use megatiny_hal::{Pin, GPIO};

#[no_mangle]
pub extern "C" fn main() {
    let periphs = attiny412::get_periphs();  // gets the peripherals
    attiny412::init_timer(&periphs, TimeUnit::Ms);  // initializes the timer
    let led_pin = Pin!(&periphs, PA3 output);  // creates an led pin for PA3 as an output
    loop {
        led_pin.toggle();  // toggles the led pin
        attiny412::delay(&periphs, 1000);  // delays for 1000 milliseconds (1 second)
    }
}
