use gilrs::{Axis, Button, Event, EventType, GamepadId, Gilrs};
use std::{
    error, fmt,
    fmt::{Display, Formatter},
    sync::mpsc::{SendError, Sender},
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};

#[derive(Debug)]
pub enum ControllerError {
    GilrsError(String),
    GeneralError(String),
}

impl Display for ControllerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::GilrsError(msg) => {
                write!(f, "Motion Controller Error: {:?}: {})", self, msg)
            }

            Self::GeneralError(msg) => {
                write!(f, "Motion Controller Error: {:?}: {})", self, msg)
            }
        }
    }
}

impl error::Error for ControllerError {}

impl From<gilrs::Error> for ControllerError {
    fn from(e: gilrs::Error) -> Self {
        Self::GilrsError(e.to_string())
    }
}

impl From<SendError<ControllerEvent>> for ControllerError {
    fn from(e: SendError<ControllerEvent>) -> Self {
        Self::GeneralError(e.to_string())
    }
}

#[derive(Debug)]
pub enum ControllerEvent {
    Throttle(f64),
    Steer(f64),
    Connected,
    Disconnected,
}

pub fn process(tx: Sender<ControllerEvent>) -> JoinHandle<Result<(), ControllerError>> {
    spawn(move || -> Result<(), ControllerError> {
        let mut gilrs = Gilrs::new()?;
        let mut gamepad: Option<GamepadId> = None;

        loop {
            // process events
            while let Some(Event { id, event, .. }) = gilrs.next_event() {
                // we have a game pad, but the event is for another game pad, we ignore it.
                if gamepad != None && Some(id) != gamepad {
                    continue;
                }

                match event {
                    EventType::Connected => {
                        if gamepad == None {
                            gamepad = Some(id);
                            tx.send(ControllerEvent::Connected)?;
                        }
                    }

                    EventType::Disconnected => {
                        if gamepad == Some(id) {
                            gamepad = None;
                            tx.send(ControllerEvent::Disconnected)?;
                        }
                    }

                    EventType::AxisChanged(Axis::LeftStickX, v, ..) => {
                        if gamepad == None {
                            gamepad = Some(id)
                        }

                        if Some(id) == gamepad {
                            tx.send(ControllerEvent::Steer(v as f64))?;
                        }
                    }

                    EventType::ButtonChanged(Button::LeftTrigger2, v, ..) => {
                        if gamepad == None {
                            gamepad = Some(id)
                        }

                        if Some(id) == gamepad {
                            tx.send(ControllerEvent::Throttle((v * -1.0) as f64))?;
                        }
                    }

                    EventType::ButtonChanged(Button::RightTrigger2, v, ..) => {
                        if gamepad == None {
                            gamepad = Some(id)
                        }

                        if Some(id) == gamepad {
                            tx.send(ControllerEvent::Throttle(v as f64))?;
                        }
                    }

                    EventType::Dropped => {}

                    _ => {
                        if gamepad == None {
                            gamepad = Some(id)
                        }
                    }
                }
            }

            sleep(Duration::from_millis(10));
        }
    })
}
