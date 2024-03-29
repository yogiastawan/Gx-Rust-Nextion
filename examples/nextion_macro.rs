#![no_std]
#![no_main]

use advance_nextion::{ FileChooser, VideoPlayer};
use gx_rust_nextion::{components::{
    nextion_object_display::Button, objects::TouchHandler,
}, nextion::Nextion};

use gx_rust_nextion::components::NextionVal;

use cortex_m_rt::entry;
use nextion_macro::object_builder;
use panic_halt as _;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial::{Config, Serial},
};
// Create new Nextion object type
#[object_builder(display)]
#[derive(Clone, Copy)]
enum AdvanceNextion {
    #[nextion(txt,val)]
    FileChooser,
    #[nextion(val)]
    VideoPlayer,
}

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
    let mut nex = Nextion::new(serial);

    let mut file_chooser = FileChooser::bind(&mut nex, 0, 0, "name");
    let mut video_player = VideoPlayer::bind(&mut nex, 0, 0, "name");

    let mut button0 = Button::bind(&mut nex, 0, 0, "name");
    let mut b = &mut || {
        file_chooser.set_value(6);
    };
    let mut c = &mut || {};
    button0.set_on_click(b);

    let mut a = Button::bind(&mut nex, 0, 0, "name");
    button0.set_on_release(&mut || {
        video_player.call_on_click();
    });

    let mut v = VideoPlayer::bind(&mut nex, 0, 0, "name");
    let mut f = FileChooser::bind(&mut nex, 1, 0, "name");
    v.set_on_click(&mut || {
        panic!("HUUHUH");
    });

    f.set_value(100);
    // v.set_on_release(&mut||{});
    loop {}
}