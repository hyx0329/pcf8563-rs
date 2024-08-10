use crate::{Pcf8563, Error, I2c};
use num_enum::{FromPrimitive, IntoPrimitive};

#[repr(u8)]
#[derive(IntoPrimitive, FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TimerClockFrequency {
    Freq4096Hz,
    Freq64Hz,
    Freq1Hz,
    /// 1/60 Hz
    #[num_enum(default)]
    Freq1d60Hz
}

impl<I2C: I2c> Pcf8563<I2C> {
    pub fn set_timer_frequency(&mut self, value: TimerClockFrequency) -> Result<(), Error> {
        self.write_bits(0x0E, 0..=1, value.into())
    }

    pub fn set_timer_countdown(&mut self, value: u8) -> Result<(), Error> {
        self.write_u8(0x0F, value)
    }
}