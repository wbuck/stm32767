#![no_std]
#![no_main]
#![allow( non_snake_case )]

use panic_semihosting as _;
use cortex_m::asm::delay;
use cortex_m_rt::entry;
use stm32f7::stm32f7x7::{Peripherals, CorePeripherals};

#[entry]
fn main( ) -> ! {
    // Cortex-M peripherals.
    let _core = CorePeripherals::take( ).unwrap( );

    // Device specific peripherals.
    let device = Peripherals::take( ).unwrap( );

    device.RCC.ahb1enr.modify( | _, w | 
        w.gpioben( ).enabled( ) );

    let gpiob = device.GPIOB;
    gpiob.moder.modify( | _, w | w.moder14( ).output( ) );
    gpiob.otyper.modify( | _, w | w.ot14( ).push_pull( ) );

    loop { 
        gpiob.bsrr.write( | w | w 
            .br14( ).clear_bit( )
            .bs14( ).set( ) );

        delay( 5_000_000 );

        gpiob.bsrr.write( | w | w 
            .br14( ).set_bit( ) );
        
        delay( 5_000_000 );
    }
}


