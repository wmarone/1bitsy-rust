#![no_main]
#![no_std]

#[allow(unused_imports)]
#[allow(unused_variables)]

use core::panic::PanicInfo;
use cortex_m_rt::{entry, exception};

use stm32f4xx_hal::{pac, prelude::*}; // the rcc import is unused.

use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let device = pac::Peripherals::take().unwrap();
    //let core = cortex_m::Peripherals::take().unwrap();

    let rcc = device.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(168.MHz())
        .pclk1(24.MHz())
        .i2s_clk(86.MHz())
        .require_pll48clk()
        .freeze();

    let _ = device.SYSCFG.constrain();

    let gpioa = device.GPIOA.split();

    let mut led = gpioa.pa5.into_push_pull_output();

    let mut delay = device.TIM2.delay_ms(&clocks);

    rprintln!("y a ba ba ba ba");

    loop { 

        led.toggle();
        delay.delay_ms(200u16);
    }

}

#[exception]
fn SysTick() {
}

#[panic_handler]
fn panic(_p: &PanicInfo) -> ! {

    loop { }

}
