use embedded_hal as hal;
use pwm_pca9685 as pca9685;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum MotorDirection {
    ClockWise,
    CounterClockWise,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum MotorChannel {
    A,
    B,
}

impl MotorChannel {
    pub fn throttle(&self) -> pca9685::Channel {
        match self {
            Self::A => pca9685::Channel::C0,
            Self::B => pca9685::Channel::C3,
        }
    }

    pub fn dir1(&self) -> pca9685::Channel {
        match self {
            Self::A => pca9685::Channel::C1,
            Self::B => pca9685::Channel::C4,
        }
    }

    pub fn dir2(&self) -> pca9685::Channel {
        match self {
            Self::A => pca9685::Channel::C2,
            Self::B => pca9685::Channel::C5,
        }
    }
}

#[derive(Debug)]
pub struct MotorDriver<I2C> {
    driver: pca9685::Pca9685<I2C>,
}

impl<I2C, E> MotorDriver<I2C>
where
    I2C: hal::blocking::i2c::Write<Error = E> + hal::blocking::i2c::WriteRead<Error = E>,
{
    pub fn new<A: Into<pca9685::Address>>(
        i2c: I2C,
        address: A,
        prescale: u8,
    ) -> Result<Self, pca9685::Error<E>> {
        let driver = pca9685::Pca9685::new(i2c, address)?;
        let mut rv = Self { driver };
        rv.set_prescale(prescale)?;

        Ok(rv)
    }

    pub fn set_prescale(&mut self, v: u8) -> Result<(), pca9685::Error<E>> {
        self.driver.set_prescale(v)
    }

    /// set direction
    pub fn set_dir(
        &mut self,
        channel: MotorChannel,
        dir: MotorDirection,
    ) -> Result<(), pca9685::Error<E>> {
        match dir {
            MotorDirection::ClockWise => {
                self.driver.set_channel_full_on(channel.dir1(), 0)?;
                self.driver.set_channel_full_off(channel.dir2())?;
            }
            MotorDirection::CounterClockWise => {
                self.driver.set_channel_full_off(channel.dir1())?;
                self.driver.set_channel_full_on(channel.dir2(), 0)?;
            }
        }
        Ok(())
    }

    // set throttle
    pub fn set_throttle(
        &mut self,
        channel: MotorChannel,
        throttle: u16,
    ) -> Result<(), pca9685::Error<E>> {
        self.driver
            .set_channel_on_off(channel.throttle(), 0, throttle)
    }
}
