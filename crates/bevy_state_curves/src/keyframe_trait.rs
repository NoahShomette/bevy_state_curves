pub trait SteppedKeyframe<T>: Clone {}

pub trait PulseKeyframe<T>: Clone {}

pub trait LinearKeyframe<T>: Clone {
    /// Performs a linear interpolation over the component based on the ratio between starting tick and next_frame_tick.
    /// The implementation decides which fields are interpolated and returns the result for use in systems
    fn lerp(&self, next_frame_state: &T, ratio: f64) -> T;
}
