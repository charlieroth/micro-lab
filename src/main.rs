#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{AnyPin, Input, Level, Output, OutputDrive, Pin, Pull};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::{Duration, Timer, WithTimeout};
use futures::{select_biased, FutureExt};
use micro_lab::{App, Button};
use panic_probe as _;

static CHANNEL: Channel<ThreadModeRawMutex, Button, 1> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Starting...");
    let p = embassy_nrf::init(Default::default());
    let mut app = App::new(p);
    let button_a = app.buttons.button_a;
    let button_b = app.buttons.button_b;
    spawner.spawn(button(button_a, Button::A)).unwrap();
    spawner.spawn(button(button_b, Button::B)).unwrap();
    loop {
        app.toggle();
        select_biased! {
            button = CHANNEL.receive().fuse() => {
                app.shift(button);
            }
            _ = Timer::after_millis(500).fuse() => {}
        }
    }
}

#[embassy_executor::task]
async fn button(mut input: Input<'static>, button: Button) {
    loop {
        input.wait_for_low().await;
        CHANNEL.send(button).await;
        Timer::after_millis(100).await;
        input.wait_for_high().await;
    }
}
