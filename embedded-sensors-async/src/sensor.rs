//! Async Sensor API
//!
//! This module contains traits generic to all sensors.
//!
//! Please see specific sensor-type modules for addtional example usage
//! (e.g. see temperature.rs for TemperatureSensor examples).

pub use embedded_sensors::sensor::{Error, ErrorKind, ErrorType};

/// Generates a threshold wait trait for the specified sensor type.
#[macro_export]
macro_rules! decl_threshold_wait {
    ($SensorName:ident, $SampleType:ty, $unit:expr) => {
        paste::paste! {
            #[doc = concat!(" Asynchronously wait for ", stringify!($SensorName:lower), " measurements to exceed specified thresholds.")]
            pub trait [<$SensorName ThresholdWait>]: ErrorType {
                #[doc = concat!(" Wait for ", stringify!($SensorName), " to be measured below the given threshold (in ", $unit, ").")]
                async fn [<wait_for_ $SensorName:lower _low>](&mut self, threshold: $SampleType) -> Result<(), Self::Error>;

                #[doc = concat!(" Wait for ", stringify!($SensorName), " to be measured above the given threshold (in ", $unit, ").")]
                async fn [<wait_for_ $SensorName:lower _high>](&mut self, threshold: $SampleType) -> Result<(), Self::Error>;

                #[doc = concat!(" Wait for ", stringify!($SensorName), " to be measured above or below the given high and low thresholds (in ", $unit, ").")]
                async fn [<wait_for_ $SensorName:lower _out_of_range>](
                    &mut self,
                    threshold_low: $SampleType,
                    threshold_high: $SampleType,
                ) -> Result<(), Self::Error>;
            }

            impl<T: [<$SensorName ThresholdWait>] + ?Sized> [<$SensorName ThresholdWait>] for &mut T {
                async fn [<wait_for_ $SensorName:lower _low>](&mut self, threshold: $SampleType) -> Result<(), Self::Error> {
                    T::[<wait_for_ $SensorName:lower _low>](self, threshold).await
                }

                async fn [<wait_for_ $SensorName:lower _high>](&mut self, threshold: $SampleType) -> Result<(), Self::Error> {
                    T::[<wait_for_ $SensorName:lower _high>](self, threshold).await
                }

                async fn [<wait_for_ $SensorName:lower _out_of_range>](
                    &mut self,
                    threshold_low: $SampleType,
                    threshold_high: $SampleType,
                ) -> Result<(), Self::Error> {
                    T::[<wait_for_ $SensorName:lower _out_of_range>](self, threshold_low, threshold_high).await
                }
            }
        }
    };
}
