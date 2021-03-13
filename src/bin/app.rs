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

        // Select the RMII ethernet phy interface. This 
        // must be set before enabling the peripheral.
        device.SYSCFG.pmc.modify( | _, w | 
            w.mii_rmii_sel( ).set_bit( ) );

        
        /*
            Pins required for PHY communication:
            PA1  (AF11) - RMII Reference Clock
            PA2  (AF11) - RMII MDIO
            PC1  (AF11) - RMII MDC
            PA7  (AF11) - RMII RX Data Valid
            PC4  (AF11) - RMII RXD0
            PC5  (AF11) - RMII RXD1
            PG11 (AF11) - RMII TX Enable
            PG13 (AF11) - RXII TXD0
            PB13 (AF11) - RMII TXD1
        */

        tcp::exit( );
    }
};


