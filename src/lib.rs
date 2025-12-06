//! This library provides an simple, lightweight, and efficient (though not heavily optimized) implementation of the Black-Scholes-Merton model for pricing European options.
//!
//! Provides methods for pricing options, calculating implied volatility, and calculating the first, second, and third order Greeks.
//!
//! ### Example:
//! ```
//! use quant_opts::{BlackScholes, MarketData, OptionStyle, OptionType, VanillaOption};
//!
//! let option = VanillaOption::new(
//!     OptionStyle::European,
//!     OptionType::Call,
//!     100.0,
//!     20.0 / 365.25,
//! );
//! let market = MarketData::new(100.0, 0.05, 0.2);
//! let price: f64 = BlackScholes::price(&option, &market, 0.2).unwrap();
//! ```
//!
//! Criterion benchmark can be ran by running:
//! ```bash
//! cargo bench
//! ```
//!
//! See the [Github Repo](https://github.com/hayden4r4/blackscholes-rust/tree/master) for full source code.  Other implementations such as a [npm WASM package](https://www.npmjs.com/package/@haydenr4/blackscholes_wasm) and a [python module](https://pypi.org/project/blackscholes/) are also available.

pub mod core;
pub mod models;

pub mod lets_be_rational;

pub use core::{MarketData, OptionStyle, OptionType, VanillaOption};

pub use models::{VanillaModel, black_scholes::BlackScholes};

#[cfg(feature = "wrappers")]
pub mod wrappers;

pub(crate) const DAYS_PER_YEAR: f64 = 365.25;

pub(crate) const A: f64 = 4.626_275_3e-1;
pub(crate) const B: f64 = -1.168_519_2e-2;
pub(crate) const C: f64 = 9.635_418_5e-4;
pub(crate) const D: f64 = 7.535_022_5e-5;
pub(crate) const _E: f64 = 1.424_516_45e-5;
pub(crate) const F: f64 = -2.102_376_9e-5;
