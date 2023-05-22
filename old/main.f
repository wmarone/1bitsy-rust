#![no_main]
#![no_std]

#[allow(unused_imports)]
#[allow(unused_variables)]

use core::panic::PanicInfo;
use cortex_m_rt::{entry, exception};

use stm32f4xx_hal::{pac, prelude::*}; // the rcc import is unused.


#[entry]
fn main() -> ! {
    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    let rcc = device.RCC.constrain();
    let cfgr = rcc.cfgr;
    let hse  = cfgr.use_hse(8.MHz());
    let sysclk = hse.sysclk(168.MHz());
    let pclk1 = sysclk.pclk1(24.MHz());
    let reqpll = pclk1.require_pll48clk();
    let clocks = reqpll.freeze();

    let _ = device.SYSCFG.constrain();

    let gpioa = device.GPIOA.split();

    let mut led = gpioa.pa5.into_push_pull_output();

    let mut delay = device.TIM2.delay_ms(&clocks);

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
