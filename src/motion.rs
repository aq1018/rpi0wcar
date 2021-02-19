use rppal as pi;

pub mod servo_driver;
pub mod waveshare_motor_driver;

use crate::{
    config::Config,
    error::MotionError,
    motion::{
        servo_driver::{ServoDriver, SERVO_PWM_PERIOD},
        waveshare_motor_driver::{MotorDirection::*, MotorDriver},
    },
};

#[derive(Debug)]
pub struct Driver {
    servo: ServoDriver<pi::pwm::Pwm>,
    motor: MotorDriver<pi::i2c::I2c>,
    config: Config,
}

impl Driver {
    pub fn new(config: Config) -> Result<Self, MotionError> {
        // create servo
        let pwm = pi::pwm::Pwm::with_period(
            config.servo.pwm_channel.into(),
            SERVO_PWM_PERIOD,
            core::time::Duration::from_millis(0),
            pi::pwm::Polarity::Normal,
            true,
        )?;
        let servo = ServoDriver::new(pwm, config.servo.offset_duty);

        // create motor
        let i2c = pi::i2c::I2c::new()?;
        let motor = MotorDriver::new(i2c, config.motor.i2c_address, config.motor.prescale)?;

        Ok(Self {
            servo,
            motor,
            config,
        })
    }

    pub fn throttle(&mut self, v: f64) -> Result<(), MotionError> {
        if !(-1.0..=1.0).contains(&v) {
            return Err(MotionError::InvalidInput(
                "throttle must be set between -1.0 and 1.0.".to_string(),
            ));
        }

        let channel = self.config.motor.channel;

        if v > 0_f64 {
            self.motor.set_dir(channel, ClockWise)?;
        } else if v < 0_f64 {
            self.motor.set_dir(channel, CounterClockWise)?;
        }

        let v = (4095_f64 * v.abs()) as u16;
        self.motor.set_throttle(channel, v)?;

        Ok(())
    }

    pub fn steer(&mut self, v: f64) -> Result<(), MotionError> {
        if !(-1.0..=1.0).contains(&v) {
            return Err(MotionError::InvalidInput(
                "steering must be set between -1.0 and 1.0.".to_string(),
            ));
        }

        let min_angle = self.config.physical.steer_min_angle;
        let max_angle = self.config.physical.steer_max_angle;
        let angle = (max_angle - min_angle) / 2.0 * v;

        self.servo.set_angle(90.0 + angle);

        Ok(())
    }
}
