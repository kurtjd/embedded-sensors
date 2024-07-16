//! Blocking Sensor API
//!
//! This module predominantly contains error-handling generic to all sensors,
//! however traits which apply to multiple kinds of sensors may be defined
//! here as well as needed.
//!
//! Please see specific sensor-type modules for example usage
//! (e.g. see temperature.rs for TemperatureSensor examples).

/// The underlying type representing a sensor sample.
/// Sampling methods should return samples in finer units
/// (e.g. microdegrees Celsius instead of degrees Celsius)
/// to maintain precision if necessary.
pub type Sample = i64;

/// Sensor error.
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic Sensor error kind.
    ///
    /// By using this method, Sensor errors freely defined by HAL implementations
    /// can be converted to a set of generic Sensor errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    #[inline]
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

/// Sensor error kind.
///
/// This represents a common set of Sensor operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common Sensor errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ErrorKind {
    /// An error occurred on the underlying peripheral supporting the sensor.
    /// e.g. An I2C error occurs for a digital sensor or an ADC error occurs for an analog sensor.
    /// The original error may contain more information.
    Peripheral,
    /// The sensor is not yet ready to be sampled.
    NotReady,
    /// The sensor is currently saturated and sample may be invalid.
    Saturated(Sample),
    /// A different error occurred. The original error may contain more information.
    Other,
}

impl Error for ErrorKind {
    #[inline]
    fn kind(&self) -> ErrorKind {
        *self
    }
}

impl core::fmt::Display for ErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Peripheral => write!(
                f,
                "An error occured on the underlying peripheral. The original error may contain more informaton"
            ),
            Self::NotReady => write!(f, "Sensor is not yet ready to be sampled"),
            Self::Saturated(_) => write!(f, "Sensor is saturated thus sample may be invalid"),
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}

/// Sensor error type trait.
///
/// This just defines the error type, to be used by the other traits.
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}
