#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]
#![allow(async_fn_in_trait)]

pub mod humidity;
pub mod sensor;
pub mod temperature;
