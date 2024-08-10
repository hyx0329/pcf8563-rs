//! Datetime module, provides basic datetime implementation.

use crate::{Error, I2c, Pcf8563, PCF8563_ADDR};
use crate::utils::*;
use bit_field::BitField;
use num_enum::{FromPrimitive, IntoPrimitive};

/// Date & time representation.
///
/// Only basic range checks are performed.
/// It's **NOT** guaranteed to be a valid time if crafted manually.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DateTime {
    pub second: u8,
    pub minute: u8,
    pub hour: u8,
    pub day: u8,
    pub month: u8,
    pub year: u16,
    pub weekday: WeekDay,
}

impl Default for DateTime {
    fn default() -> Self {
        Self {
            second: 0,
            minute: 0,
            hour: 0,
            day: 1,
            month: 1,
            year: 2000,
            weekday: WeekDay::Saturday,
        }
    }
}

impl DateTime {
    pub fn new() -> Self {
        Default::default()
    }

    #[must_use]
    pub fn second(mut self, value: u8) -> Self {
        if value <= 59 {
            self.second = value;
        }
        self
    }

    #[must_use]
    pub fn minute(mut self, value: u8) -> Self {
        if value <= 59 {
            self.minute = value;
        }
        self
    }

    #[must_use]
    pub fn hour(mut self, value: u8) -> Self {
        if value <= 23 {
            self.hour = value;
        }
        self
    }

    #[must_use]
    pub fn day(mut self, value: u8) -> Self {
        if (1..=31).contains(&value) {
            self.day = value;
        }
        self
    }

    #[must_use]
    pub fn weekday(mut self, value: WeekDay) -> Self {
        self.weekday = value;
        self
    }

    #[must_use]
    pub fn month(mut self, value: u8) -> Self {
        if (1..=12).contains(&value) {
            self.month = value;
        }
        self
    }

    #[must_use]
    pub fn year(mut self, value: u16) -> Self {
        if (1900..=2099).contains(&value) {
            self.year = value;
        }
        self
    }

    #[inline]
    pub fn iso_second(&self) -> u8 {
        self.second
    }

    #[inline]
    pub fn iso_minute(&self) -> u8 {
        self.minute
    }

    #[inline]
    pub fn iso_hour(&self) -> u8 {
        self.hour
    }

    #[inline]
    pub fn iso_day(&self) -> u8 {
        self.day
    }

    #[inline]
    pub fn iso_month(&self) -> u8 {
        self.month
    }

    #[inline]
    pub fn iso_year(&self) -> u16 {
        self.year
    }

    #[inline]
    pub fn iso_weekday(&self) -> u8 {
        self.weekday.to_iso_weekday()
    }
}

/// Weekday representation. The numbering matches the hardware values.
#[repr(u8)]
#[derive(IntoPrimitive, FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WeekDay {
    #[num_enum(default)]
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl WeekDay {
    #[inline]
    pub fn to_iso_weekday(self) -> u8 {
        let internal_value: u8 = self.into();
        if internal_value == 0 {
            7
        } else {
            internal_value
        }
    }
}

impl<I2C: I2c> Pcf8563<I2C> {
    pub fn datetime(&mut self) -> Result<DateTime, Error> {
        let mut buf: [u8; 7] = [0; 7];
        self.i2c.write_read(PCF8563_ADDR, &[0x02], &mut buf)?;
        let datetime = DateTime::new()
            .second(u8_bcd_decode(buf[0] & 0x7F))
            .minute(u8_bcd_decode(buf[1] & 0x7F))
            .hour(u8_bcd_decode(buf[2] & 0x3F))
            .day(u8_bcd_decode(buf[3] & 0x3F))
            .weekday(WeekDay::from_primitive(u8_bcd_decode(buf[4] & 0x07)))
            .month(u8_bcd_decode(buf[5] & 0x1F));

        if buf[5].get_bit(7) {
            Ok(datetime.year((buf[6] as u16) + 1900))
        } else {
            Ok(datetime.year((buf[6] as u16) + 2000))
        }
    }

    pub fn set_datetime(&mut self, value: DateTime) -> Result<(), Error> {
        let mut buf: [u8; 8] = [0; 8];
        buf[0] = 0x02; // addr
        buf[1] = u8_bcd_encode(value.second);
        buf[2] = u8_bcd_encode(value.minute);
        buf[3] = u8_bcd_encode(value.hour);
        buf[4] = u8_bcd_encode(value.day);
        buf[5] = u8_bcd_encode(value.weekday.into());
        buf[6] = u8_bcd_encode(value.month);
        buf[7] = u8_bcd_encode((value.year % 100) as u8);
        if value.year < 2000 {
            buf[6].set_bit(7, true);
        }
        Ok(self.i2c.write(PCF8563_ADDR, &buf)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iso_weekday() {
        let value: u8 = WeekDay::Monday.to_iso_weekday();
        assert_eq!(1, value);
        let value: u8 = WeekDay::Tuesday.to_iso_weekday();
        assert_eq!(2, value);
        let value: u8 = WeekDay::Sunday.to_iso_weekday();
        assert_eq!(7, value);

        let day = WeekDay::from_primitive(7);
        assert_eq!(day, WeekDay::Sunday);

        let day = WeekDay::from_primitive(1);
        assert_eq!(day, WeekDay::Monday);
    }
}