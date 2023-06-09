use core::fmt::{Display, Write};

use crate::nextion::{ComError, Nextion, NextionCom};

use heapless::String;
use nextion_macro::object_builder;
use num_traits::{NumCast, PrimInt};

#[object_builder]
#[derive(Clone, Copy)]
pub enum NextionObject {
    #[nextion(tim = timer, tim, en)]
    Timer,
    #[nextion(val)]
    Variable,
}

#[object_builder(display)]
#[derive(Clone, Copy)]
pub enum NextionObjectDisplay {
    #[nextion(txt, font, bco, pco, pic, picc, xcen, ycen, isbr, pw)]
    Text,
    #[nextion(txt, bco, pco, pic, picc, xcen, ycen, isbr, dir, dis, dis = scroll_text, tim, tim = scroll_text, en, spax, spay)]
    ScrollingText,
    #[nextion(
        val, bco, pco, font, pic, picc, xcen, ycen, isbr, format, spax, spay, lenth
    )]
    Number,
    #[nextion(
        txt, font, bco, bco2, pco, pco2, pic, pic2, picc, picc2, xcen, ycen, isbr
    )]
    Button,
    #[nextion(val, bco, pco, bpic, ppic)]
    ProgressBar,
    #[nextion(pic)]
    Picture,
    #[nextion(picc)]
    Crop,
    Hotspot,
    #[nextion(val, bco, pic, picc, pco, wid)]
    Gauge,
    #[nextion(bco, pic, picc, gdw, gdh, dis, dis = wave_form, pco = wave_form, add, cle)]
    WaveForm,
    #[nextion(val=u16, bco, pic, picc, bco1, picc1, pco, pic2, maxval, minval)] //check again
    Slider,
    #[nextion(
        val=bool, font, bco, pic, picc, bco2, pic2, picc2, pco, pco2, xcen, ycen, txt, isbr
    )]
    DualStateButton,
    #[nextion(val = bool, bco, pco)]
    Checkbox,
    #[nextion(val = bool, bco, pco)]
    Radio,
    #[nextion(val, font, bco, pco, pic, picc, xcen, ycen, isbr, spax, spay, ws0, ws1)]
    XFloat,
    TouchCap,
    #[nextion(txt, dis, bco, pco)]
    QRCode,
}

// pub trait ObjectTypes {}
pub trait ObjInfo<USART>
where
    USART: embedded_hal::serial::Read<u8> + embedded_hal::blocking::serial::Write<u8>,
{
    fn get_device(&mut self) -> &mut Nextion<USART>;
}

pub trait NextionVal<USART>: NextionCom<USART> + ObjInfo<USART> + BaseInfo
where
    USART: embedded_hal::serial::Read<u8> + embedded_hal::blocking::serial::Write<u8>,
{
    type ValueType: PrimInt + Display;

    fn set_value(&mut self, value: Self::ValueType) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<32>::new();
        match write!(&mut cmd, "{}.val={}", name, value) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };
        self.send_cmd(cmd.as_bytes())
    }

    fn get_value(&mut self) -> Result<Self::ValueType, ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<32>::new();
        match write!(&mut cmd, "get {}.val", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };
        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => {}
            Err(err) => return Err(err),
        };

        let mut buff: Self::ValueType = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number(&mut buff) {
            Ok(_) => {}
            Err(err) => return Err(err),
        };

        Ok(buff)
    }
}

pub trait NextionAct<USART>: NextionVal<USART, ValueType = u8>
where
    USART: embedded_hal::serial::Read<u8> + embedded_hal::blocking::serial::Write<u8>,
{
    fn set_active(&mut self, value: bool) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let val = match value {
            true => 1u8,
            false => 0u8,
        };
        self.set_value(val)
    }

    fn get_active(&mut self) -> Result<bool, ComError>
    where
        Self: Sized,
    {
        let res = match self.get_value() {
            Ok(x) => x,
            Err(err) => return Err(err),
        };

        let val = match res {
            1 => true,
            0 => false,
            _ => return Err(ComError::InvalidValue),
        };

        Ok(val)
    }
}

pub trait BaseInfo {
    /// Get page id.
    fn get_page_id(&self) -> u8;
    /// Get component id.
    fn get_component_id(&self) -> u8;
    /// Get component name.
    fn get_component_name(&self) -> &str;
}

pub struct NextionBaseObj<'life> {
    pub pid: u8,
    pub cid: u8,
    pub name: &'life str,
}

impl<'l> NextionBaseObj<'l> {
    /// Creates a new [`NextionBaseObj`].
    pub fn new(pid: u8, cid: u8, name: &'l str) -> Self {
        NextionBaseObj { pid, cid, name }
    }
}

impl BaseInfo for NextionBaseObj<'_> {
    fn get_page_id(&self) -> u8 {
        self.pid
    }

    fn get_component_id(&self) -> u8 {
        self.cid
    }

    fn get_component_name(&self) -> &str {
        self.name
    }
}
pub mod component_trait;
pub mod objects;
