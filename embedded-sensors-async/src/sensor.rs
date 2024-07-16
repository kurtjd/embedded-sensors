//! Async Sensor API
//!
//! This module predominantly contains error-handling generic to all sensors,
//! however traits which apply to multiple kinds of sensors may be defined
//! here as well as needed.
//!
//! Please see specific sensor-type modules for example usage
//! (e.g. see temperature.rs for TemperatureSensor examples).

pub use embedded_sensors::sensor::{Error, ErrorKind, ErrorType, Sample};
