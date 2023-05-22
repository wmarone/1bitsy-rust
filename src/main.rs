//! Demonstrate the use of a blocking `Delay` using the SYST (sysclock) timer.

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
//use panic_halt as _; // panic handler

use core::panic::PanicInfo;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // 1Bitsy LED is on GPIOA, PA8 
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa8.into_push_pull_output();

        // Set up the system clock. We want to run at 168MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(168.MHz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = cp.SYST.delay(&clocks);

        loop {
            // On for 1s, off for 1s.
            led.toggle();
            delay.delay_ms(1000_u32);
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_p: &PanicInfo) -> ! {

    loop { }

}
