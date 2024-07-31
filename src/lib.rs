#![doc = include_str!("../README.md")]
#![warn(unsafe_code)]
#![no_std]

use core::ops::RangeBounds;

use bit_field::BitField;
use embedded_hal::i2c::{Error as I2cError, ErrorKind as I2cErrorKind, I2c};

mod datetime;
mod utils;
mod timer;
mod alarm;

const PCF8563_ADDR: u8 = 0x51;

/// PCF8563 error type.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Error {
    /// An I2C error occurred during the transaction.
    I2cError(I2cErrorKind),
    /// Other error. The original error converted from may contain more information.
    Other,
}

impl<T: I2cError> From<T> for Error {
    fn from(value: T) -> Self {
        Self::I2cError(value.kind())
    }
}

impl embedded_hal::digital::Error for Error {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        embedded_hal::digital::ErrorKind::Other
    }
}

#[derive(Debug)]
pub struct Pcf8563<I2C> {
    i2c: I2C,
}

#[allow(dead_code)]
impl<I2C: I2c> Pcf8563<I2C> {
    pub fn new(&mut self, i2c: I2C) -> Self {
        Self {
            i2c
        }
    }

    pub fn power_loss(&mut self) -> Result<bool, Error> {
        Ok(self.read_u8(0x02)?.get_bit(7))
    }

    /// Sets whether to only send a pulse signal when timer interrupt triggered.
    /// 
    /// If set false, INT will be kept at low when timer interrupt triggered and enabled.
    pub fn set_timer_interrupt_use_pulse(&mut self, value: bool) -> Result<(), Error> {
        self.write_bit(0x01, 4, value)
    }

    /// Reads one u8 integer.
    fn read_u8(&mut self, reg: u8) -> Result<u8, Error> {
        let mut buf: [u8; 1] = [0; 1];

        match self.i2c.write_read(PCF8563_ADDR, &[reg], &mut buf) {
            Ok(_) => Ok(buf[0]),
            Err(e) => Err(e.into()),
        }
    }

    fn write_u8(&mut self, reg: u8, value: u8) -> Result<(), Error> {
        Ok(self.i2c.write(PCF8563_ADDR, &[reg, value])?)
    }

    /// Write a single bit.
    #[inline]
    fn write_bit(&mut self, reg: u8, bit: usize, value: bool) -> Result<(), Error> {
        let mut reg_val = self.read_u8(reg)?;
        if reg_val.get_bit(bit) == value {
            Ok(())
        } else {
            reg_val.set_bit(bit, value);
            self.write_u8(reg, reg_val)
        }
    }

    /// Writes bits.
    #[inline]
    fn write_bits<T: RangeBounds<usize>>(
        &mut self,
        reg: u8,
        range: T,
        value: u8,
    ) -> Result<(), Error> {
        let mut reg_val = self.read_u8(reg)?;
        reg_val.set_bits(range, value);
        self.write_u8(reg, reg_val)
    }
}
