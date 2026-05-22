use stm32_metapac as pac;

pub struct RccConfig {
    pub source: ClockSource,
    pub pllm: pac::rcc::vals::Pllm,
    pub plln: pac::rcc::vals::Plln,
    pub pllr: pac::rcc::vals::Plldiv,
    pub hpre: pac::rcc::vals::Hpre,
    pub ppre1: pac::rcc::vals::Ppre,
    pub ppre2: pac::rcc::vals::Ppre,
}

impl Default for RccConfig {
    fn default() -> Self {
        Self {
            source: ClockSource::Hsi,
            pllm: pac::rcc::vals::Pllm::DIV4,
            plln: pac::rcc::vals::Plln::MUL40,
            pllr: pac::rcc::vals::Plldiv::DIV2,
            hpre: pac::rcc::vals::Hpre::DIV1,
            ppre1: pac::rcc::vals::Ppre::DIV2,
            ppre2: pac::rcc::vals::Ppre::DIV1,
        }
    }
}

pub enum ClockSource {
    Hsi,
    Hse { frequency: u32 },
}

pub struct Clocks {
    pub sysclk: u32,
    pub ahb: u32,
    pub apb1: u32,
    pub apb2: u32,
}

impl Clocks {
    pub fn sysclk_mhz(&self) -> u32 { self.sysclk / 1_000_000 }
    pub fn ahb_mhz(&self) -> u32 { self.ahb / 1_000_000 }
    pub fn apb1_mhz(&self) -> u32 { self.apb1 / 1_000_000 }
    pub fn apb2_mhz(&self) -> u32 { self.apb2 / 1_000_000 }

    pub(crate) fn new() -> Self {
        Self {
            sysclk: 0,
            ahb: 0,
            apb1: 0,
            apb2: 0,
        }
    }
}

pub trait ClockEnable {
    fn enable();
    fn disable();
    fn reset();
}

pub struct Rcc {
    clocks: Clocks,
}

impl Rcc {
    pub fn init_default() -> Self {
        Self::init(RccConfig::default())
    }

    pub fn init(config: RccConfig) -> Self {
        let mut clocks: Clocks = Clocks::new();

        unsafe {
            let rcc = pac::rcc::Rcc::from_ptr(pac::RCC.as_ptr());

            let clock: u32 = match config.source {
                ClockSource::Hsi => {
                    rcc.cr().modify(|w| w.set_hsion(true));
                    while !rcc.cr().read().hsirdy() {}
                    16_000_000
                }
                ClockSource::Hse { frequency } => {
                    rcc.cr().modify(|w| w.set_hseon(true));
                    while !rcc.cr().read().hserdy() {}
                    frequency
                }
            };

            rcc.cr().modify(|w| w.set_pllon(1, false));
            while rcc.cr().read().pllrdy(1) {}

            let pllsrc = match config.source {
                ClockSource::Hsi => pac::rcc::vals::Pllsrc::HSI,
                ClockSource::Hse { .. } => pac::rcc::vals::Pllsrc::HSE,
            };

            rcc.pll1cfgr().modify(|w| {
                w.set_pllsrc(pllsrc);
                w.set_pllm(config.pllm);
            });

            rcc.pll1divr().modify(|w| {
                w.set_plln(config.plln);
                w.set_pllr(config.pllr);
                w.set_pllp(2.into());
                w.set_pllq(2.into());
            });

            rcc.cr().modify(|w| w.set_pllon(1, true));
            while !rcc.cr().read().pllrdy(1) {};

            rcc.cfgr2().modify(|w| {
                w.set_hpre(config.hpre);
                w.set_ppre1(config.ppre1);
                w.set_ppre2(config.ppre2);
            });

            rcc.cfgr1().modify(|w| w.set_sw(pac::rcc::vals::Sw::PLL1_R));
            while rcc.cfgr1().read().sws() != pac::rcc::vals::Sw::PLL1_R {}

            let pllm_val: u32 = match config.pllm {
                pac::rcc::vals::Pllm::DIV2 => 2,
                pac::rcc::vals::Pllm::DIV3 => 3,
                pac::rcc::vals::Pllm::DIV4 => 4,
                pac::rcc::vals::Pllm::DIV5 => 5,
                pac::rcc::vals::Pllm::DIV6 => 6,
                pac::rcc::vals::Pllm::DIV7 => 7,
                pac::rcc::vals::Pllm::DIV8 => 8,
                pac::rcc::vals::Pllm::DIV9 => 9,
                pac::rcc::vals::Pllm::DIV10 => 10,
                pac::rcc::vals::Pllm::DIV11 => 11,
                pac::rcc::vals::Pllm::DIV12 => 12,
                pac::rcc::vals::Pllm::DIV13 => 13,
                pac::rcc::vals::Pllm::DIV14 => 14,
                pac::rcc::vals::Pllm::DIV15 => 15,
                pac::rcc::vals::Pllm::DIV16 => 16,
                _ => 1,
            };

            let plln_val: u32 = match config.plln {
                pac::rcc::vals::Plln::MUL4 => 4,
                pac::rcc::vals::Plln::MUL8 => 8,
                pac::rcc::vals::Plln::MUL16 => 16,
                pac::rcc::vals::Plln::MUL24 => 24,
                pac::rcc::vals::Plln::MUL32 => 32,
                pac::rcc::vals::Plln::MUL40 => 40,
                pac::rcc::vals::Plln::MUL48 => 48,
                pac::rcc::vals::Plln::MUL56 => 56,
                pac::rcc::vals::Plln::MUL64 => 64,
                pac::rcc::vals::Plln::MUL72 => 72,
                pac::rcc::vals::Plln::MUL80 => 80,
                _ => 1,
            };

            let pllr_val: u32 = match config.pllr {
                pac::rcc::vals::Plldiv::DIV2 => 2,
                pac::rcc::vals::Plldiv::DIV4 => 4,
                pac::rcc::vals::Plldiv::DIV8 => 8,
                pac::rcc::vals::Plldiv::DIV16 => 16,
                _ => 1,
            };

            let vco_hz: u32 = (clock / pllm_val) * plln_val;
            let sysclk: u32 = vco_hz / pllr_val;

            let ahb: u32 = match config.hpre {
                pac::rcc::vals::Hpre::DIV1 => sysclk,
                pac::rcc::vals::Hpre::DIV2 => sysclk / 2,
                pac::rcc::vals::Hpre::DIV4 => sysclk / 4,
                pac::rcc::vals::Hpre::DIV8 => sysclk / 8,
                pac::rcc::vals::Hpre::DIV16 => sysclk / 16,
                pac::rcc::vals::Hpre::DIV64 => sysclk / 64,
                pac::rcc::vals::Hpre::DIV128 => sysclk / 128,
                pac::rcc::vals::Hpre::DIV256 => sysclk / 256,
                pac::rcc::vals::Hpre::DIV512 => sysclk / 512,
                _ => sysclk,
            };

            let apb1: u32 = match config.ppre1 {
                pac::rcc::vals::Ppre::DIV1 => ahb,
                pac::rcc::vals::Ppre::DIV2 => ahb / 2,
                pac::rcc::vals::Ppre::DIV4 => ahb / 4,
                pac::rcc::vals::Ppre::DIV8 => ahb / 8,
                pac::rcc::vals::Ppre::DIV16 => ahb / 16,
                _ => ahb,
            };

            let apb2: u32 = match config.ppre2 {
                pac::rcc::vals::Ppre::DIV1 => ahb,
                pac::rcc::vals::Ppre::DIV2 => ahb / 2,
                pac::rcc::vals::Ppre::DIV4 => ahb / 4,
                pac::rcc::vals::Ppre::DIV8 => ahb / 8,
                pac::rcc::vals::Ppre::DIV16 => ahb / 16,
                _ => ahb,
            };
                
            clocks.sysclk = sysclk;
            clocks.ahb = ahb;
            clocks.apb1 = apb1;
            clocks.apb2 = apb2;
        }
        
        Rcc { clocks }
    }
    
    pub fn clocks(&self) -> &Clocks {
        &self.clocks
    }
}