use core::marker::PhantomData;
use stm32_metapac as pac;
use crate::rcc::rcc::ClockEnable;

pub mod gpio_port {
    pub struct Gpioa;
    pub struct Gpiob;
    pub struct Gpioc;
    pub struct Gpiod;
    pub struct Gpioe;
    pub struct Gpiof;
    pub struct Gpiog;
    pub struct Gpioh;
}

use gpio_port::*;

pub type Pa0 = Pin<Gpioa, 0>;
pub type Pa1 = Pin<Gpioa, 1>;
pub type Pa2 = Pin<Gpioa, 2>;
pub type Pa3 = Pin<Gpioa, 3>;
pub type Pa4 = Pin<Gpioa, 4>;
pub type Pa5 = Pin<Gpioa, 5>;
pub type Pa6 = Pin<Gpioa, 6>;
pub type Pa7 = Pin<Gpioa, 7>;
pub type Pa8 = Pin<Gpioa, 8>;
pub type Pa9 = Pin<Gpioa, 9>;
pub type Pa10 = Pin<Gpioa, 10>;
pub type Pa11 = Pin<Gpioa, 11>;
pub type Pa12 = Pin<Gpioa, 12>;
pub type Pa13 = Pin<Gpioa, 13>;
pub type Pa14 = Pin<Gpioa, 14>;
pub type Pa15 = Pin<Gpioa, 15>;

pub type Pb0 = Pin<Gpiob, 0>;
pub type Pb1 = Pin<Gpiob, 1>;
pub type Pb2 = Pin<Gpiob, 2>;
pub type Pb3 = Pin<Gpiob, 3>;
pub type Pb4 = Pin<Gpiob, 4>;
pub type Pb5 = Pin<Gpiob, 5>;
pub type Pb6 = Pin<Gpiob, 6>;
pub type Pb7 = Pin<Gpiob, 7>;
pub type Pb8 = Pin<Gpiob, 8>;
pub type Pb9 = Pin<Gpiob, 9>;
pub type Pb10 = Pin<Gpiob, 10>;
pub type Pb11 = Pin<Gpiob, 11>;
pub type Pb12 = Pin<Gpiob, 12>;
pub type Pb13 = Pin<Gpiob, 13>;
pub type Pb14 = Pin<Gpiob, 14>;
pub type Pb15 = Pin<Gpiob, 15>;

pub type Pc0 = Pin<Gpioc, 0>;
pub type Pc1 = Pin<Gpioc, 1>;
pub type Pc2 = Pin<Gpioc, 2>;
pub type Pc3 = Pin<Gpioc, 3>;
pub type Pc4 = Pin<Gpioc, 4>;
pub type Pc5 = Pin<Gpioc, 5>;
pub type Pc6 = Pin<Gpioc, 6>;
pub type Pc7 = Pin<Gpioc, 7>;
pub type Pc8 = Pin<Gpioc, 8>;
pub type Pc9 = Pin<Gpioc, 9>;
pub type Pc10 = Pin<Gpioc, 10>;
pub type Pc11 = Pin<Gpioc, 11>;
pub type Pc12 = Pin<Gpioc, 12>;
pub type Pc13 = Pin<Gpioc, 13>;
pub type Pc14 = Pin<Gpioc, 14>;
pub type Pc15 = Pin<Gpioc, 15>;

pub type Pd0 = Pin<Gpiod, 0>;
pub type Pd1 = Pin<Gpiod, 1>;
pub type Pd2 = Pin<Gpiod, 2>;
pub type Pd3 = Pin<Gpiod, 3>;
pub type Pd4 = Pin<Gpiod, 4>;
pub type Pd5 = Pin<Gpiod, 5>;
pub type Pd6 = Pin<Gpiod, 6>;
pub type Pd7 = Pin<Gpiod, 7>;
pub type Pd8 = Pin<Gpiod, 8>;
pub type Pd9 = Pin<Gpiod, 9>;
pub type Pd10 = Pin<Gpiod, 10>;
pub type Pd11 = Pin<Gpiod, 11>;
pub type Pd12 = Pin<Gpiod, 12>;
pub type Pd13 = Pin<Gpiod, 13>;
pub type Pd14 = Pin<Gpiod, 14>;
pub type Pd15 = Pin<Gpiod, 15>;

pub type Pe0 = Pin<Gpioe, 0>;
pub type Pe1 = Pin<Gpioe, 1>;
pub type Pe2 = Pin<Gpioe, 2>;
pub type Pe3 = Pin<Gpioe, 3>;
pub type Pe4 = Pin<Gpioe, 4>;
pub type Pe5 = Pin<Gpioe, 5>;
pub type Pe6 = Pin<Gpioe, 6>;
pub type Pe7 = Pin<Gpioe, 7>;
pub type Pe8 = Pin<Gpioe, 8>;
pub type Pe9 = Pin<Gpioe, 9>;
pub type Pe10 = Pin<Gpioe, 10>;
pub type Pe11 = Pin<Gpioe, 11>;
pub type Pe12 = Pin<Gpioe, 12>;
pub type Pe13 = Pin<Gpioe, 13>;
pub type Pe14 = Pin<Gpioe, 14>;
pub type Pe15 = Pin<Gpioe, 15>;

pub type Pf0 = Pin<Gpiof, 0>;
pub type Pf1 = Pin<Gpiof, 1>;
pub type Pf2 = Pin<Gpiof, 2>;
pub type Pf3 = Pin<Gpiof, 3>;
pub type Pf4 = Pin<Gpiof, 4>;
pub type Pf5 = Pin<Gpiof, 5>;
pub type Pf6 = Pin<Gpiof, 6>;
pub type Pf7 = Pin<Gpiof, 7>;
pub type Pf8 = Pin<Gpiof, 8>;
pub type Pf9 = Pin<Gpiof, 9>;
pub type Pf10 = Pin<Gpiof, 10>;
pub type Pf11 = Pin<Gpiof, 11>;
pub type Pf12 = Pin<Gpiof, 12>;
pub type Pf13 = Pin<Gpiof, 13>;
pub type Pf14 = Pin<Gpiof, 14>;
pub type Pf15 = Pin<Gpiof, 15>;

pub type Pg0 = Pin<Gpiog, 0>;
pub type Pg1 = Pin<Gpiog, 1>;
pub type Pg2 = Pin<Gpiog, 2>;
pub type Pg3 = Pin<Gpiog, 3>;
pub type Pg4 = Pin<Gpiog, 4>;
pub type Pg5 = Pin<Gpiog, 5>;
pub type Pg6 = Pin<Gpiog, 6>;
pub type Pg7 = Pin<Gpiog, 7>;
pub type Pg8 = Pin<Gpiog, 8>;
pub type Pg9 = Pin<Gpiog, 9>;
pub type Pg10 = Pin<Gpiog, 10>;
pub type Pg11 = Pin<Gpiog, 11>;
pub type Pg12 = Pin<Gpiog, 12>;
pub type Pg13 = Pin<Gpiog, 13>;
pub type Pg14 = Pin<Gpiog, 14>;
pub type Pg15 = Pin<Gpiog, 15>;

pub type Ph0 = Pin<Gpioh, 0>;
pub type Ph1 = Pin<Gpioh, 1>;
pub type Ph2 = Pin<Gpioh, 2>;
pub type Ph3 = Pin<Gpioh, 3>;
pub type Ph4 = Pin<Gpioh, 4>;
pub type Ph5 = Pin<Gpioh, 5>;
pub type Ph6 = Pin<Gpioh, 6>;
pub type Ph7 = Pin<Gpioh, 7>;
pub type Ph8 = Pin<Gpioh, 8>;
pub type Ph9 = Pin<Gpioh, 9>;
pub type Ph10 = Pin<Gpioh, 10>;
pub type Ph11 = Pin<Gpioh, 11>;
pub type Ph12 = Pin<Gpioh, 12>;
pub type Ph13 = Pin<Gpioh, 13>;
pub type Ph14 = Pin<Gpioh, 14>;
pub type Ph15 = Pin<Gpioh, 15>;

pub trait GpioPort: ClockEnable {
    fn ptr() -> *mut pac::gpio::Gpio;
}

impl GpioPort for gpio_port::Gpioa {
    fn ptr() -> *mut pac::gpio::Gpio { pac::GPIOA.as_ptr() as _ }
}
impl GpioPort for gpio_port::Gpiob {
    fn ptr() -> *mut pac::gpio::Gpio { pac::GPIOB.as_ptr() as _ }
}
impl GpioPort for gpio_port::Gpioc {
    fn ptr() -> *mut pac::gpio::Gpio { pac::GPIOC.as_ptr() as _ }
}
impl GpioPort for gpio_port::Gpiod {
    fn ptr() -> *mut pac::gpio::Gpio { pac::GPIOD.as_ptr() as _ }
}
impl GpioPort for gpio_port::Gpioe {
    fn ptr() -> *mut pac::gpio::Gpio { pac::GPIOE.as_ptr() as _ }
}
impl GpioPort for gpio_port::Gpiof {
    fn ptr() -> *mut pac::gpio::Gpio { pac::GPIOF.as_ptr() as _ }
}
impl GpioPort for gpio_port::Gpiog {
    fn ptr() -> *mut pac::gpio::Gpio { pac::GPIOG.as_ptr() as _ }
}

pub struct Pin<PORT: GpioPort, const N: usize> {
    _port: PhantomData<PORT>
}

impl<PORT: GpioPort, const N: usize> Pin<PORT, N> {    
    pub fn output() -> Self {
        PORT::enable();
        PORT::reset();
        
        unsafe {
            let gpio = &*PORT::ptr();
            gpio.moder().write(|w| w.set_moder(N, pac::gpio::vals::Moder::OUTPUT));
            gpio.otyper().write(|w| w.set_ot(N, pac::gpio::vals::Ot::PUSH_PULL));
            gpio.ospeedr().write(|w| w.set_ospeedr(N, pac::gpio::vals::Ospeedr::MEDIUM_SPEED));
        }
        
        Self { _port: PhantomData }
    }
    
    pub fn input() -> Self {
        PORT::enable();
        PORT::reset();
        
        unsafe {
            let gpio = &*PORT::ptr();
            gpio.moder().write(|w| w.set_moder(N, pac::gpio::vals::Moder::INPUT));
        }
        
        Self { _port: PhantomData }
    }
    
    pub fn analog() -> Self {
        PORT::enable();
        PORT::reset();
        
        unsafe {
            let gpio = &*PORT::ptr();
            gpio.moder().write(|w| w.set_moder(N, pac::gpio::vals::Moder::ANALOG));
        }
        
        Self { _port: PhantomData }
    }
    
    pub fn push_pull(&self) {
        unsafe {
            let gpio = &*PORT::ptr();
            gpio.otyper().write(|w| w.set_ot(N, pac::gpio::vals::Ot::PUSH_PULL));
        }
    }
    
    pub fn open_drain(&self) {
        unsafe {
            let gpio = &*PORT::ptr();
            gpio.otyper().write(|w| w.set_ot(N, pac::gpio::vals::Ot::OPEN_DRAIN));
        }
    }

    pub fn low_speed(&self) {
        unsafe {
            let gpio = &*PORT::ptr();
            gpio.ospeedr().write(|w| w.set_ospeedr(N, pac::gpio::vals::Ospeedr::LOW_SPEED));
        }
    }
    
    pub fn medium_speed(&self) {
        unsafe {
            let gpio = &*PORT::ptr();
            gpio.ospeedr().write(|w| w.set_ospeedr(N, pac::gpio::vals::Ospeedr::MEDIUM_SPEED));
        }
    }
    
    pub fn high_speed(&self) {
        unsafe {
            let gpio = &*PORT::ptr();
            gpio.ospeedr().write(|w| w.set_ospeedr(N, pac::gpio::vals::Ospeedr::HIGH_SPEED));
        }
    }
    
    pub fn very_high_speed(&self) {
        unsafe {
            let gpio = &*PORT::ptr();
            gpio.ospeedr().write(|w| w.set_ospeedr(N, pac::gpio::vals::Ospeedr::VERY_HIGH_SPEED));
        }
    }
    
    pub fn pull_none(&self) {
        unsafe {
            let gpio = &*PORT::ptr();
            gpio.pupdr().write(|w| w.set_pupdr(N, pac::gpio::vals::Pupdr::FLOATING));
        }
    }
    
    pub fn pull_up(&self) {
        unsafe {
            let gpio = &*PORT::ptr();
            gpio.pupdr().write(|w| w.set_pupdr(N, pac::gpio::vals::Pupdr::PULL_UP));
        }
    }
    
    pub fn pull_down(&self) {
        unsafe {
            let gpio = &*PORT::ptr();
            gpio.pupdr().write(|w| w.set_pupdr(N, pac::gpio::vals::Pupdr::PULL_DOWN));
        }
    }
    
    pub fn set_alt_function(&self, af: u8) {
        unsafe {
            let gpio = &*PORT::ptr();
            if N < 8 {
                gpio.afr(N).write(|w| w.set_afr(N, af));
            } else {
                gpio.afr(N).write(|w| w.set_afr(N - 8, af));
            }
        }
    }

    pub fn set_high(&self) {
        unsafe { (&*PORT::ptr()).bsrr().write(|w| w.set_bs(N, true)); }
    }
    
    pub fn set_low(&self) {
        unsafe { (&*PORT::ptr()).bsrr().write(|w| w.set_br(N, true)); }
    }
    
    pub fn toggle(&self) {
        unsafe {
            let gpio = &*PORT::ptr();
            let odr: u32 = gpio.odr().read().0;
            if odr & (1 << N) != 0 {
                gpio.bsrr().write(|w| w.set_br(N, true));
            } else {
                gpio.bsrr().write(|w| w.set_bs(N, true));
            }
        }
    }
    
    pub fn is_high(&self) -> bool {
        unsafe {
            let gpio = &*PORT::ptr();
            let idr: u32 = gpio.idr().read().0;
            idr & (1 << N) != 0
        }
    }
}