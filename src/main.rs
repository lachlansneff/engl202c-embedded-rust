#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    let mut delay = hal::delay::Delay::tim5(dp.TIM5, &clocks);

    let gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output(); // The LED is on pin C13.

    loop {
        led.toggle(); // Toggle the LED on and off.
        delay.delay_ms(1000_u16); // Delay for 1 second (1000 milliseconds).
    }
}