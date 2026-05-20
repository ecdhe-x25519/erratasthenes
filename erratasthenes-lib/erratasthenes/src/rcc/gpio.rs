use super::rcc::ClockEnable;
use stm32_metapac as pac;
use crate::gpio::gpio_port::*;

macro_rules! impl_gpio_clock {
    ($port:ident, $en_bit:ident, $rst_bit:ident) => {
        impl ClockEnable for $port {
            fn enable() {
                unsafe { pac::rcc::Rcc::from_ptr(pac::RCC.as_ptr()).ahb2enr1().modify(|w| w.$en_bit(true)); }
            }
            fn disable() {
                unsafe { pac::rcc::Rcc::from_ptr(pac::RCC.as_ptr()).ahb2enr1().modify(|w| w.$en_bit(false)); }
            }
            fn reset() {
                unsafe { pac::rcc::Rcc::from_ptr(pac::RCC.as_ptr()).ahb2rstr1().modify(|w| w.$rst_bit(true)); }
                core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
                unsafe { pac::rcc::Rcc::from_ptr(pac::RCC.as_ptr()).ahb2rstr1().modify(|w| w.$rst_bit(false)); }
            }
        }
    };
}

impl_gpio_clock!(Gpioa, set_gpioaen, set_gpioarst);
impl_gpio_clock!(Gpiob, set_gpioben, set_gpiobrst);
impl_gpio_clock!(Gpioc, set_gpiocen, set_gpiocrst);
impl_gpio_clock!(Gpiod, set_gpioden, set_gpiodrst);
impl_gpio_clock!(Gpioe, set_gpioeen, set_gpioerst);
impl_gpio_clock!(Gpiof, set_gpiofen, set_gpiofrst);
impl_gpio_clock!(Gpiog, set_gpiogen, set_gpiogrst);
impl_gpio_clock!(Gpioh, set_gpiohen, set_gpiohrst);