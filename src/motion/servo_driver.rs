use core::time::Duration;

use embedded_hal as hal;
use num_traits::{AsPrimitive, FromPrimitive};

// 20 milliseconds or 50 Hertz
pub const SERVO_PWM_PERIOD: Duration = Duration::from_millis(20);

// min duty cycle is 1ms or 5% of period
pub const MIN_ANGLE_DUTY: f64 = 0.05;

// max duty cycle is 2ms or 10% of period
pub const MAX_ANGLE_DUTY: f64 = 0.1;

// min angle is 0 degrees
pub const MIN_ANGLE: f64 = 0.0;

// max angle is 180 degrees
pub const MAX_ANGLE: f64 = 180.0;

/// SG90 servo
#[derive(Debug)]
pub struct ServoDriver<PWM>
where
    PWM: hal::PwmPin,
    PWM::Duty: FromPrimitive + AsPrimitive<f64>,
{
    pwm: PWM,
    angle: f64,
    min_angle: f64,
    max_angle: f64,
    min_angle_duty: PWM::Duty,
    max_angle_duty: PWM::Duty,
    offset_duty: PWM::Duty,
}

impl<PWM> ServoDriver<PWM>
where
    PWM: hal::PwmPin,
    PWM::Duty: FromPrimitive + AsPrimitive<f64>,
{
    /// Creates a new servo
    pub fn new(pwm: PWM, offset_duty: PWM::Duty) -> Self {
        let duty: f64 = pwm.get_max_duty().as_();
        let min_angle_duty = PWM::Duty::from_f64(duty * MIN_ANGLE_DUTY).unwrap();
        let max_angle_duty = PWM::Duty::from_f64(duty * MAX_ANGLE_DUTY).unwrap();

        Self {
            pwm,
            min_angle_duty,
            max_angle_duty,
            offset_duty,
            angle: 0_f64,
            min_angle: MIN_ANGLE,
            max_angle: MAX_ANGLE,
        }
    }

    pub fn enable(&mut self) -> &mut Self {
        self.pwm.enable();
        self
    }

    pub fn disable(&mut self) -> &mut Self {
        self.pwm.disable();
        self
    }

    pub fn set_angle(&mut self, angle: f64) -> &mut Self {
        let angle = match angle {
            a if a > self.max_angle => self.max_angle,
            a if a < self.min_angle => self.min_angle,
            _ => angle,
        };

        let duty = self.angle_to_duty(angle);

        self.pwm.set_duty(duty);
        self.angle = angle;
        self
    }

    pub fn set_offset_duty(&mut self, offset_duty: PWM::Duty) -> &mut Self {
        self.offset_duty = offset_duty;
        self
    }

    pub fn angle(self) -> f64 {
        self.angle
    }

    pub fn min_angle(&self) -> f64 {
        self.min_angle
    }

    pub fn max_angle(&self) -> f64 {
        self.max_angle
    }

    pub fn min_angle_duty(&self) -> PWM::Duty {
        self.min_angle_duty
    }

    pub fn max_angle_duty(&self) -> PWM::Duty {
        self.max_angle_duty
    }

    pub fn offset_duty(&self) -> PWM::Duty {
        self.max_angle_duty
    }

    fn angle_to_duty(&self, angle: f64) -> PWM::Duty {
        let min_angle_duty: f64 = self.min_angle_duty.as_();
        let max_angle_duty: f64 = self.max_angle_duty.as_();
        let offset_duty: f64 = self.offset_duty.as_();
        let duty_per_angle: f64 =
            (max_angle_duty - min_angle_duty) / (self.max_angle - self.min_angle);
        let duty = min_angle_duty + offset_duty + duty_per_angle * angle;

        PWM::Duty::from_f64(duty).unwrap()
    }
}
