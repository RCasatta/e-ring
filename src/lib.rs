#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

//! A no_std append only ring buffer, when full new element replace oldest one

mod avg_std;
mod rescale;
mod ring;

#[cfg(feature = "hist")]
pub mod hist;

pub use ring::Ring;

pub use rescale::FindRange;
pub use rescale::Range;
