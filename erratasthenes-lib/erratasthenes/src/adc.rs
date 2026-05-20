use core::marker::PhantomData;
use stm32_metapac as pac;
use super::rcc::rcc::ClockEnable;

pub mod adc {
    pub struct Adc1;
    pub struct Adc2;
    pub struct Adc4;
}

use adc::*;

pub struct AdcConfig {
    pub resolution: pac::adc::vals::Res,
    pub alignment: u8,
    pub continuous: bool,
    pub overrun_mode: bool,
    
    pub discontinuous: bool,
    pub discontinuous_channels: u8, // DISC NUM 0-7
    
    pub auto_off: bool,
    
    pub wait_conversion: bool,
    
    pub external_trigger: pac::adc::vals::Exten,
    pub external_trigger_select: u8, // EXTSEL 0-15
    
    pub watchdog1_enable: bool,
    pub watchdog1_channel: u8, // AWD1CH 0-18
    pub watchdog1_single: bool, // AWD1SGL
    pub watchdog1_low_threshold: u16, // AWDLTR
    pub watchdog1_high_threshold: u16, // AWDHTR
    
    pub injected_auto: bool, // JAUTO
    pub injected_watchdog1_enable: bool, // JAWD1EN
    pub injected_discontinuous: bool, // JDISCEN
    
    pub dma_management: pac::adc::vals::Dmngt,

    pub auto_delay: bool,
}

impl Default for AdcConfig {
    fn default() -> Self {
        Self {
            resolution: pac::adc::vals::Res::BITS12,
            alignment: 0,
            continuous: false,
            overrun_mode: false,
            discontinuous: false,
            discontinuous_channels: 0,
            auto_off: false,
            wait_conversion: false,
            external_trigger: pac::adc::vals::Exten::DISABLED,
            external_trigger_select: 0,
            watchdog1_enable: false,
            watchdog1_channel: 0,
            watchdog1_single: false,
            watchdog1_low_threshold: 0,
            watchdog1_high_threshold: 4095,
            injected_auto: false,
            injected_watchdog1_enable: false,
            injected_discontinuous: false,
            dma_management: pac::adc::vals::Dmngt::DMA_ONE_SHOT,
            auto_delay: false,
        }
    }
}

pub struct Channel<const N: u8>;

impl<const N: u8> Channel<N> {
    pub fn new() -> Self { Self }
    pub const fn number(&self) -> u8 { N }
}

pub type In5 = Channel<5>;
pub type In6 = Channel<6>;
pub type In7 = Channel<7>;
pub type In8 = Channel<8>;
pub type In9 = Channel<9>;
pub type In10 = Channel<10>;
pub type In11 = Channel<11>;
pub type In12 = Channel<12>;
pub type In15 = Channel<15>;
pub type In16 = Channel<16>;

pub trait AdcPeriph: ClockEnable {
    fn ptr() -> *mut pac::adc::Adc;
    const RESOLUTION_BITS: u8;
    const MAX_VALUE: u16;
}

macro_rules! impl_adc {
    ($name:ident, $ptr:expr, $resolution_bits:expr) => {
        impl AdcPeriph for $name {
            fn ptr() -> *mut pac::adc::Adc { $ptr as _ }
            const RESOLUTION_BITS: u8 = $resolution_bits;
            const MAX_VALUE: u16 = (1 << $resolution_bits) - 1;
        }

        paste::paste! {
            pub type [<Adc $name>] = Adc<$name>;
        }
    };
}

impl_adc!(Adc1, pac::ADC1.as_ptr(), 14);
impl_adc!(Adc2, pac::ADC2.as_ptr(), 14);
impl_adc!(Adc4, pac::ADC4.as_ptr(), 12);

pub struct Adc<T: AdcPeriph> {
    _adc: PhantomData<T>,
    _config: AdcConfig,
}

impl<T: AdcPeriph> Adc<T> {
    pub fn new(config: AdcConfig) -> Self {
        T::enable();
        T::reset();
        
        unsafe {
            let adc = T::ptr();
            
            (*adc).cfgr().modify(|w| {
                w.set_autdly(config.auto_delay);
                w.set_awd1ch(config.watchdog1_channel);
                w.set_awd1en(config.watchdog1_enable);
                w.set_awd1sgl(config.watchdog1_single);
                w.set_discnum(config.discontinuous_channels);
                w.set_dmngt(config.dma_management);
                w.set_extsel(config.external_trigger_select);
                w.set_jauto(config.injected_auto);
                w.set_jawd1en(config.injected_watchdog1_enable);
                w.set_jdiscen(config.injected_discontinuous);
                w.set_res(config.resolution);
                w.set_cont(config.continuous);
                w.set_discen(config.discontinuous);
                w.set_ovrmod(config.overrun_mode);
                w.set_exten(config.external_trigger);
            });
            
            (*adc).cr().modify(|w| w.set_aden(true));
            while !(*adc).cr().read().aden() {}
        }
        
        Self { _adc: PhantomData, _config: config }
    }
    
    pub fn read_channel<const N: u8>(&self, _channel: Channel<N>) -> u16 {
        unsafe {
            let ptr = T::ptr();
            
            (*ptr).sqr1().modify(|_| 0);
            (*ptr).sqr3().modify(|_| N);
            (*ptr).cr().modify(|_| true);
            while !(*ptr).isr().read().eoc() {}
            (*ptr).dr().read().0 as u16
        }
    }

    pub fn set_continuous_mode(&mut self, enabled: bool) {
        unsafe { (*T::ptr()).cfgr().modify(|w| w.set_cont(enabled)); }
    }
    
    pub fn set_discontinuous_mode(&mut self, enabled: bool) {
        unsafe { (*T::ptr()).cfgr().modify(|w| w.set_discen(enabled)); }
    }
    
    pub fn set_resolution(&mut self, bits: u8) {
        let res = match bits {
            14 => pac::adc::vals::Res::BITS14,
            12 => pac::adc::vals::Res::BITS12,
            10 => pac::adc::vals::Res::BITS10,
            8 => pac::adc::vals::Res::BITS8,
            _ => pac::adc::vals::Res::BITS12,
        };
        unsafe { (*T::ptr()).cfgr().modify(|w| w.set_res(res)); }
    }

    pub fn calibrate(&self) {
        unsafe {
            let adc = T::ptr();
            (*adc).cr().modify(|w| w.set_adcal(true));
            while (*adc).cr().read().adcal() {}
        }
    }

    pub fn stop(&self) {
        unsafe {
            let adc = T::ptr();
            (*adc).cr().modify(|w| w.set_adstp(pac::adc::vals::Adstp::STOP));
        }
    }

    pub fn is_converting(&self) -> bool {
        unsafe { 
            let adc = &*T::ptr();
            adc.cr().read().adstart() 
        }
    }

    pub fn disable(&self) {
        unsafe {
            let adc = T::ptr();
            (*adc).cr().modify(|w| w.set_aden(false));
        }
    }

    pub fn enable(&self) {
        unsafe {
            let adc = T::ptr();
            (*adc).cr().modify(|w| w.set_aden(true));
            while !(*adc).cr().read().aden() {}
        }
    }

    pub fn read_channel_timeout<const N: u8, TIM: crate::tim::TimerPeriph>(
        &self,
        _channel: Channel<N>,
        timeout_ms: u32,
        timer: &crate::tim::Timer<TIM>,
    ) -> Option<u16> {
        let start = timer.get_counter();
        
        unsafe {
            let adc = &*T::ptr();
            
            (*adc).sqr1().modify(|w| w.set_sq(0, 0));
            (*adc).sqr3().modify(|w| w.set_sq(0, N));
            (*adc).cr().modify(|w| w.set_adstart(true));
            
            while !(*adc).isr().read().eoc() {
                if timer.get_counter() - start >= timeout_ms {
                    return None;
                }
            }
            
            Some((*adc).dr().read().0 as u16)
        }
    }
    
    pub fn resolution_bits(&self) -> u8 { T::RESOLUTION_BITS }
    pub fn max_value(&self) -> u16 { T::MAX_VALUE }
}