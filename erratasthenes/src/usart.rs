use core::marker::PhantomData;
use stm32_metapac as pac;
use crate::rcc::rcc::ClockEnable;
use crate::gpio::{GpioPort, Pin};

pub mod usart {
    pub struct Usart1;
    pub struct Usart2;
    pub struct Usart3;
    pub struct Usart6;
}

use usart::*;

pub trait UsartPeriph: ClockEnable {
    fn ptr() -> *mut pac::usart::Usart;
}

pub struct UsartConfig {
    pub baudrate: u32,
    pub word_length: WordLength,
    pub stop_bits: StopBits,
    pub parity: Parity,
}

pub enum WordLength {
    Bits8 = 0,
    Bits9 = 1,
}

pub enum StopBits {
    Stop1 = 0,
    Stop2 = 1,
}

pub enum Parity {
    None = 0,
    Even = 2,
    Odd = 3,
}

impl Default for UsartConfig {
    fn default() -> Self {
        Self {
            baudrate: 115200,
            word_length: WordLength::Bits8,
            stop_bits: StopBits::Stop1,
            parity: Parity::None,
        }
    }
}

pub struct Usart<T: UsartPeriph> {
    _usart: PhantomData<T>,
}

macro_rules! impl_usart {
    ($name:ident, $ptr:expr) => {
        impl UsartPeriph for $name {
            fn ptr() -> *mut pac::usart::Usart { $ptr as _ }
        }
    };
}

impl_usart!(Usart1, pac::USART1.as_ptr());
impl_usart!(Usart2, pac::USART2.as_ptr());
impl_usart!(Usart3, pac::USART3.as_ptr());
impl_usart!(Usart6, pac::USART6.as_ptr());

impl<T: UsartPeriph> Usart<T> {
    pub fn new<PORT1: GpioPort, const TX: usize, PORT2: GpioPort, const RX: usize>(
        config: UsartConfig,
        pclk: u32,
        tx_pin: Pin<PORT1, TX>,
        rx_pin: Pin<PORT2, RX>,
    ) -> Self {
        T::enable();
        T::reset();
        
        tx_pin.set_alt_function(7);
        tx_pin.push_pull();
        tx_pin.medium_speed();
        
        rx_pin.set_alt_function(7);
        rx_pin.pull_up();
        
        unsafe {
            let usart = T::ptr();
            
            (*usart).cr1().modify(|w| {
                w.set_ue(true);
                w.set_te(true);
                w.set_re(true);
            });
            
            let brr = pclk / config.baudrate;
            (*usart).brr().write(|_| brr as u16);
            
            let stop_val = match config.stop_bits {
                StopBits::Stop1 => pac::usart::vals::Stop::STOP1,
                StopBits::Stop2 => pac::usart::vals::Stop::STOP2,
            };
            (*usart).cr2().modify(|w| w.set_stop(stop_val));
            
            let m0 = match config.word_length {
                WordLength::Bits8 => pac::usart::vals::M0::BIT8,
                WordLength::Bits9 => pac::usart::vals::M0::BIT9,
            };
            match config.parity {
                Parity::None => {
                    (*usart).cr1().modify(|w| {
                        w.set_pce(false);
                        w.set_ps(pac::usart::vals::Ps::EVEN);
                    });
                },
                Parity::Even => {
                    (*usart).cr1().modify(|w| {
                        w.set_pce(true);
                        w.set_ps(pac::usart::vals::Ps::EVEN);
                    });
                },
                Parity::Odd => {
                    (*usart).cr1().modify(|w| {
                        w.set_pce(true);
                        w.set_ps(pac::usart::vals::Ps::ODD);
                    });
                },
            };
            (*usart).cr1().modify(|w| {
                w.set_m0(m0);
            });
        }
        
        Self { _usart: PhantomData }
    }
    
    pub fn write_byte(&self, data: u8) {
        unsafe {
            let usart = T::ptr();
            while !(*usart).isr().read().txe() {}
            (*usart).tdr().write(|_| data as u16);
        }
    }
    
    pub fn write_str(&self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }
    
    pub fn read_byte(&self) -> u8 {
        unsafe {
            let usart = T::ptr();
            while !(*usart).isr().read().rxne() {}
            (*usart).rdr().read().dr() as u8
        }
    }
}

pub type Usart1Type = Usart<Usart1>;
pub type Usart2Type = Usart<Usart2>;
pub type Usart3Type = Usart<Usart3>;
pub type Usart6Type = Usart<Usart6>;