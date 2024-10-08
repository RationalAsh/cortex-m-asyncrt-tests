#![no_std]
#![no_main]

use cortex_m_asyncrt::os::{self, executor, init_heap, Task};
use cortex_m_rt::entry;
use cortex_m_semihosting::{dbg, hprintln};
// use panic_probe as _;
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    init_heap();
    hprintln!("Hello, worlds!");
    // New executor that can run up to 32 tasks
    let mut executor = executor::Executor::new::<64>();

    // Spawn a task
    executor.spawn(Task::new(example_task()));

    // Run the executor
    executor.run();

    // This code is unreachable because the executor.run() function runs tasks to completion.
    loop {}
}

async fn example_task() {
    // your code goes here
    let r = example_fn().await;

    hprintln!("r = {}", r);
}

async fn example_fn() -> u32 {
    42
}
