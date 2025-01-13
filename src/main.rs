#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_nrf::{
    bind_interrupts,
    gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull},
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, signal::Signal};
use embassy_time::{Duration, Timer, WithTimeout};
use panic_probe as _;

#[derive(Clone, Copy)]
enum Button {
    A,
    B,
}

static SIGNAL: Signal<ThreadModeRawMutex, Button> = Signal::new();

bind_interrupts!(struct Irqs {
    TEMP => embassy_nrf::temp::InterruptHandler;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Starting...");
    let p = embassy_nrf::init(Default::default());
    spawner.spawn(led_display_task(p.P0_22.degrade())).unwrap();
    let button_a = button(p.P0_14.degrade(), "A", Button::A);
    let button_b = button(p.P0_23.degrade(), "B", Button::B);
    join(button_a, button_b).await;
}

async fn button(pin: AnyPin, name: &str, b: Button) {
    let mut button = Input::new(pin, Pull::None);
    loop {
        button.wait_for_low().await;
        info!("Button {} pressed", name);
        SIGNAL.signal(b);
        Timer::after_millis(200).await;
        button.wait_for_high().await;
    }
}

#[embassy_executor::task]
async fn led_display_task(left: AnyPin) {
    let mut left_led = Output::new(left, Level::Low, OutputDrive::Standard);
    let delay = Duration::from_millis(100);
    loop {
        if let Some(b) = SIGNAL.wait().with_timeout(delay).await.ok() {
            match b {
                Button::A => {
                    left_led.set_low();
                    Timer::after_secs(1).await;
                    left_led.set_high();
                }
                Button::B => {
                    Timer::after_millis(100).await;
                }
            }
        }
    }
}
