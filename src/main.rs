#![no_main]
#![no_std]

#[allow(unused)]


use core::panic::PanicInfo;
use stm32f4xx_hal as hal;

use cortex_m_rt::entry;
use hal::delay::Delay;
use hal::prelude::*;
use hal::stm32;


// Entry point of the program
#[entry]
fn main() -> ! {

    // Core-Peripherals from the Cortex-Processor
    let cp = cortex_m::Peripherals::take().unwrap();
    // Hardware-Peripherals from the St32M-4x Controller
    let dp = stm32::Peripherals::take().unwrap();



    let sys= cp.SYST;
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();
    let mut delay_provider = Delay::new(sys, clocks);

    loop {
        delay_provider.delay_ms(200 as u32);
        let _ = led.set_high();
        delay_provider.delay_ms(200 as u32);
        let _ = led.set_low();
    }


// Function to handle a panic situation
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {

    }

}









}