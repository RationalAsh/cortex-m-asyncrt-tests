#![no_std]
#![no_main]

use cortex_m_asyncrt::os::{self, executor, Task};
use cortex_m_rt::entry;
use panic_probe as _;

#[entry]
fn main() -> ! {
    // New executor that can run up to 32 tasks
    let mut executor = executor::Executor::new::<32>();

    // Spawn a task
    executor.spawn(Task::new(example_task()));

    // Run the executor
    executor.run();

    // This code is unreachable because the executor.run() function runs tasks to completion.
    loop {}
}

async fn example_task() {
    // your code goes here
    example_fn().await;
}

async fn example_fn() -> u32 {
    42
}

// use embassy_executor::Spawner;
// use embassy_stm32::gpio::{Level, Output, Speed};
// use embassy_time::{Duration, Ticker};
// use libm::sinf;
// use panic_probe as _;

// fn clock_config() -> embassy_stm32::Config {
//     let mut config = embassy_stm32::Config::default();

//     // Configure to use the internal oscillator (HSI) at 16 MHz
//     config.rcc.hse = Hse::Oscillator(16_000_000);

//     // Configure to use the high speed internal oscillator (HSI

//     config
// }

// #[embassy_executor::main]
// async fn main(_spawner: Spawner) {
//     // Initialize embassy
//     let peripherals = embassy_stm32::init(clock_config());

//     // Create a new output pin - PA9 is the green led on the Discovery board
//     let mut green_led = Output::new(peripherals.PA9, Level::High, Speed::VeryHigh);
//     let mut red_led = Output::new(peripherals.PD5, Level::High, Speed::VeryHigh);
//     let mut green_led2 = Output::new(peripherals.PD12, Level::High, Speed::VeryHigh);
//     let mut orange_led = Output::new(peripherals.PD13, Level::High, Speed::VeryHigh);
//     let mut red_led2 = Output::new(peripherals.PD14, Level::High, Speed::VeryHigh);
//     let mut blue_led = Output::new(peripherals.PD15, Level::High, Speed::VeryHigh);

//     // Create a new Ticker for the delay
//     let mut ticker = Ticker::every(Duration::from_millis(100));
//     let mut ctr: f32 = 0.0;
//     let mut sin: f32 = 0.0;

//     loop {
//         // Wait for the ticker to expire
//         ticker.next().await;

//         ctr += 0.1f32;
//         ctr *= 0.1f32;
//         ctr /= 0.1f32;

//         // sin = sinf(ctr as f32) * 100.0f32;

//         // Toggle the leds
//         if (sin > 50.0f32) {
//             green_led.toggle();
//         }
//         red_led.toggle();
//         green_led2.toggle();
//         orange_led.toggle();
//         red_led2.toggle();
//         blue_led.toggle();
//     }
// }
