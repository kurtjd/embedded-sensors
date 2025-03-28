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
//!     async fn wait_for_temperature_low(&mut self, threshold: DegreesCelsius) -> Result<(), Self::Error> {
//!         // Enable lower threshold alerts for sensor...
//!         // Await alert (e.g. GPIO level change)...
//!         // Disable alerts for sensor...
//!
//!         Ok(())
//!     }
//!
//!     async fn wait_for_temperature_high(&mut self, threshold: DegreesCelsius) -> Result<(), Self::Error> {
//!         // Enable upper threshold alerts for sensor...
//!         // Await alert (e.g. await GPIO level change)...
//!         // Disable alerts for sensor...
//!
//!         Ok(())
//!     }
//!
//!     async fn wait_for_temperature_out_of_range(
//!         &mut self,
//!         threshold_low: DegreesCelsius,
//!         threshold_high: DegreesCelsius
//!     ) -> Result<(), Self::Error> {
//!         // Enable lower and upper threshold alerts for sensor...
//!         // Await alert (e.g. await GPIO level change)...
//!         // Disable alerts for sensor...
//!
//!         Ok(())
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

decl_threshold_wait!(Temperature, DegreesCelsius, "degrees Celsius");
