//! Blocking Temperature Sensor API
//!
//! This API provides generic methods for interfacing with temperature sensors specifically.
//!
//! # For HAL authors
//!
//! Here is an example for the implementation of the TemperatureSensor trait for a temperature sensor.
//!
//! ```
//! use embedded_sensors::sensor;
//! use embedded_sensors::temperature::TemperatureSensor;
//!
//! // A struct representing a temperature sensor.
//! pub struct MyTempSensor {
//!     // ...
//! }
//!
//! #[derive(Clone, Copy, Debug)]
//! pub enum Error {
//!     // ...
//! }
//!
//! impl sensor::Error for Error {
//!     fn kind(&self) -> sensor::ErrorKind {
//!         match *self {
//!             // ...
//!         }
//!     }
//! }
//!
//! impl sensor::ErrorType for MyTempSensor {
//!     type Error = Error;
//! }
//!
//! impl TemperatureSensor for MyTempSensor {
//!     fn temperature(&mut self) -> Result<sensor::Sample, Self::Error> {
//!         // ...
//!         Ok(42_000_000)
//!     }
//! }
//! ```

use crate::sensor::{ErrorType, Sample};

/// Blocking Temperature Sensor methods.
pub trait TemperatureSensor: ErrorType {
    /// Returns a temperature sample in microdegrees Celsius.
    fn temperature(&mut self) -> Result<Sample, Self::Error>;
}

impl<T: TemperatureSensor + ?Sized> TemperatureSensor for &mut T {
    #[inline]
    fn temperature(&mut self) -> Result<Sample, Self::Error> {
        T::temperature(self)
    }
}
