//! Async Sensor API
//!
//! This module contains error-handling and traits generic to all sensors.
//!
//! # For HAL authors
//!
//! Here is an example implementation of the ThresholdWait trait for a generic sensor.
//!
//! Please see specific sensor-type modules for addtional example usage
//! (e.g. see temperature.rs for TemperatureSensor examples).
//!
//! ```
//! use embedded_sensors_async::sensor::{self, ThresholdWait, Sample};
//!
//! struct MySensor {
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
//! impl sensor::ErrorType for MySensor {
//!     type Error = Error;
//! }
//!
//! impl ThresholdWait for MySensor {
//!     async fn wait_for_sample_low(&mut self, threshold: Sample) -> Result<(), Self::Error> {
//!         // Enable lower threshold alerts for sensor...
//!         // Await alert (e.g. GPIO level change)...
//!         // Disable alerts for sensor...
//!
//!         Ok(())
//!     }
//!
//!     async fn wait_for_sample_high(&mut self, threshold: Sample) -> Result<(), Self::Error> {
//!         // Enable upper threshold alerts for sensor...
//!         // Await alert (e.g. await GPIO level change)...
//!         // Disable alerts for sensor...
//!
//!         Ok(())
//!     }
//!
//!     async fn wait_for_sample_out_of_range(
//!         &mut self,
//!         threshold_low: Sample,
//!         threshold_high: Sample
//!     ) -> Result<(), Self::Error> {
//!         // Enable lower and upper threshold alerts for sensor...
//!         // Await alert (e.g. await GPIO level change)...
//!         // Disable alerts for sensor...
//!
//!         Ok(())
//!     }
//! }
//! ```

pub use embedded_sensors::sensor::{Error, ErrorKind, ErrorType, Sample};

/// Asynchronously wait for sample measurements to exceed specified thresholds.
pub trait ThresholdWait: ErrorType {
    /// Wait for a sample to be measured below the given threshold.
    /// The threshold should be in the same units as the units returned by the sensor's sampling method
    /// (e.g. since the TemperatureSensor temperature() method returns microdegrees Celsius,
    /// supplied threshold should also be in microdegrees Celsius).
    async fn wait_for_sample_low(&mut self, threshold: Sample) -> Result<(), Self::Error>;

    /// Wait for a sample to be measured above the given threshold.
    /// The threshold should be in the same units as the units returned by the sensor's sampling method
    /// (e.g. since the TemperatureSensor temperature() method returns microdegrees Celsius,
    /// supplied threshold should also be in microdegrees Celsius).
    async fn wait_for_sample_high(&mut self, threshold: Sample) -> Result<(), Self::Error>;

    /// Wait for a sample to be measured below the given lower threshold or above the given upper threshold.
    /// The thresholds should be in the same units as the units returned by the sensor's sampling method
    /// (e.g. since the TemperatureSensor temperature() method returns microdegrees Celsius,
    /// supplied threshold should also be in microdegrees Celsius).
    async fn wait_for_sample_out_of_range(
        &mut self,
        threshold_low: Sample,
        threshold_high: Sample,
    ) -> Result<(), Self::Error>;
}
