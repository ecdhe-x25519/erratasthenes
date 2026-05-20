use erratasthenes::rcc::rcc::ClockEnable;
use erratasthenes::tim::{TimerConfig, TimerTim2};
use erratasthenes::tim::tim::general::Tim2;

fn main() {
    Tim2::enable();

    let tim_config: TimerConfig = TimerConfig::default();
    let timur: TimerTim2 = TimerTim2::new(tim_config, 1_000_000);
}