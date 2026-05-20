use super::rcc::ClockEnable;
use stm32_metapac as pac;
use crate::usart::usart::*;

macro_rules! impl_usart_clock {
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

impl_usart_clock!(Usart1, set_usart1en, set_usart1rst, apb2enr, apb2rstr);
impl_usart_clock!(Usart2, set_usart2en, set_usart2rst, apb1enr1, apb1rstr1);
impl_usart_clock!(Usart3, set_usart3en, set_usart3rst, apb1enr1, apb1rstr1);
impl_usart_clock!(Usart6, set_usart6en, set_usart6rst, apb1enr1, apb1rstr1);