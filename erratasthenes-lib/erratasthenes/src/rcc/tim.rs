use stm32_metapac as pac;
use crate::rcc::rcc::ClockEnable;
use crate::tim::tim::{general::*, basic::*, advanced::*};

macro_rules! impl_timer_clock {
    (
        $timer:ident,
        $en_bit:ident,
        $rst_bit:ident,
        $bus_enr:ident,
        $bus_rstr:ident
    ) => {
        impl ClockEnable for $timer {
            fn enable() {
                unsafe { pac::rcc::Rcc::from_ptr(pac::RCC.as_ptr()).$bus_enr().modify(|w| w.$en_bit(true)); }
            }
            fn disable() {
                unsafe { pac::rcc::Rcc::from_ptr(pac::RCC.as_ptr()).$bus_enr().modify(|w| w.$en_bit(false)); }
            }
            fn reset() {
                unsafe { pac::rcc::Rcc::from_ptr(pac::RCC.as_ptr()).$bus_rstr().modify(|w| w.$rst_bit(true)); }
                core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
                unsafe { pac::rcc::Rcc::from_ptr(pac::RCC.as_ptr()).$bus_rstr().modify(|w| w.$rst_bit(false)); }
            }
        }
    }
}

impl_timer_clock!(Tim2, set_tim2en, set_tim2rst, apb1enr1, apb1rstr1);
impl_timer_clock!(Tim3, set_tim3en, set_tim3rst, apb1enr1, apb1rstr1);
impl_timer_clock!(Tim4, set_tim4en, set_tim4rst, apb1enr1, apb1rstr1);
impl_timer_clock!(Tim5, set_tim5en, set_tim5rst, apb1enr1, apb1rstr1);

impl_timer_clock!(Tim6, set_tim6en, set_tim6rst, apb1enr1, apb1rstr1);
impl_timer_clock!(Tim7, set_tim7en, set_tim7rst, apb1enr1, apb1rstr1);

impl_timer_clock!(Tim1, set_tim1en, set_tim1rst, apb2enr, apb2rstr);
impl_timer_clock!(Tim8, set_tim8en, set_tim8rst, apb2enr, apb2rstr);