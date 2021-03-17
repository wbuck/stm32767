use stm32f7xx_hal as stm32;
use stm32::device::ethernet_mac::{MACMIIAR, MACMIIDR, macmiiar::CR_A};
use stm32::rcc::Clocks;

pub struct Smi<'a> {
    macmiiar: &'a MACMIIAR,
    macmiidr: &'a MACMIIDR,
    clocks: Clocks,
}

impl<'a> Smi<'a> {
    pub fn new( macmiiar: &'a MACMIIAR, macmiidr: &'a MACMIIDR, clocks: Clocks ) -> Self {
        Self { macmiiar, macmiidr, clocks }
    }

    pub fn read( &self, phy: u8, reg: u8 ) -> u16 {
        self.macmiiar.modify( | _, w | w
            .pa( ).bits( phy ) 
            .mr( ).bits( reg )
            .cr( ).variant( self.divider( ) )
            .mw( ).read( )
            .mb( ).busy( ) );

        self.wait_ready( );
        self.macmiidr.read( ).bits( ) as u16
    }

    pub fn write( &self, phy: u8, reg: u8, data: u16 ) {
        self.clocks.hclk( );
        self.macmiidr.write( | w | w.md( ).bits( data ) );
        self.macmiiar.modify( | _, w | w
            .pa( ).bits( phy ) 
            .mr( ).bits( reg )
            .cr( ).variant( self.divider( ) )
            .mw( ).write( )
            .mb( ).busy( ) );

        self.wait_ready( );
    }

    /// Helper: `read()` and `write()` by OR-ing the current value of
    /// the register `reg` with `mask`.
    pub fn set_bits( &self, phy: u8, reg: u8, mask: u16 ) {
        let value = self.read( phy, reg );
        self.write( phy, reg, value | mask );
    }

    fn wait_ready( &self ) {
        while self.macmiiar.read( ).mb( ).is_busy( ) { }
    }

    fn divider( &self ) -> CR_A {
        match self.clocks.hclk( ).0 / 1_000_000 {
            20..=34 => CR_A::CR_20_35,
            35..=59 => CR_A::CR_35_60,
            60..=99 => CR_A::CR_60_100,
            100..=149 => CR_A::CR_100_150,
            _ => CR_A::CR_150_168,
        }
    }
}

