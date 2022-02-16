//! This provides implementations of the input/output pin [`embedded-hal`] traits with inverted logic.
//!
//! For example, an `InvertedOutputPin` wraps an `OutputPin` and when setting it low, it will set the
//! wrapped `OutputPin` high.
//!
//! This is useful when dealing with pins that use a logic that is inverted with respect to what
//! the rest of the system expects.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! ## Usage examples
//!
//! ### `InvertedOutputPin` interaction
//!
//! ```no_run
//! # use embedded_hal::digital::v2::OutputPin;
//! use linux_embedded_hal::Pin;
//! use inverted_pin::InvertedOutputPin;
//!
//! let pin = Pin::new(25);
//! let mut inverted_pin = InvertedOutputPin::new(pin);
//!
//! // This calls `pin.set_low()`
//! inverted_pin.set_high().unwrap();
//! ```
//!
//! ### `InvertedInputPin` interaction
//!
//! ```no_run
//! # use embedded_hal::digital::v2::InputPin;
//! use linux_embedded_hal::Pin;
//! use inverted_pin::InvertedInputPin;
//!
//! let pin = Pin::new(25);
//! let mut inverted_pin = InvertedInputPin::new(pin);
//!
//! // This returns the result of calling `pin.is_low()`
//! inverted_pin.is_high().unwrap();
//! ```

#![deny(unsafe_code, missing_docs)]
#![no_std]

mod inverted;
pub use crate::inverted::{InvertedInputPin, InvertedOutputPin};
