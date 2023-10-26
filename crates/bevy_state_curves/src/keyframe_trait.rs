use std::any::Any;

use bevy::reflect::Reflect;

/// A keyframe 
pub trait SteppedKeyframe<T>: Reflect + Clone {}

pub trait PulseKeyframe<T>: Reflect + Clone {}

/// Lerp should be given the last keyframe, the next keyframe, and then it returns the result for the requested [`GameTick`].
pub trait LinearKeyFrame<T>: Reflect + Clone {
    /// Performs a linear interpolation over the component based on the ratio between starting tick and next_frame_tick.
    /// The implementation decides which fields are interpolated and returns the result for use in systems
    fn lerp(&self, next_frame_state: &T, ratio: f64) -> T;

    fn self_as_any(&self) -> &dyn Any {
        self
    }
}
