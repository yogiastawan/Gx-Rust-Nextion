use crate::nextion::{ComError, IntoU8, Nextion, NextionCom};

use super::{BaseInfo, NextionBaseObj, ObjInfo};

pub enum TextScrollDirection {
    LeftToRigth,
    RightToLeft,
    UpToBottom,
    BottomToUp,
}

impl IntoU8 for TextScrollDirection {
    fn into_u8(self) -> u8 {
        self as u8
    }
}

impl TextScrollDirection {
    pub fn from_u8(data: u8) -> Result<Self, ComError> {
        match data {
            0 => Ok(Self::LeftToRigth),
            1 => Ok(Self::RightToLeft),
            2 => Ok(Self::UpToBottom),
            3 => Ok(Self::BottomToUp),
            _ => Err(ComError::InvalidValue),
        }
    }
}
pub enum TextType {
    Character,
    Password,
}

impl IntoU8 for TextType {
    fn into_u8(self) -> u8 {
        self as u8
    }
}

impl TextType {
    pub fn from_u8(data: u8) -> Result<Self, ComError> {
        match data {
            0 => Ok(Self::Character),
            1 => Ok(Self::Password),
            _ => Err(ComError::InvalidValue),
        }
    }
}
pub enum NumberFormat {
    Decimal,
    Currency,
    Hex,
}

impl IntoU8 for NumberFormat {
    fn into_u8(self) -> u8 {
        self as u8
    }
}

impl NumberFormat {
    pub fn from_u8(data: u8) -> Result<Self, ComError> {
        match data {
            0 => Ok(Self::Decimal),
            1 => Ok(Self::Currency),
            2 => Ok(Self::Hex),
            _ => Err(ComError::InvalidValue),
        }
    }
}

pub enum TextHorizontalAlignment {
    Left,
    Center,
    Right,
}

impl TextHorizontalAlignment {
    pub fn from_u8(data: u8) -> Result<Self, ComError> {
        match data {
            0 => Ok(Self::Left),
            1 => Ok(Self::Center),
            2 => Ok(Self::Right),
            _ => Err(ComError::InvalidValue),
        }
    }
}

pub enum TextVerticalAlignment {
    Top,
    Middle,
    Bottom,
}

impl TextVerticalAlignment {
    pub fn from_u8(data: u8) -> Result<Self, ComError> {
        match data {
            0 => Ok(Self::Top),
            1 => Ok(Self::Middle),
            2 => Ok(Self::Bottom),
            _ => Err(ComError::InvalidValue),
        }
    }
}

impl IntoU8 for TextHorizontalAlignment {
    fn into_u8(self) -> u8 {
        self as u8
    }
}

impl IntoU8 for TextVerticalAlignment {
    fn into_u8(self) -> u8 {
        self as u8
    }
}

pub struct NextionObject<'l, USART> {
    base: NextionBaseObj<'l>,
    device: *mut Nextion<USART>,
}

impl<'l, USART> NextionCom<USART> for NextionObject<'l, USART> where
    USART: embedded_hal::serial::Read<u8> + embedded_hal::blocking::serial::Write<u8>
{
}

impl<'l, USART> NextionObject<'l, USART>
where
    USART: embedded_hal::serial::Read<u8> + embedded_hal::blocking::serial::Write<u8>,
{
    pub fn bind(device: &mut Nextion<USART>, pid: u8, cid: u8, name: &'l str) -> Self {
        NextionObject {
            base: NextionBaseObj::new(pid, cid, name),
            device,
        }
    }
}

impl<'l, USART> BaseInfo for NextionObject<'l, USART> {
    fn get_page_id(&self) -> u8 {
        self.base.pid
    }

    fn get_component_id(&self) -> u8 {
        self.base.cid
    }

    fn get_component_name(&self) -> &str {
        self.base.name
    }
}

impl<'l, USART> ObjInfo<USART> for NextionObject<'l, USART>
where
    USART: embedded_hal::serial::Read<u8> + embedded_hal::blocking::serial::Write<u8>,
{
    fn get_device(&mut self) -> &mut Nextion<USART> {
        unsafe {
            {
                let this = self.device.as_mut();
                match this {
                    Some(val) => val,
                    None => panic!("Device is NULL"),
                }
            }
        }
    }
}

pub trait TouchHandler<'l> {
    fn set_on_click(&mut self, handler: &'l mut dyn FnMut());
    fn set_on_release(&mut self, handler: &'l mut dyn FnMut());
    fn call_on_click(&mut self);
    fn call_on_release(&mut self);
}

pub struct NextionObjectDisplay<'l, USART> {
    base: NextionBaseObj<'l>,
    device: *mut Nextion<USART>,
    on_click: Option<&'l mut dyn FnMut()>,
    on_release: Option<&'l mut dyn FnMut()>,
}

impl<'l, USART> NextionObjectDisplay<'l, USART>
where
    USART: embedded_hal::serial::Read<u8> + embedded_hal::blocking::serial::Write<u8>,
{
    pub fn bind(device: &mut Nextion<USART>, pid: u8, cid: u8, name: &'l str) -> Self {
        NextionObjectDisplay {
            base: NextionBaseObj::new(pid, cid, name),
            device,
            on_click: None,
            on_release: None,
        }
    }
}

impl<'l, USART> TouchHandler<'l> for NextionObjectDisplay<'l, USART>
where
    USART: embedded_hal::serial::Read<u8> + embedded_hal::blocking::serial::Write<u8>,
{
    fn set_on_click(&mut self, handler: &'l mut dyn FnMut()) {
        self.on_click = Some(handler);
    }

    fn set_on_release(&mut self, handler: &'l mut dyn FnMut()) {
        self.on_release = Some(handler);
    }

    fn call_on_click(&mut self) {
        match &mut self.on_click {
            Some(x) => {
                (x)();
            }
            None => {}
        };
    }

    fn call_on_release(&mut self) {
        match &mut self.on_click {
            Some(x) => {
                (x)();
            }
            None => {}
        };
    }
}

impl<'l, USART> NextionCom<USART> for NextionObjectDisplay<'l, USART> where
    USART: embedded_hal::serial::Read<u8> + embedded_hal::blocking::serial::Write<u8>
{
}

impl<'l, USART> BaseInfo for NextionObjectDisplay<'l, USART>
where
    USART: embedded_hal::serial::Read<u8> + embedded_hal::blocking::serial::Write<u8>,
{
    fn get_page_id(&self) -> u8 {
        self.base.pid
    }

    fn get_component_id(&self) -> u8 {
        self.base.cid
    }

    fn get_component_name(&self) -> &str {
        self.base.name
    }
}

impl<'l, USART> ObjInfo<USART> for NextionObjectDisplay<'l, USART>
where
    USART: embedded_hal::serial::Read<u8> + embedded_hal::blocking::serial::Write<u8>,
{
    fn get_device(&mut self) -> &mut Nextion<USART> {
        unsafe {
            {
                let this = self.device.as_mut();
                match this {
                    Some(val) => val,
                    None => panic!("Device is NULL"),
                }
            }
        }
    }
}
