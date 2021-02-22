use anyhow::*;

pub mod config;
pub mod controller;
pub mod error;
pub mod motion;

use crate::{
    config::Config,
    controller::{process, ControllerEvent},
    motion::Driver,
};

const CONFIG_PATH: &str = "/etc/rpi0wcar.json";

fn main() -> Result<()> {
    // load config
    let config = Config::from_file(CONFIG_PATH)?;

    // initialize hardware
    let mut driver = Driver::new(config)?;

    // initialize controller handling thread
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = process(tx);

    while let Ok(event) = rx.recv() {
        match event {
            ControllerEvent::Connected => {
                println!("Controller Connected!");
            }
            ControllerEvent::Disconnected => {
                println!("Controller Disconnected!");
            }
            ControllerEvent::Steer(v) => {
                driver.steer(v)?;
            }
            ControllerEvent::Throttle(v) => {
                driver.throttle(v)?;
            }
        }
    }

    handle.join().unwrap()?;

    Ok(())
}
