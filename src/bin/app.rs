#![no_std]
#![no_main]
#![allow( non_snake_case )]

use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f7xx_hal::device::{CorePeripherals, Peripherals};
use tcp as _;

#[entry]
fn main( ) -> ! {    
    defmt::info!( "Hello, warren!" );
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

        asm::delay( 10_000_000 );

        gpiob.bsrr.write( | w | w 
            .br14( ).set_bit( ) );
        
        asm::delay( 10_000_000 );
    }
}


