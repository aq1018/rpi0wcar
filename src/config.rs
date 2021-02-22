use anyhow::Result;
use rppal as pi;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::motion::waveshare_motor_driver::MotorChannel;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PwmChannel {
    Pwm0,
    Pwm1,
}

impl Default for PwmChannel {
    fn default() -> Self {
        PwmChannel::Pwm0
    }
}

impl Into<pi::pwm::Channel> for PwmChannel {
    fn into(self) -> pi::pwm::Channel {
        match self {
            Self::Pwm0 => pi::pwm::Channel::Pwm0,
            Self::Pwm1 => pi::pwm::Channel::Pwm1,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Physical {
    pub steer_min_angle: f64,
    pub steer_max_angle: f64,
}

impl Default for Physical {
    fn default() -> Self {
        Self {
            steer_min_angle: -30_f64,
            steer_max_angle: 30_f64,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Servo {
    pub pwm_channel: PwmChannel,
    pub offset_duty: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Motor {
    pub channel: MotorChannel,
    pub i2c_address: u8,
    pub prescale: u8,
}

impl Default for Motor {
    fn default() -> Self {
        Self {
            channel: MotorChannel::A,
            i2c_address: 0x40,
            prescale: 100,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Config {
    pub physical: Physical,
    pub servo: Servo,
    pub motor: Motor,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Config> {
        let config = serde_json::from_reader(BufReader::new(File::open(path)?))?;
        Ok(config)
    }
}
