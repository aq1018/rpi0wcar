use std::{error, fmt};

pub type PwmError = rppal::pwm::Error;
pub type I2cError = rppal::i2c::Error;
pub type Pca9685Error = pwm_pca9685::Error<I2cError>;

#[derive(Debug)]
pub enum MotionError {
    PwmError(PwmError),
    I2cError(I2cError),
    Pca9685Error(Pca9685Error),

    InvalidInput(String),
}

impl fmt::Display for MotionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I2cError(err) => write!(f, "Motion Controller Error: {:?}: {})", self, err),
            Self::PwmError(err) => write!(f, "Motion Controller Error: {:?}: {})", self, err),
            Self::InvalidInput(err) => write!(f, "Motion Controller Error: {:?}: {})", self, err),
            Self::Pca9685Error(err) => match err {
                pwm_pca9685::Error::I2C(err) => {
                    write!(f, "Motion Controller Error: {:?}: {})", self, err)
                }
                pwm_pca9685::Error::InvalidInputData => {
                    write!(
                        f,
                        "Motion Controller Error: {:?}: Invalid Input Data)",
                        self
                    )
                }
            },
        }
    }
}

impl error::Error for MotionError {}

impl From<PwmError> for MotionError {
    fn from(e: PwmError) -> Self {
        Self::PwmError(e)
    }
}

impl From<I2cError> for MotionError {
    fn from(e: I2cError) -> Self {
        Self::I2cError(e)
    }
}

impl From<Pca9685Error> for MotionError {
    fn from(e: Pca9685Error) -> Self {
        Self::Pca9685Error(e)
    }
}
