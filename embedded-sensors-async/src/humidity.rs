//! Async Humidity Sensor API
//!
//! This API provides generic methods for interfacing with humidity sensors specifically.
//!
//! # For HAL authors
//!
//! Here is an example for the implementation of the RelativeHumiditySensor
//! and RelativityHumidityThresholdWait traits for a humidity sensor.
//!
//! ```
//! use embedded_sensors_hal_async::sensor;
//! use embedded_sensors_hal_async::humidity::{RelativeHumiditySensor, Percentage};
//!
//! // A struct representing a humidity sensor.
//! pub struct MyHumiditySensor {
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
//! impl sensor::ErrorType for MyHumiditySensor {
//!     type Error = Error;
//! }
//!
//! impl RelativeHumiditySensor for MyHumiditySensor {
//!     async fn relative_humidity(&mut self) -> Result<Percentage, Self::Error> {
//!         // ...
//!         Ok(42.0)
//!     }
//! }
//!
//! impl RelativeHumidityThresholdWait for MyHumiditySensor {
//!     async fn wait_for_relative_humidity_low(
//!         &mut self,
//!         threshold: Percentage)
//!     -> Result<(), Self::Error> {
//!         // Enable lower threshold alerts for sensor...
//!         // Await alert (e.g. GPIO level change)...
//!         // Disable alerts for sensor...
//!
//!         Ok(())
//!     }
//!
//!     async fn wait_for_relative_humidity_high(
//!         &mut self,
//!         threshold: Percentage)
//!     -> Result<(), Self::Error> {
//!         // Enable upper threshold alerts for sensor...
//!         // Await alert (e.g. await GPIO level change)...
//!         // Disable alerts for sensor...
//!
//!         Ok(())
//!     }
//!
//!     async fn wait_for_relative_humidity_out_of_range(
//!         &mut self,
//!         threshold_low: Percentage,
//!         threshold_high: Percentage
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
pub use embedded_sensors_hal::humidity::Percentage;

/// Async Relative Humidity Sensor methods.
pub trait RelativeHumiditySensor: ErrorType {
    /// Returns a relative humidity (RH) sample as a percentage.
    async fn relative_humidity(&mut self) -> Result<Percentage, Self::Error>;
}

impl<T: RelativeHumiditySensor + ?Sized> RelativeHumiditySensor for &mut T {
    #[inline]
    async fn relative_humidity(&mut self) -> Result<Percentage, Self::Error> {
        T::relative_humidity(self).await
    }
}

decl_threshold_wait!(RelativeHumidity, Percentage, "percentage");
