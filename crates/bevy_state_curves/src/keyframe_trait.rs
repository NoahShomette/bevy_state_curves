/// A trait that must be implemented to allow a type to be used in a [`SteppedCurve`](crate::curves::SteppedCurve)
pub trait SteppedKeyframe<T>: Clone {}

/// A trait that must be implemented to allow a type to be used in a [`PulseCurve`](crate::curves::PulseCurve)
pub trait PulseKeyframe<T>: Clone {}

/// A trait that must be implemented to allow a type to be used in a [`LinearCurve`](crate::curves::LinearCurve)
pub trait LinearKeyframe<T>: Clone {
    /// Performs a linear interpolation over the component based on the ratio between the tick that the linear keyframe is on and the tick the next keyframe is on.
    /// The implementation decides which fields are interpolated and returns the result for use in systems
    fn lerp(&self, next_frame_state: &T, ratio: f64) -> T;
}
