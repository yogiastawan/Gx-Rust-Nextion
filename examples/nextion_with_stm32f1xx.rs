#![no_std]
#![no_main]

use cortex_m_rt::entry;
use gx_rust_nextion::components::nextion_object_display::NextionText;
use gx_rust_nextion::components::{BaseInfo, NextionBaseObj};
use gx_rust_nextion::nextion::{Nextion, NextionCom};
use nextion_macro::object_builder;
use panic_halt as _;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial::{Config, Serial},
};

use gx_rust_nextion::components::component_trait::{NextionFont, NextionTim, NextionTxt};

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let p = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Prepare the alternate function I/O registers
    let mut afio = p.AFIO.constrain();

    // Prepare the GPIOB peripheral
    let mut gpiob = p.GPIOB.split();
    // USART3
    // Configure pb10 as a push_pull output, this will be the tx pin
    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    // Take ownership over pb11
    let rx = gpiob.pb11;

    // Set up the usart device. Take ownership over the USART register and tx/rx pins. The rest of
    // the registers are used to enable and configure the device.
    let mut serial = Serial::new(
        p.USART3,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(9600.bps()),
        &clocks,
    );
    let mut nex = Nextion::new(&mut serial);

    let mut a = NextionText::bind(&mut nex, 0, 0, "name");
    a.set_txt("sss");

    a.set_font(1);

    loop {}
}
