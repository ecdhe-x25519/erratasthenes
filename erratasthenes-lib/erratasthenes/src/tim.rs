use core::marker::PhantomData;
use stm32_metapac as pac;
use crate::rcc::rcc::ClockEnable;

pub mod tim {
    pub mod general {
        pub struct Tim2;
        pub struct Tim3;
        pub struct Tim4;
        pub struct Tim5;
    }
    
    pub mod basic {
        pub struct Tim6;
        pub struct Tim7;
    }

    pub mod advanced {
        pub struct Tim1;
        pub struct Tim8;
    }
}

use tim::{general::*, basic::*, advanced::*};

pub struct TimerConfig {
    pub frequency: u32,
    pub one_pulse: bool, // OPM (One Pulse Mode)
    pub auto_reload: bool, // ARPE (Auto-reload preload)
    pub update_disable: bool, // UDIS (Update disable)
    pub update_source: pac::timer::vals::Urs, // URS (Update request source)
    pub counter_enable: bool, // CEN (Counter enable)
}

impl Default for TimerConfig {
    fn default() -> Self {
        Self {
            frequency: 1_000_000,
            one_pulse: false,
            auto_reload: true,
            update_disable: false,
            update_source: pac::timer::vals::Urs::COUNTER_ONLY,
            counter_enable: true,
        }
    }
}

pub trait TimerPeriph: ClockEnable {
    type Reg;
    fn ptr() -> *mut Self::Reg;
    const MAX_ARR: u32;
    const IS_32BIT: bool;
    const IS_BASIC: bool;
    const IS_ADVANCED: bool;
}

pub struct Timer<T: TimerPeriph> {
    _tim: PhantomData<T>,
    _config: TimerConfig,
}

impl<T: TimerPeriph> Timer<T> {
    pub fn new(config: TimerConfig, pclk: u32) -> Self {
        T::enable();
        T::reset();
        
        unsafe {
            let ptr = T::ptr();
            let psc = (pclk / config.frequency) - 1;
            
            if T::IS_BASIC {
                let tim = ptr as *mut pac::timer::TimBasic;
                (*tim).cr1().write(|_| 0);
                (*tim).psc().write(|_| psc as u16);
                (*tim).arr().write(|_| T::MAX_ARR as u16);
                (*tim).cnt().write(|_| 0);
                (*tim).cr1().write(|_| 1);
            } else if T::IS_32BIT {
                let tim = ptr as *mut pac::timer::TimGp32;
                (*tim).cr1().modify(|w| {
                    w.set_cen(false);
                    w.set_opm(config.one_pulse);
                    w.set_arpe(config.auto_reload);
                    w.set_udis(config.update_disable);
                    w.set_urs(config.update_source);
                });
                (*tim).psc().write(|_| psc as u16);
                (*tim).arr().write(|_| T::MAX_ARR);
                (*tim).cnt().write(|_| 0);
                (*tim).cr1().modify(|w| w.set_cen(config.counter_enable));
            } else {
                let tim = ptr as *mut pac::timer::TimGp16;
                (*tim).cr1().modify(|w| {
                    w.set_cen(false);
                    w.set_opm(config.one_pulse);
                    w.set_arpe(config.auto_reload);
                    w.set_udis(config.update_disable);
                    w.set_urs(config.update_source);
                });
                (*tim).psc().write(|_| psc as u16);
                (*tim).arr().write(|_| T::MAX_ARR as u16);
                (*tim).cnt().write(|_| 0);
                (*tim).cr1().modify(|w| w.set_cen(config.counter_enable));
            }
        }
        
        Self { _tim: PhantomData, _config: config }
    }
    
    pub fn delay_us(&self, us: u32) {
        unsafe {
            let ptr = T::ptr();
            
            if T::IS_BASIC {
                let tim = ptr as *mut pac::timer::TimBasic;
                let max_ticks = T::MAX_ARR;
                let mut remaining = us;
                while remaining > 0 {
                    let wait = if remaining > max_ticks { max_ticks } else { remaining };
                    (*tim).cnt().write(|_| 0);
                    while (*tim).cnt().read().0 < wait {}
                    remaining -= wait;
                }
            } else if T::IS_ADVANCED {
                let tim = ptr as *mut pac::timer::TimAdv;
                let max_ticks = T::MAX_ARR;
                let mut remaining = us;
                while remaining > 0 {
                    let wait = if remaining > max_ticks { max_ticks } else { remaining };
                    (*tim).cnt().write(|_| 0);
                    while (*tim).cnt().read().0 < wait {}
                    remaining -= wait;
                }
            } else if T::IS_32BIT {
                let tim = ptr as *mut pac::timer::TimGp32;
                (*tim).cnt().write(|_| 0);
                while (*tim).cnt().read() < us {}
            } else {
                let tim = ptr as *mut pac::timer::TimGp16;
                let max_ticks = T::MAX_ARR;
                let mut remaining = us;
                while remaining > 0 {
                    let wait = if remaining > max_ticks { max_ticks } else { remaining };
                    (*tim).cnt().write(|_| 0);
                    while (*tim).cnt().read().0 < wait {}
                    remaining -= wait;
                }
            }
        }
    }
    
    pub fn delay_ms(&self, ms: u32) {
        for _ in 0..ms {
            self.delay_us(1000);
        }
    }
    
    pub fn get_counter(&self) -> u32 {
        unsafe {
            let ptr = T::ptr();
            if T::IS_32BIT {
                (*(ptr as *mut pac::timer::TimGp32)).cnt().read()
            } else {
                (*(ptr as *mut pac::timer::TimGp16)).cnt().read().0
            }
        }
    }
}

pub type Timer1 = Timer<Tim1>;
pub type Timer2 = Timer<Tim2>;
pub type Timer3 = Timer<Tim3>;
pub type Timer4 = Timer<Tim4>;
pub type Timer5 = Timer<Tim5>;
pub type Timer6 = Timer<Tim6>;
pub type Timer7 = Timer<Tim7>;
pub type Timer8 = Timer<Tim8>;