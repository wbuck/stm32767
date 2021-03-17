#![no_std]
#![no_main]
#![allow( non_snake_case )]

use stm32f7xx_hal as stm32;
use stm32::{
    device::RCC, 
    gpio::GpioExt, 
    rcc::RccExt, 
    time::*
};

use tcp as _;
use defmt as log;
use eth::phy::PHY_REG_ID2;
use eth::smi::Smi;


#[rtic::app( device = stm32f7xx_hal::device, peripherals = true )]
const APP: ( ) = {

    #[init]
    fn init( cx: init::Context ) {
        // Cortex-M peripherals.
        let _core = cx.core;

        // Device specific peripherals.
        let device: stm32::device::Peripherals = cx.device;

        let rcc = device.RCC.constrain( );
        let clocks = rcc.cfgr
            .sysclk( 216u32.mhz( ) )
            .hclk( 216u32.mhz( ) )
            .pclk1( 54u32.mhz( ) )
            .pclk2( 108u32.mhz( ) )
            .timclk1( 108u32.mhz( ) )
            .timclk2( 216u32.mhz( ) )
            .freeze( );

        let gpioa = device.GPIOA.split( );
        let gpiob = device.GPIOB.split( );
        let gpioc = device.GPIOC.split( );
        let gpiog = device.GPIOG.split( );

        let _rmii_ref_clock = gpioa.pa1.into_alternate_af11( );
        let _rmii_mdio = gpioa.pa2.into_alternate_af11( );
        let _rmii_mdc = gpioc.pc1.into_alternate_af11( );
        let _rmii_rx_data_valid = gpioa.pa7.into_alternate_af11( );
        let _rmii_rxd0 = gpioc.pc4.into_alternate_af11( );
        let _rmii_rxd1 = gpioc.pc5.into_alternate_af11( );
        let _rmii_tx_enable = gpiog.pg11.into_alternate_af11( );
        let _rmii_txd0 = gpiog.pg13.into_alternate_af11( );
        let _rmii_pb13 = gpiob.pb13.into_alternate_af11( );

        // Select the RMII ethernet phy interface. This 
        // must be set before enabling the peripheral.
        device.SYSCFG.pmc.modify( | _, w | 
            w.mii_rmii_sel( ).set_bit( ) );

        let rcc = unsafe { &*RCC::ptr( ) };

        rcc.ahb1enr.modify( | _, w | w
            .ethmacen( ).enabled( )
            .ethmacrxen( ).enabled( )
            .ethmactxen( ).enabled( ) );

        let mac = device.ETHERNET_MAC;
        let smi = Smi::new( &mac.macmiiar, &mac.macmiidr, clocks );

        log::debug!( "Querying PHY" );
        let id = smi.read( 0, PHY_REG_ID2 );
        log::debug!( "ID: {=u16:X}", id );

        log::debug!( "Setting PHY power down" );
        smi.write( 0, eth::phy::PHY_REG_BSR, eth::phy::PHY_REG_BCR_POWERDN );
        log::debug!( "PHY power down state set" );


        log::debug!( "Setting PHY auto negotiation" );
        smi.write( 0, eth::phy::PHY_REG_BSR, 
            eth::phy::PHY_REG_BCR_POWERDN | eth::phy::PHY_REG_BCR_AN );
        log::debug!( "PHY auto negotiation state set" );

        log::debug!( "Querying PHY BCR" );
        let status = smi.read( 0, eth::phy::PHY_REG_BCR, );
        log::debug!( "ID: {=u16:b}", status );


        log::debug!( "Querying PHY BSR" );
        let status = smi.read( 0, eth::phy::PHY_REG_BSR, );
        log::debug!( "ID: {=u16:b}", status );

        tcp::exit( );
    }

    #[idle]
    fn idle( _cx: idle::Context ) -> ! {
        loop { }
    }

    #[task( binds = ETH )]
    fn eth_handler( _cx: eth_handler::Context ) {
        // Clear interrupt flags
        //let p = unsafe { pac::Peripherals::steal( ) };
        //stm32_eth::eth_interrupt_handler( &p.ETHERNET_DMA );
    }
};


