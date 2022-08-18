#![doc = "Peripheral access API for ATTINY412 microcontrollers (generated using svd2rust v0.24.1 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.24.1/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
#[doc = "Analog Comparator"]
pub struct AC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AC0 {}
impl AC0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ac0::RegisterBlock = 0x0670 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ac0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for AC0 {
    type Target = ac0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for AC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AC0").finish()
    }
}
#[doc = "Analog Comparator"]
pub mod ac0;
#[doc = "Analog to Digital Converter"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const adc0::RegisterBlock = 0x0600 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ADC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC0").finish()
    }
}
#[doc = "Analog to Digital Converter"]
pub mod adc0;
#[doc = "Bod interface"]
pub struct BOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BOD {}
impl BOD {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const bod::RegisterBlock = 0x80 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bod::RegisterBlock {
        Self::PTR
    }
}
impl Deref for BOD {
    type Target = bod::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for BOD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOD").finish()
    }
}
#[doc = "Bod interface"]
pub mod bod;
#[doc = "Configurable Custom Logic"]
pub struct CCL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCL {}
impl CCL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const ccl::RegisterBlock = 0x01c0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CCL {
    type Target = ccl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CCL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCL").finish()
    }
}
#[doc = "Configurable Custom Logic"]
pub mod ccl;
#[doc = "Clock controller"]
pub struct CLKCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLKCTRL {}
impl CLKCTRL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const clkctrl::RegisterBlock = 0x60 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const clkctrl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CLKCTRL {
    type Target = clkctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CLKCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKCTRL").finish()
    }
}
#[doc = "Clock controller"]
pub mod clkctrl;
#[doc = "CPU"]
pub struct CPU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPU {}
impl CPU {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const cpu::RegisterBlock = 0x34 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cpu::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CPU {
    type Target = cpu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CPU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU").finish()
    }
}
#[doc = "CPU"]
pub mod cpu;
#[doc = "Interrupt Controller"]
pub struct CPUINT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CPUINT {}
impl CPUINT {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const cpuint::RegisterBlock = 0x0110 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cpuint::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CPUINT {
    type Target = cpuint::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CPUINT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPUINT").finish()
    }
}
#[doc = "Interrupt Controller"]
pub mod cpuint;
#[doc = "CRCSCAN"]
pub struct CRCSCAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRCSCAN {}
impl CRCSCAN {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const crcscan::RegisterBlock = 0x0120 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crcscan::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CRCSCAN {
    type Target = crcscan::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CRCSCAN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRCSCAN").finish()
    }
}
#[doc = "CRCSCAN"]
pub mod crcscan;
#[doc = "Digital to Analog Converter"]
pub struct DAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC0 {}
impl DAC0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dac0::RegisterBlock = 0x0680 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DAC0 {
    type Target = dac0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DAC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC0").finish()
    }
}
#[doc = "Digital to Analog Converter"]
pub mod dac0;
#[doc = "Event System"]
pub struct EVSYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EVSYS {}
impl EVSYS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const evsys::RegisterBlock = 0x0180 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const evsys::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EVSYS {
    type Target = evsys::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EVSYS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVSYS").finish()
    }
}
#[doc = "Event System"]
pub mod evsys;
#[doc = "Fuses"]
pub struct FUSE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FUSE {}
impl FUSE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const fuse::RegisterBlock = 0x1280 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fuse::RegisterBlock {
        Self::PTR
    }
}
impl Deref for FUSE {
    type Target = fuse::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for FUSE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUSE").finish()
    }
}
#[doc = "Fuses"]
pub mod fuse;
#[doc = "General Purpose IO"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpio::RegisterBlock = 0x1c as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO").finish()
    }
}
#[doc = "General Purpose IO"]
pub mod gpio;
#[doc = "Lockbit"]
pub struct LOCKBIT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LOCKBIT {}
impl LOCKBIT {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const lockbit::RegisterBlock = 0x128a as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lockbit::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LOCKBIT {
    type Target = lockbit::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LOCKBIT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCKBIT").finish()
    }
}
#[doc = "Lockbit"]
pub mod lockbit;
#[doc = "Non-volatile Memory Controller"]
pub struct NVMCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMCTRL {}
impl NVMCTRL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const nvmctrl::RegisterBlock = 0x1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nvmctrl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for NVMCTRL {
    type Target = nvmctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for NVMCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NVMCTRL").finish()
    }
}
#[doc = "Non-volatile Memory Controller"]
pub mod nvmctrl;
#[doc = "I/O Ports"]
pub struct PORTA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTA {}
impl PORTA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const porta::RegisterBlock = 0x0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const porta::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PORTA {
    type Target = porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PORTA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTA").finish()
    }
}
#[doc = "I/O Ports"]
pub mod porta;
#[doc = "Port Multiplexer"]
pub struct PORTMUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTMUX {}
impl PORTMUX {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const portmux::RegisterBlock = 0x0200 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const portmux::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PORTMUX {
    type Target = portmux::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PORTMUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTMUX").finish()
    }
}
#[doc = "Port Multiplexer"]
pub mod portmux;
#[doc = "Reset controller"]
pub struct RSTCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTCTRL {}
impl RSTCTRL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rstctrl::RegisterBlock = 0x40 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rstctrl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RSTCTRL {
    type Target = rstctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RSTCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTCTRL").finish()
    }
}
#[doc = "Reset controller"]
pub mod rstctrl;
#[doc = "Real-Time Counter"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc::RegisterBlock = 0x0140 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC").finish()
    }
}
#[doc = "Real-Time Counter"]
pub mod rtc;
#[doc = "Signature row"]
pub struct SIGROW {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SIGROW {}
impl SIGROW {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const sigrow::RegisterBlock = 0x1100 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sigrow::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SIGROW {
    type Target = sigrow::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SIGROW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGROW").finish()
    }
}
#[doc = "Signature row"]
pub mod sigrow;
#[doc = "Sleep Controller"]
pub struct SLPCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SLPCTRL {}
impl SLPCTRL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const slpctrl::RegisterBlock = 0x50 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const slpctrl::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SLPCTRL {
    type Target = slpctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SLPCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLPCTRL").finish()
    }
}
#[doc = "Sleep Controller"]
pub mod slpctrl;
#[doc = "Serial Peripheral Interface"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi0::RegisterBlock = 0x0820 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI0").finish()
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi0;
#[doc = "System Configuration Registers"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const syscfg::RegisterBlock = 0x0f01 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscfg::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SYSCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCFG").finish()
    }
}
#[doc = "System Configuration Registers"]
pub mod syscfg;
#[doc = "16-bit Timer Type B"]
pub struct TCB0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCB0 {}
impl TCB0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tcb0::RegisterBlock = 0x0a40 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcb0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TCB0 {
    type Target = tcb0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TCB0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCB0").finish()
    }
}
#[doc = "16-bit Timer Type B"]
pub mod tcb0;
#[doc = "Timer Counter D"]
pub struct TCD0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TCD0 {}
impl TCD0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const tcd0::RegisterBlock = 0x0a80 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tcd0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TCD0 {
    type Target = tcd0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TCD0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCD0").finish()
    }
}
#[doc = "Timer Counter D"]
pub mod tcd0;
#[doc = "Two-Wire Interface"]
pub struct TWI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI0 {}
impl TWI0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const twi0::RegisterBlock = 0x0810 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twi0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TWI0 {
    type Target = twi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TWI0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWI0").finish()
    }
}
#[doc = "Two-Wire Interface"]
pub mod twi0;
#[doc = "Universal Synchronous and Asynchronous Receiver and Transmitter"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart0::RegisterBlock = 0x0800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART0").finish()
    }
}
#[doc = "Universal Synchronous and Asynchronous Receiver and Transmitter"]
pub mod usart0;
#[doc = "User Row"]
pub struct USERROW {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USERROW {}
impl USERROW {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const userrow::RegisterBlock = 0x1300 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const userrow::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USERROW {
    type Target = userrow::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USERROW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USERROW").finish()
    }
}
#[doc = "User Row"]
pub mod userrow;
#[doc = "Virtual Ports"]
pub struct VPORTA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VPORTA {}
impl VPORTA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const vporta::RegisterBlock = 0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vporta::RegisterBlock {
        Self::PTR
    }
}
impl Deref for VPORTA {
    type Target = vporta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for VPORTA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VPORTA").finish()
    }
}
#[doc = "Virtual Ports"]
pub mod vporta;
#[doc = "Virtual Ports"]
pub struct VPORTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VPORTB {}
impl VPORTB {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const vportb::RegisterBlock = 0x04 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vportb::RegisterBlock {
        Self::PTR
    }
}
impl Deref for VPORTB {
    type Target = vportb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for VPORTB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VPORTB").finish()
    }
}
#[doc = "Virtual Ports"]
pub mod vportb;
#[doc = "Virtual Ports"]
pub struct VPORTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VPORTC {}
impl VPORTC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const vportc::RegisterBlock = 0x08 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vportc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for VPORTC {
    type Target = vportc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for VPORTC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VPORTC").finish()
    }
}
#[doc = "Virtual Ports"]
pub mod vportc;
#[doc = "Voltage reference"]
pub struct VREF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREF {}
impl VREF {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const vref::RegisterBlock = 0xa0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vref::RegisterBlock {
        Self::PTR
    }
}
impl Deref for VREF {
    type Target = vref::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for VREF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VREF").finish()
    }
}
#[doc = "Voltage reference"]
pub mod vref;
#[doc = "Watch-Dog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const wdt::RegisterBlock = 0x0100 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        Self::PTR
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WDT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT").finish()
    }
}
#[doc = "Watch-Dog Timer"]
pub mod wdt;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "AC0"]
    pub AC0: AC0,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "BOD"]
    pub BOD: BOD,
    #[doc = "CCL"]
    pub CCL: CCL,
    #[doc = "CLKCTRL"]
    pub CLKCTRL: CLKCTRL,
    #[doc = "CPU"]
    pub CPU: CPU,
    #[doc = "CPUINT"]
    pub CPUINT: CPUINT,
    #[doc = "CRCSCAN"]
    pub CRCSCAN: CRCSCAN,
    #[doc = "DAC0"]
    pub DAC0: DAC0,
    #[doc = "EVSYS"]
    pub EVSYS: EVSYS,
    #[doc = "FUSE"]
    pub FUSE: FUSE,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "LOCKBIT"]
    pub LOCKBIT: LOCKBIT,
    #[doc = "NVMCTRL"]
    pub NVMCTRL: NVMCTRL,
    #[doc = "PORTA"]
    pub PORTA: PORTA,
    #[doc = "PORTMUX"]
    pub PORTMUX: PORTMUX,
    #[doc = "RSTCTRL"]
    pub RSTCTRL: RSTCTRL,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SIGROW"]
    pub SIGROW: SIGROW,
    #[doc = "SLPCTRL"]
    pub SLPCTRL: SLPCTRL,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "TCB0"]
    pub TCB0: TCB0,
    #[doc = "TCD0"]
    pub TCD0: TCD0,
    #[doc = "TWI0"]
    pub TWI0: TWI0,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USERROW"]
    pub USERROW: USERROW,
    #[doc = "VPORTA"]
    pub VPORTA: VPORTA,
    #[doc = "VPORTB"]
    pub VPORTB: VPORTB,
    #[doc = "VPORTC"]
    pub VPORTC: VPORTC,
    #[doc = "VREF"]
    pub VREF: VREF,
    #[doc = "WDT"]
    pub WDT: WDT,
}
impl Peripherals {
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            AC0: AC0 {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            BOD: BOD {
                _marker: PhantomData,
            },
            CCL: CCL {
                _marker: PhantomData,
            },
            CLKCTRL: CLKCTRL {
                _marker: PhantomData,
            },
            CPU: CPU {
                _marker: PhantomData,
            },
            CPUINT: CPUINT {
                _marker: PhantomData,
            },
            CRCSCAN: CRCSCAN {
                _marker: PhantomData,
            },
            DAC0: DAC0 {
                _marker: PhantomData,
            },
            EVSYS: EVSYS {
                _marker: PhantomData,
            },
            FUSE: FUSE {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            LOCKBIT: LOCKBIT {
                _marker: PhantomData,
            },
            NVMCTRL: NVMCTRL {
                _marker: PhantomData,
            },
            PORTA: PORTA {
                _marker: PhantomData,
            },
            PORTMUX: PORTMUX {
                _marker: PhantomData,
            },
            RSTCTRL: RSTCTRL {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SIGROW: SIGROW {
                _marker: PhantomData,
            },
            SLPCTRL: SLPCTRL {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            TCB0: TCB0 {
                _marker: PhantomData,
            },
            TCD0: TCD0 {
                _marker: PhantomData,
            },
            TWI0: TWI0 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USERROW: USERROW {
                _marker: PhantomData,
            },
            VPORTA: VPORTA {
                _marker: PhantomData,
            },
            VPORTB: VPORTB {
                _marker: PhantomData,
            },
            VPORTC: VPORTC {
                _marker: PhantomData,
            },
            VREF: VREF {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
        }
    }
}
