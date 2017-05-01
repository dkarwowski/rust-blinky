#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f429x;

use cortex_m::asm;
use stm32f429x::{GPIOG, RCC};

fn main() {
    cortex_m::interrupt::free(
        |cs| {
            let gpiog = GPIOG.borrow(cs);
            let rcc = RCC.borrow(cs);

            rcc.ahb1enr.modify(|_, w| w.gpiogen().enabled());
            gpiog.moder.modify(|_, w| w.moder13().output());
            gpiog.otyper.modify(|_, w| w.ot13().pushpull());
            gpiog.ospeedr.modify(|_, w| w.ospeedr13().high());
            gpiog.pupdr.modify(|_, w| w.pupdr13().none());

            let mut count: u32;

            loop {
                count = 13 * 5000;
                if gpiog.odr.read().odr13().bits() != 0u8 {
                    gpiog.bsrr.write(|w| w.br13().reset());
                } else {
                    gpiog.bsrr.write(|w| w.bs13().set());
                }
                loop {
                    count = count - 1;
                    if count < 1 {
                        break;
                    }
                }
            }
        },
    );
}

#[allow(dead_code)]
#[link_section = ".rodata.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
