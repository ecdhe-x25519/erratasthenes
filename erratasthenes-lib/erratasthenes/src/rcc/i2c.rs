use super::rcc::ClockEnable;
use stm32_metapac as pac;
use crate::i2c::i2c::*;

macro_rules! impl_i2c_clock {
    ($name:ident, $en_bit:ident, $rst_bit:ident, $en_reg:ident, $rst_reg:ident) => {
        impl ClockEnable for $name {
            fn enable() {
                unsafe { pac::rcc::Rcc::from_ptr(pac::RCC.as_ptr()).$en_reg().modify(|w| w.$en_bit(true)); }
            }
            fn disable() {
                unsafe { pac::rcc::Rcc::from_ptr(pac::RCC.as_ptr()).$en_reg().modify(|w| w.$en_bit(false)); }
            }
            fn reset() {
                unsafe { pac::rcc::Rcc::from_ptr(pac::RCC.as_ptr()).$rst_reg().modify(|w| w.$rst_bit(true)); }
                core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
                unsafe { pac::rcc::Rcc::from_ptr(pac::RCC.as_ptr()).$rst_reg().modify(|w| w.$rst_bit(false)); }
            }
        }
    }
}

impl_i2c_clock!(I2c1, set_i2c1en, set_i2c1rst, apb1enr1, apb1rstr1);
impl_i2c_clock!(I2c2, set_i2c2en, set_i2c2rst, apb1enr1, apb1rstr1);
impl_i2c_clock!(I2c4, set_i2c4en, set_i2c4rst, apb1enr2, apb1rstr2);
impl_i2c_clock!(I2c5, set_i2c5en, set_i2c5rst, apb1enr2, apb1rstr2);
impl_i2c_clock!(I2c6, set_i2c6en, set_i2c6rst, apb1enr2, apb1rstr2);