#![no_std]
#![no_main]
#![allow( non_snake_case )]

use tcp as _;
use defmt as log;

#[rtic::app( device = stm32f7xx_hal::device, peripherals = true )]
const APP: ( ) = {

    #[init]
    fn init( _cx: init::Context ) {
        log::warn!( "RTIC test" );

        tcp::exit( );
    }
};


