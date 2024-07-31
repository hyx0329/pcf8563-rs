use crate::{Error, I2c, Pcf8563, PCF8563_ADDR};
use crate::datetime::WeekDay;
use crate::utils::*;

impl<I2C: I2c> Pcf8563<I2C> {
    /// Sets minute alarm value. This will automatically enable the minute alarm.
    pub fn set_minute_alarm(&mut self, value: u8) -> Result<(), Error> {
        if value > 59 {
            Err(Error::Other)
        } else {
            let reg_val = u8_bcd_encode(value);
            self.write_u8(0x09, reg_val)
        }
    }

    /// Enables checking the minute alarm value. The raw bit value is 0.
    pub fn enable_minute_alarm(&mut self) -> Result<(), Error> {
        self.write_bit(0x09, 7, false)
    }

    /// Disables checking the minute alarm value. The raw bit value is 1.
    pub fn disable_minute_alarm(&mut self) -> Result<(), Error> {
        self.write_bit(0x09, 7, true)
    }

    /// Sets hour alarm value. This will automatically enable the hour alarm.
    pub fn set_hour_alarm(&mut self, value: u8) -> Result<(), Error> {
        if value > 23 {
            Err(Error::Other)
        } else {
            let reg_val = u8_bcd_encode(value);
            self.write_u8(0x0A, reg_val)
        }
    }

    /// Enables checking the hour alarm value. The raw bit value is 0.
    pub fn enable_hour_alarm(&mut self) -> Result<(), Error> {
        self.write_bit(0x0A, 7, false)
    }

    /// Disables checking the hour alarm value. The raw bit value is 1.
    pub fn disable_hour_alarm(&mut self) -> Result<(), Error> {
        self.write_bit(0x0A, 7, true)
    }

    /// Sets day alarm value. This will automatically enable the day alarm.
    pub fn set_day_alarm(&mut self, value: u8) -> Result<(), Error> {
        if value > 31 {
            Err(Error::Other)
        } else {
            let reg_val = u8_bcd_encode(value);
            self.write_u8(0x0B, reg_val)
        }
    }

    /// Enables checking the day alarm value. The raw bit value is 0.
    pub fn enable_day_alarm(&mut self) -> Result<(), Error> {
        self.write_bit(0x0B, 7, false)
    }

    /// Disables checking the day alarm value. The raw bit value is 1.
    pub fn disable_day_alarm(&mut self) -> Result<(), Error> {
        self.write_bit(0x0B, 7, true)
    }

    /// Sets weekday alarm value. This will automatically enable the weekday alarm.
    pub fn set_weekday_alarm(&mut self, value: WeekDay) -> Result<(), Error> {
        self.write_u8(0x0C, value.into())
    }

    /// Enables checking the weekday alarm value. The raw bit value is 0.
    pub fn enable_weekday_alarm(&mut self) -> Result<(), Error> {
        self.write_bit(0x0C, 7, false)
    }

    /// Disables checking the weekday alarm value. The raw bit value is 1.
    pub fn disable_weekday_alarm(&mut self) -> Result<(), Error> {
        self.write_bit(0x0C, 7, true)
    }

    /// Clears/disables all alarm by disabling each alarm and set the corresponding value to 0.
    pub fn clear_all_alarms(&mut self) -> Result<(), Error> {
        let buf: [u8; 5] = [0x09, 0x80, 0x80, 0x80, 0x80];
        Ok(self.i2c.write(PCF8563_ADDR, &buf)?)
    }
}