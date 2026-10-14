#![doc = include_str!("../../../README.md")]

mod curves;
mod keyframe_trait;
#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::curves::{CurveTrait, LinearCurve, PulseCurve, SteppedCurve};
    pub use super::keyframe_trait::{LinearKeyframe, PulseKeyframe, SteppedKeyframe};
    pub use super::GameTick;
    pub use derive_macros::{LinearKeyframe, PulseKeyframe, SteppedKeyframe};
}

/// An alias for a u64 representing the type used to drive the state curves.
pub type GameTick = u64;
/// An alias for a u64 representing the type used to drive the state curves.
pub type GameTicks = u64;
