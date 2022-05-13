#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32_hal2;

use stm32_hal2::{
    clocks::{Clocks},
    gpio::{Pin, PinMode, Port},
};

extern crate cortex_m;
use cortex_m::delay::Delay;
extern crate cortex_m_rt;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();

    // Configure clock to 48 MHz and freeze it
    let clock_cfg = Clocks::default();
    clock_cfg.setup().unwrap();

    let mut delay = Delay::new(cp.SYST, clock_cfg.systick());
    let mut led = Pin::new(Port::A, 5, PinMode::Output);

    loop {
        led.set_high();
        delay.delay_ms(500_u32);
        led.set_low();
        delay.delay_ms(500_u32);
    }
}

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
