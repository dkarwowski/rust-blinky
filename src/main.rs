#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate f4;

use cortex_m::asm;
use f4::stm32f429x::{GPIOG, RCC};
use f4::gpio::GPIO;
use f4::gpio;

fn main() {
    cortex_m::interrupt::free(
        |cs| {
            let gpiog = GPIOG.borrow(cs);
            let rcc = RCC.borrow(cs);

            rcc.ahb1enr.modify(|_, w| w.gpiogen().enabled());
            gpiog.pin_enable(
                gpio::PIN13,
                gpio::Mode::Out,
                gpio::Speed::Low,
                gpio::Output::PushPull,
                gpio::PullUpPullDown::NoPull
            );

            let count = 13 * 5000;

            loop {
                if gpiog.odr.read().odr13().bits() != 0u8 {
                    gpiog.pin_reset(gpio::PIN13);
                } else {
                    gpiog.pin_set(gpio::PIN13);
                }
                delay(count);
            }
        },
    );
}

fn delay(count: u32) {
    for _ in 0..count { cortex_m::asm::nop() }
}

#[allow(dead_code)]
#[link_section = ".rodata.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
