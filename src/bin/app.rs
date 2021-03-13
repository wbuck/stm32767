#![no_std]
#![no_main]
#![allow( non_snake_case )]

use stm32f7xx_hal as hal;
use hal::{
    rcc::RccExt,
    time::*,
};

use tcp as _;
use defmt as log;

#[rtic::app( device = stm32f7xx_hal::device, peripherals = true )]
const APP: ( ) = {

    #[init]
    fn init( cx: init::Context ) {
        // Cortex-M peripherals.
        let _core = cx.core;

        // Device specific peripherals.
        let device: stm32f7xx_hal::device::Peripherals = cx.device;

        let rcc = device.RCC.constrain( );
        let clocks = rcc.cfgr
            .sysclk( 216u32.mhz( ) )
            .hclk( 216u32.mhz( ) )
            .pclk1( 54u32.mhz( ) )
            .pclk2( 108u32.mhz( ) )
            .timclk1( 108u32.mhz( ) )
            .timclk2( 216u32.mhz( ) )
            .freeze( );

        log::info!( "HCLK: {} | SYSCLK: {} | PCLK1: {} | PCLK2: {} | TIMCLK1: {} | TIMCLK2: {}", 
            clocks.hclk( ).0,
            clocks.sysclk( ).0,
            clocks.pclk1( ).0,
            clocks.pclk2( ).0,
            clocks.timclk1( ).0,
            clocks.timclk2( ).0 );

        tcp::exit( );
    }
};


