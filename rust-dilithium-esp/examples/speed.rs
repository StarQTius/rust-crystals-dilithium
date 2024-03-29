#![no_std]
#![no_main]

use esp_idf_sys::esp_task_wdt_init;
use log::info;
use rust_dilithium_esp::{
    compute_hardware, compute_reference, compute_software, true_random_seed, Timer,
};

type Chronometer = Timer<0, 0>;

#[no_mangle]
#[allow(clippy::empty_loop)]
fn main() {
    const TRIALS_NB: usize = 1000;

    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    unsafe {
        esp_task_wdt_init(86400, false);
    }

    info!(file!());

    let software_time = {
        let chronometer = Chronometer::start();

        (0..TRIALS_NB).for_each(|_| {
            compute_software(&true_random_seed(), &true_random_seed());
        });
        chronometer.get()
    };
    info!(
        "Software perf: {software_time} for {TRIALS_NB} iterations ({}/it)",
        software_time / TRIALS_NB as u64
    );

    let hardware_time = {
        let chronometer = Chronometer::start();

        (0..TRIALS_NB).for_each(|_| {
            compute_hardware(&true_random_seed(), &true_random_seed());
        });
        chronometer.get()
    };
    info!(
        "Hardware perf: {hardware_time} for {TRIALS_NB} iterations ({}/it)",
        hardware_time / TRIALS_NB as u64
    );

    let reference_time = {
        let chronometer = Chronometer::start();

        (0..TRIALS_NB).for_each(|_| {
            compute_reference(&true_random_seed());
        });
        chronometer.get()
    };
    info!(
        "Reference perf: {reference_time} for {TRIALS_NB} iterations ({}/it)",
        reference_time / TRIALS_NB as u64
    );

    loop {}
}
