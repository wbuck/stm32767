#![no_std]

use defmt_rtt as _;
use stm32f7xx_hal as _;
use panic_probe as _;

// Same panicking *behavior* as `panic-probe` but doesn't 
// print a panic message this prevents the panic message 
// being printed *twice* when `defmt::panic` is invoked.
#[defmt::panic_handler]
fn panic( ) -> ! {
    cortex_m::asm::udf( )
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0.
pub fn exit( ) -> ! {
    loop {
        cortex_m::asm::bkpt( );
    }
}