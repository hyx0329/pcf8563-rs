use crate::{Pcf8563, Error, I2c};
use bit_field::BitField;

impl<I2C: I2c> Pcf8563<I2C> {
    pub fn timer_interrupt_enabled(&mut self) -> Result<bool, Error> {
        Ok(self.read_u8(0x01)?.get_bit(0))
    }

    pub fn enable_timer_interrupt(&mut self) -> Result<(), Error> {
        self.write_bit(0x01, 0, true)
    }

    pub fn disable_timer_interrupt(&mut self) -> Result<(), Error> {
        self.write_bit(0x01, 0, false)
    }

    pub fn alarm_interrupt_enabled(&mut self) -> Result<bool, Error> {
        Ok(self.read_u8(0x01)?.get_bit(1))
    }

    pub fn enable_alarm_interrupt(&mut self) -> Result<(), Error> {
        self.write_bit(0x01, 1, true)
    }

    pub fn disable_alarm_interrupt(&mut self) -> Result<(), Error> {
        self.write_bit(0x01, 1, false)
    }

    pub fn timer_interrupt_triggered(&mut self) -> Result<bool, Error> {
        Ok(self.read_u8(0x01)?.get_bit(2))
    }

    pub fn clear_timer_interrupt(&mut self) -> Result<(), Error> {
        self.write_bit(0x01, 2, false)
    }

    pub fn alarm_interrupt_triggered(&mut self) -> Result<bool, Error> {
        Ok(self.read_u8(0x01)?.get_bit(3))
    }

    pub fn clear_alarm_interrupt(&mut self) -> Result<(), Error> {
        self.write_bit(0x01, 3, false)
    }
}