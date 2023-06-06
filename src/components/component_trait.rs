use core::fmt::Write;

use heapless::String;
use num_traits::NumCast;

use crate::nextion::{ComError, IntoU8, NextionCom};

use super::{
    objects::{
        NumberFormat, TextHorizontalAlignment, TextScrollDirection, TextType, TextVerticalAlignment,
    },
    BaseInfo, ObjInfo,
};

pub trait NextionTim<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_tim(&mut self, tim: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name(); //max size is 16*u8
        let mut cmd = String::<26>::new();
        match write!(cmd, "{}.val={}", name, tim) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_tim(&mut self) -> Result<u16, ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<24>::new();
        match write!(cmd, "get {}.val", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionEn<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_en(&mut self, en: u8) -> Result<(), ComError>
    where
        Self: Sized,
    {
        if en > 1 {
            return Err(ComError::InvalidDataRange);
        }
        let name = self.get_component_name();
        let mut cmd = String::<23>::new();
        match write!(cmd, "{}.en={}", name, en) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_en(&mut self) -> Result<u8, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<23>::new();
        match write!(cmd, "get {}.en", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u8>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionTxt<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_txt(&mut self, txt: &str) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<265>::new();
        match write!(cmd, "{}.txt={}", name, txt) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_txt(&mut self, str: &mut [u8]) -> Result<u8, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<24>::new();
        match write!(cmd, "get {}.txt", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        self.get_str(str)
    }
}

pub trait NextionBco<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_bco(&mut self, bco: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<26>::new();
        match write!(cmd, "{}.bco={}", name, bco) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_bco(&mut self) -> Result<u16, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<24>::new();
        match write!(cmd, "get {}.bco", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionBco2<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_bco2(&mut self, bco2: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<27>::new();
        match write!(cmd, "{}.bco2={}", name, bco2) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_bco2(&mut self) -> Result<u16, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<24>::new();
        match write!(cmd, "get {}.bco2", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionPic<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_pic(&mut self, img: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<26>::new();
        match write!(cmd, "{}.pic={}", name, img) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_pic(&mut self) -> Result<u16, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<24>::new();
        match write!(cmd, "get {}.pic", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionPic2<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_pic2(&mut self, img: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<27>::new();
        match write!(cmd, "{}.pic2={}", name, img) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_pic2(&mut self) -> Result<u16, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<24>::new();
        match write!(cmd, "get {}.pic2", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionPicc<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_picc(&mut self, img: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<27>::new();
        match write!(cmd, "{}.picc={}", name, img) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_picc(&mut self) -> Result<u16, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<25>::new();
        match write!(cmd, "get {}.picc", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionPicc2<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_picc2(&mut self, img: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<28>::new();
        match write!(cmd, "{}.picc2={}", name, img) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_picc2(&mut self) -> Result<u16, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<25>::new();
        match write!(cmd, "get {}.picc2", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionPco<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_pco(&mut self, pco: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<26>::new();
        match write!(cmd, "{}.pco={}", name, pco) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_pco(&mut self) -> Result<u16, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<24>::new();
        match write!(cmd, "get {}.pco", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionPco2<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_pco2(&mut self, pco2: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<27>::new();
        match write!(cmd, "{}.pco2={}", name, pco2) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_pco2(&mut self) -> Result<u16, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<24>::new();
        match write!(cmd, "get {}.pco2", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionFont<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_font(&mut self, font: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<27>::new();
        match write!(cmd, "{}.font={}", name, font) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_font(&mut self) -> Result<u16, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<25>::new();
        match write!(cmd, "get {}.font", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionXcen<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_xcen(&mut self, xcen: TextHorizontalAlignment) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let xcen = xcen.into_u8();
        let name = self.get_component_name();
        let mut cmd = String::<25>::new();
        match write!(cmd, "{}.xcen={}", name, xcen) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_xcen(&mut self) -> Result<TextHorizontalAlignment, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<25>::new();
        match write!(cmd, "get {}.xcen", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        let val = match self.get_number::<u8>(&mut buff) {
            Ok(_) => match TextHorizontalAlignment::from_u8(buff) {
                Ok(x) => x,
                Err(err) => return Err(err),
            },
            Err(err) => return Err(err),
        };
        Ok(val)
    }
}

pub trait NextionYcen<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_ycen(&mut self, ycen: TextVerticalAlignment) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<25>::new();
        match write!(cmd, "{}.ycen={}", name, ycen.into_u8()) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_ycen(&mut self) -> Result<TextVerticalAlignment, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<25>::new();
        match write!(cmd, "get {}.ycen", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        let val = match self.get_number::<u8>(&mut buff) {
            Ok(_) => match TextVerticalAlignment::from_u8(buff) {
                Ok(x) => x,
                Err(err) => return Err(err),
            },
            Err(err) => return Err(err),
        };
        Ok(val)
    }
}

pub trait NextionIsbr<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_isbr(&mut self, isbr: bool) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let isbr = match isbr {
            true => 1u8,
            false => 0u8,
        };
        let name = self.get_component_name();
        let mut cmd = String::<25>::new();
        match write!(cmd, "{}.isbr={}", name, isbr) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_isbr(&mut self) -> Result<bool, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<25>::new();
        match write!(cmd, "get {}.isbr", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u8>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        match buff {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ComError::InvalidValue),
        }
    }
}

pub trait NextionWid<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_wid(&mut self, wid: u8) -> Result<(), ComError>
    where
        Self: Sized,
    {
        if wid > 20 {
            return Err(ComError::InvalidDataRange);
        }
        let name = self.get_component_name();
        let mut cmd = String::<24>::new();
        match write!(cmd, "{}.wid={}", name, wid) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_wid(&mut self) -> Result<u8, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<24>::new();
        match write!(cmd, "get {}.wid", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u8>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionFormat<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_fromat(&mut self, format: NumberFormat) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<27>::new();
        match write!(cmd, "{}.fromat={}", name, format.into_u8()) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_fromat(&mut self) -> Result<NumberFormat, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<27>::new();
        match write!(cmd, "get {}.format", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u8>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let val = match NumberFormat::from_u8(buff) {
            Ok(x) => x,
            Err(err) => return Err(err),
        };
        Ok(val)
    }
}

pub trait NextionSpax<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_spax(&mut self, spax: u8) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<25>::new();
        match write!(cmd, "{}.spax={}", name, spax) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_spax(&mut self) -> Result<u8, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<25>::new();
        match write!(cmd, "get {}.spax", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u8>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionSpay<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_spay(&mut self, spay: u8) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<25>::new();
        match write!(cmd, "{}.spay={}", name, spay) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_spay(&mut self) -> Result<u8, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<25>::new();
        match write!(cmd, "get {}.spay", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u8>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionLenth<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_lenth(&mut self, lenth: u8) -> Result<(), ComError>
    where
        Self: Sized,
    {
        if lenth > 15 {
            return Err(ComError::InvalidDataRange);
        }
        let name = self.get_component_name();
        let mut cmd = String::<26>::new();
        match write!(cmd, "{}.lenth={}", name, lenth) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_lenth(&mut self) -> Result<u8, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<25>::new();
        match write!(cmd, "get {}.lenth", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u8>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionBpic<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_bpic(&mut self, img: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<27>::new();
        match write!(cmd, "{}.bpic={}", name, img) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_bpic(&mut self) -> Result<u16, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<25>::new();
        match write!(cmd, "get {}.bpic", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionPpic<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_ppic(&mut self, img: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<27>::new();
        match write!(cmd, "{}.ppic={}", name, img) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_ppic(&mut self) -> Result<u16, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<25>::new();
        match write!(cmd, "get {}.ppic", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionDis<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_dis(&mut self, dis: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<24>::new();
        match write!(cmd, "{}.dis={}", name, dis) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_dis(&mut self) -> Result<u16, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<24>::new();
        match write!(cmd, "get {}.dis", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionDir<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_dir(&mut self, dir: TextScrollDirection) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<24>::new();
        match write!(cmd, "{}.dir={}", name, dir.into_u8()) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_dir(&mut self) -> Result<TextScrollDirection, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<24>::new();
        match write!(cmd, "get {}.dir", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u8>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        let val = match TextScrollDirection::from_u8(buff) {
            Ok(x) => x,
            Err(err) => return Err(err),
        };
        Ok(val)
    }
}

pub trait NextionBco1<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_bco1(&mut self, bco1: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<27>::new();
        match write!(cmd, "{}.bco1={}", name, bco1) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_bco1(&mut self) -> Result<u16, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<24>::new();
        match write!(cmd, "get {}.bco1", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionPicc1<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_picc1(&mut self, img: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<28>::new();
        match write!(cmd, "{}.picc1={}", name, img) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_picc1(&mut self) -> Result<u16, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<25>::new();
        match write!(cmd, "get {}.picc1", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionMaxval<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_maxval(&mut self, val: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<29>::new();
        match write!(cmd, "{}.maxval={}", name, val) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_maxval(&mut self) -> Result<u16, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<26>::new();
        match write!(cmd, "get {}.maxval", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionMinval<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_minval(&mut self, val: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<29>::new();
        match write!(cmd, "{}.minval={}", name, val) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_minval(&mut self) -> Result<u16, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<26>::new();
        match write!(cmd, "get {}.minval", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionPw<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_pw(&mut self, pw: TextType) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<23>::new();
        match write!(cmd, "{}.pw={}", name, pw.into_u8()) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_pw(&mut self) -> Result<TextType, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<23>::new();
        match write!(cmd, "get {}.pw", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u8>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        let val = match TextType::from_u8(buff) {
            Ok(x) => x,
            Err(err) => return Err(err),
        };
        Ok(val)
    }
}

pub trait NextionGdc<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_gdc(&mut self, gdc: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<26>::new();
        match write!(cmd, "{}.gdc={}", name, gdc) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_gdc(&mut self) -> Result<u16, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<24>::new();
        match write!(cmd, "get {}.gdc", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionGdw<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_gdw(&mut self, gdw: u32) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<31>::new();
        match write!(cmd, "{}.gdw={}", name, gdw) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_gdw(&mut self) -> Result<u32, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<24>::new();
        match write!(cmd, "get {}.gdw", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u32>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionGdh<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_gdh(&mut self, gdh: u32) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<31>::new();
        match write!(cmd, "{}.gdw={}", name, gdh) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_gdh(&mut self) -> Result<u32, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let name = self.get_component_name();
        let mut cmd = String::<24>::new();
        match write!(cmd, "get {}.gdh", name) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u32>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionAdd<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn add_data(&mut self, channel: u8, val: u8) -> Result<(), ComError>
    where
        Self: Sized,
    {
        if channel > 4 {
            return Err(ComError::InvalidDataRange);
        }
        let id = self.get_component_id();
        let mut cmd = String::<15>::new();
        match write!(cmd, "add {},{},{}", id, channel, val) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }
}

pub trait NextionCle<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn cle(&mut self, channel: u8) -> Result<(), ComError>
    where
        Self: Sized,
    {
        if channel > 4 {
            return Err(ComError::InvalidDataRange);
        }
        let id = self.get_component_id();
        let mut cmd = String::<11>::new();
        match write!(cmd, "cle {},{}", id, channel) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }
}

pub trait NextionScrollTextTim<USART>: NextionTim<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_scroll_text_tim(&mut self, tim: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        if tim < 80 {
            return Err(ComError::InvalidDataRange);
        }

        self.set_tim(tim)
    }

    fn get_scroll_text_tim(&mut self) -> Result<u16, ComError>
    where
        Self: Sized,
    {
        self.get_tim()
    }
}

pub trait NextionTimerTim<USART>: NextionTim<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_timer_tim(&mut self, tim: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        if tim < 50 {
            return Err(ComError::InvalidDataRange);
        }
        self.set_tim(tim)
    }

    fn get_timer_tim(&mut self) -> Result<u16, ComError>
    where
        Self: Sized,
    {
        self.get_tim()
    }
}

pub trait NextionScrollTextDis<USART>: NextionDis<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_scroll_text_dis(&mut self, dis: u8) -> Result<(), ComError>
    where
        Self: Sized,
    {
        if dis > 50 || dis < 2 {
            return Err(ComError::InvalidDataRange);
        }
        self.set_dis(dis as u16)
    }

    fn get_scroll_text_dis(&mut self) -> Result<u8, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let buff = match self.get_dis() {
            Ok(x) => x as u8,
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionWaveFormDis<USART>: NextionDis<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_wave_form_dis(&mut self, dis: u8) -> Result<(), ComError>
    where
        Self: Sized,
    {
        if dis > 100 || dis < 10 {
            return Err(ComError::InvalidDataRange);
        }
        self.set_dis(dis as u16)
    }

    fn get_wave_form_dis(&mut self) -> Result<u8, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        let buff = match self.get_dis() {
            Ok(x) => x as u8,
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}

pub trait NextionWaveFormPco<USART>: NextionCom<USART> + BaseInfo + ObjInfo<USART>
where
    USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
{
    fn set_wave_form_pco(&mut self, channel: u8, color: u16) -> Result<(), ComError>
    where
        Self: Sized,
    {
        if channel > 4 {
            return Err(ComError::InvalidDataRange);
        }
        let name = self.get_component_name();
        let mut cmd = String::<29>::new();
        match write!(cmd, "{}.pco{}={}", name, channel, color) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        self.send_cmd(cmd.as_bytes())
    }

    fn get_wave_form_pco(&mut self, channel: u8) -> Result<u16, ComError>
    where
        USART: embedded_hal::blocking::serial::Write<u8> + embedded_hal::serial::Read<u8>,
        Self: Sized,
    {
        if channel > 4 {
            return Err(ComError::InvalidDataRange);
        }
        let name = self.get_component_name();
        let mut cmd = String::<27>::new();
        match write!(cmd, "get {}.pco{}", name, channel) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedCreateCommand),
        };

        match self.send_cmd(cmd.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };

        let mut buff = match NumCast::from(0) {
            Some(x) => x,
            None => return Err(ComError::FailedCreateNumberBuffer),
        };

        match self.get_number::<u16>(&mut buff) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        Ok(buff)
    }
}
