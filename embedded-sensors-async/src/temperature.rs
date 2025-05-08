//! Async Temperature Sensor API
//!
//! This API provides generic methods for interfacing with temperature sensors specifically.
//!
//! # For HAL authors
//!
//! Here is an example for the implementation of the TemperatureSensor and TemperatureThresholdWait traits for a temperature sensor.
//!
//! ```
//! use embedded_sensors_hal_async::sensor;
//! use embedded_sensors_hal_async::temperature::{TemperatureSensor, DegreesCelsius};
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
//!     async fn temperature(&mut self) -> Result<DegreesCelsius, Self::Error> {
//!         // ...
//!         Ok(42.0)
//!     }
//! }
//!
//! impl TemperatureThresholdWait for MyTempSensor {
//!     async fn set_temperature_threshold_low(&mut self, threshold: DegreesCelsius) -> Result<(), Self::Error> {
//!         // Write value to threshold low register of sensor...
//!         Ok(())
//!     }
//!
//!     async fn set_temperature_threshold_high(&mut self, threshold: DegreesCelsius) -> Result<(), Self::Error> {
//!         // Write value to threshold high register of sensor...
//!         Ok(())
//!     }
//!
//!     async fn wait_for_temperature_threshold<ALERT: embedded_hal_async::digital::Wait>(&mut self, alert: ALERT) -> Result<DegreesCelsius, Self::Error> {
//!         // Await threshold alert (e.g. await GPIO level change on ALERT pin)...
//!         alert.wait_for_rising_edge().await.map_err(|_| ...)?;
//!         // Then return current temperature so caller can determine which threshold was crossed
//!         self.temperature().await
//!     }
//! }
//! ```

use crate::decl_threshold_wait;
use crate::sensor::ErrorType;
pub use embedded_sensors_hal::temperature::DegreesCelsius;

/// Async Temperature Sensor methods.
pub trait TemperatureSensor: ErrorType {
    /// Returns a temperature sample in degrees Celsius.
    async fn temperature(&mut self) -> Result<DegreesCelsius, Self::Error>;
}

impl<T: TemperatureSensor + ?Sized> TemperatureSensor for &mut T {
    #[inline]
    async fn temperature(&mut self) -> Result<DegreesCelsius, Self::Error> {
        T::temperature(self).await
    }
}

decl_threshold_wait!(
    Temperature,
    TemperatureSensor,
    DegreesCelsius,
    "degrees Celsius"
);
