use core::cell::RefCell;

use embedded_hal::blocking::serial as blocking;
use num_traits::{NumCast, PrimInt};

use crate::components::ObjInfo;
// #[repr(u8)]
#[derive(PartialEq)]
pub enum NextionCmd {
    CmdEnd = 0xFF,
    CmdFinishOk = 0x01,
    CmdEventLaunched = 0x88,
    CmdEventUpgraded = 0x89,
    CmdEventTouchHead = 0x65,
    CmdEventPositionHead = 0x67,
    CmdEventSleepPositionHead = 0x68,
    CmdCurrentPageIdHead = 0x66,
    CmdStringHead = 0x70,
    CmdNumberHead = 0x71,
    CmdInvalidCmd = 0x00,
    CmdInvalidComponentId = 0x02,
    CmdInvalidPageId = 0x03,
    CmdInvalidPictureId = 0x04,
    CmdInvalidFontId = 0x05,
    CmdInvalidBaud = 0x11,
    CmdInvalidVariable = 0x1A,
    CmdInvalidOperation = 0x1B,
}

pub enum ComError {
    FailedCreateCommand,
    PeripheralNotFound,
    FailedWrite,
    FailedRead,
    FailedSendCmd,
    FailedCreateNumberBuffer,
    IvalidGetDataString,
    IvalidGetDataNumber,
    InvalidDataRange,
    InvalidValue,
}

pub trait IntoU8 {
    fn into_u8(self) -> u8;
}

impl IntoU8 for NextionCmd {
    fn into_u8(self) -> u8 {
        self as u8
        // match self {
        //     NextionCmd::CmdEnd => 0xFF,
        //     NextionCmd::CmdFinishOk => 0x01,
        //     NextionCmd::CmdEventLaunched => 0x88,
        //     NextionCmd::CmdEventUpgraded => 0x89,
        //     NextionCmd::CmdEventTouchHead => 0x65,
        //     NextionCmd::CmdEventPositionHead => 0x67,
        //     NextionCmd::CmdEventSleepPositionHead => 0x68,
        //     NextionCmd::CmdCurrentPageIdHead => 0x66,
        //     NextionCmd::CmdStringHead => 0x70,
        //     NextionCmd::CmdNumberHead => 0x71,
        //     NextionCmd::CmdInvalidCmd => 0x00,
        //     NextionCmd::CmdInvalidComponentId => 0x02,
        //     NextionCmd::CmdInvalidPageId => 0x03,
        //     NextionCmd::CmdInvalidPictureId => 0x04,
        //     NextionCmd::CmdInvalidFontId => 0x05,
        //     NextionCmd::CmdInvalidBaud => 0x11,
        //     NextionCmd::CmdInvalidVariable => 0x1A,
        //     NextionCmd::CmdInvalidOperation => 0x1B,
        // }
    }
}

pub trait NextionCom<USART>: ObjInfo<USART>
where
    USART: embedded_hal::serial::Read<u8> + blocking::Write<u8>,
{
    fn send_cmd(&mut self, cmd: &[u8]) -> Result<(), ComError>
    where
        Self: Sized,
    {
        let device = self.get_device();
        match device.get_peripheral().borrow_mut().bwrite_all(cmd) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedWrite),
        };
        let cmd: [u8; 3] = [0xFF, 0xFF, 0xFF];
        match device.get_peripheral().borrow_mut().bwrite_all(&cmd) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedWrite),
        };
        match device.get_peripheral().borrow_mut().bflush() {
            Ok(_) => (),
            Err(_) => return Err(ComError::FailedSendCmd),
        };
        Ok(())
    }

    fn get_str(&mut self, buff: &mut [u8]) -> Result<u8, ComError>
    where
        Self: Sized,
    {
        let device = self.get_device();
        //Get string head
        let mut len = match device.get_peripheral().borrow_mut().read() {
            Ok(word) => {
                if NextionCmd::CmdStringHead.into_u8() == word {
                    0u8
                } else {
                    return Err(ComError::IvalidGetDataString);
                }
            }
            Err(_) => return Err(ComError::FailedRead),
        };
        let mut end = 0u8;
        for buf in buff {
            *buf = match device.get_peripheral().borrow_mut().read() {
                Ok(word) => {
                    if NextionCmd::CmdEnd.into_u8() == word {
                        end += 1;
                        if end >= 3 {
                            break;
                        }
                        continue;
                    }
                    len += 1;
                    word
                }
                Err(_) => return Err(ComError::FailedRead),
            }
        }

        Ok(len)
    }

    fn get_number<T>(&mut self, buff: &mut T) -> Result<(), ComError>
    where
        Self: Sized,
        T: PrimInt,
    {
        let device = self.get_device();
        //Get number head
        match device.get_peripheral().borrow_mut().read() {
            Ok(word) => {
                if NextionCmd::CmdNumberHead.into_u8() != word {
                    return Err(ComError::IvalidGetDataNumber);
                }
            }
            Err(_) => return Err(ComError::FailedRead),
        };

        let mut buffer = [0u8; 7];
        for buf in &mut buffer {
            *buf = match device.get_peripheral().borrow_mut().read() {
                Ok(val) => val,
                Err(_) => return Err(ComError::FailedRead),
            }
        }

        if (NextionCmd::CmdEnd.into_u8() == buffer[4])
            && (NextionCmd::CmdEnd.into_u8() == buffer[5])
            && (NextionCmd::CmdEnd.into_u8() == buffer[6])
        {
            let val = (buffer[3] as i32) << 24
                | (buffer[2] as i32) << 16
                | (buffer[1] as i32) << 8
                | (buffer[3] as i32);
            *buff = match NumCast::from(val) {
                Some(x) => x,
                None => return Err(ComError::IvalidGetDataNumber),
            };
        } else {
            return Err(ComError::IvalidGetDataNumber);
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Nextion<USART> {
    usart: RefCell<USART>,
}

impl<USART> Nextion<USART>
where
    USART: embedded_hal::serial::Read<u8> + blocking::Write<u8>,
{
    /// Creates a new [`Nextion<X>`].
    pub fn new(peripheral: USART) -> Self {
        Nextion {
            usart: RefCell::new(peripheral),
        }
    }

    /// Returns a reference to the get peripheral of this [`Nextion<X>`].
    pub fn get_peripheral(&mut self) -> &RefCell<USART> {
        {
            &self.usart
        }
    }
}

impl<USART> Nextion<USART>
where
    USART: embedded_hal::serial::Read<u8> + blocking::Write<u8>,
{
    pub fn send_cmd(&mut self, cmd: &[u8]) -> Result<(), ComError> {
        match self.get_peripheral().borrow_mut().bwrite_all(cmd) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedWrite),
        };
        let cmd: [u8; 3] = [0xFF, 0xFF, 0xFF];
        match self.get_peripheral().borrow_mut().bwrite_all(&cmd) {
            Ok(_) => {}
            Err(_) => return Err(ComError::FailedWrite),
        };
        match self.get_peripheral().borrow_mut().bflush() {
            Ok(_) => (),
            Err(_) => return Err(ComError::FailedSendCmd),
        };
        Ok(())
    }

    pub fn get_str(&mut self, buff: &mut [u8]) -> Result<u16, ComError> {
        //Get string head
        let mut len = match self.get_peripheral().borrow_mut().read() {
            Ok(word) => {
                if NextionCmd::CmdStringHead.into_u8() == word {
                    0u16
                } else {
                    return Err(ComError::IvalidGetDataString);
                }
            }
            Err(_) => return Err(ComError::FailedRead),
        };
        let mut end = 0u8;
        for buf in buff {
            *buf = match self.get_peripheral().borrow_mut().read() {
                Ok(word) => {
                    if NextionCmd::CmdEnd.into_u8() == word {
                        end += 1;
                        if end >= 3 {
                            break;
                        }
                        continue;
                    }
                    len += 1;
                    word
                }
                Err(_) => return Err(ComError::FailedRead),
            }
        }

        Ok(len)
    }

    pub fn get_number<T>(&mut self, buff: &mut T) -> Result<(), ComError>
    where
        T: PrimInt,
    {
        //Get number head
        match self.get_peripheral().borrow_mut().read() {
            Ok(word) => {
                if NextionCmd::CmdNumberHead.into_u8() != word {
                    return Err(ComError::IvalidGetDataNumber);
                }
            }
            Err(_) => return Err(ComError::FailedRead),
        };

        let mut buffer = [0u8; 7];
        for buf in &mut buffer {
            *buf = match self.get_peripheral().borrow_mut().read() {
                Ok(val) => val,
                Err(_) => return Err(ComError::FailedRead),
            }
        }

        if (NextionCmd::CmdEnd.into_u8() == buffer[4])
            && (NextionCmd::CmdEnd.into_u8() == buffer[5])
            && (NextionCmd::CmdEnd.into_u8() == buffer[6])
        {
            let val = (buffer[3] as i32) << 24
                | (buffer[2] as i32) << 16
                | (buffer[1] as i32) << 8
                | (buffer[3] as i32);
            *buff = match NumCast::from(val) {
                Some(x) => x,
                None => return Err(ComError::IvalidGetDataNumber),
            };
        } else {
            return Err(ComError::IvalidGetDataNumber);
        }
        Ok(())
    }
}
