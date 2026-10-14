/// A trait that must be implemented to allow a type to be used in a [`SteppedCurve`](crate::curves::SteppedCurve)
pub trait SteppedKeyframe<T>: Clone {}

/// A trait that must be implemented to allow a type to be used in a [`PulseCurve`](crate::curves::PulseCurve)
pub trait PulseKeyframe<T>: Clone {}

/// A trait that must be implemented to allow a type to be used in a [`LinearCurve`](crate::curves::LinearCurve)
/// ## Simple implementation
/// ```rust
///
/// #[derive(Clone)]
/// pub struct Transform2d {
///    point_x: f32,
///    point_y: f32,
///}
///
/// impl bevy_state_curves::prelude::LinearKeyframe<Transform2d> for Transform2d {
///    fn lerp(&self, next_frame_state: &Transform2d, ratio: f64) -> Transform2d {
///        Transform2d {
///            point_x: self.point_x + (next_frame_state.point_x - self.point_x) * ratio as f32,
///            point_y: self.point_y + (next_frame_state.point_y - self.point_y) * ratio as f32,
///        }
///    }
///}
///
/// ```
///
/// ## Enum
/// ```rust
/// #[derive(Clone, PartialEq)]
/// enum Test{
///     A,
///     B(u32, f32),
///     C {
///         count: u32,
///         float: f32
///       }
///}
///
/// impl bevy_state_curves::prelude::LinearKeyframe<Test> for Test {
///     fn lerp(&self, next_frame_state: &Test, ratio: f64) -> Test {
///        match self {
///             Test::A => Test::A,
///             Test::B(field_0, field_1) => {
///                 let Test::B(count_other, float_other) = next_frame_state else {
///                     // If the next frame state is not the same state, then just return the last state
///                     return self.clone();
///                 };
///
///                 Test::B(
///                     (*field_0 as f64 + (*count_other as f64 - *field_0 as f64) * ratio) as u32,
///                     1.0,
///                 )
///             }
///             Test::C { count, float } => {
///                 let count_first = count;
///                 let float_first = float;
///                 let Test::C { count, float } = next_frame_state else {
///                     // If the next frame state is not the same state, then just return the last state
///                     return self.clone();
///                 };
///
///                 let count_other = count;
///                 let float_other = float;
///
///                 Test::C {
///                     count: (*count_first as f64
///                         + (*count_other as f64 - *count_first as f64) * ratio)
///                        as u32,
///                     float: (*float_first as f64
///                         + (*float_other as f64 - *float_first as f64) * ratio)
///                         as f32,
///                 }
///             }
///         }
///     }
/// }
///
/// ```
pub trait LinearKeyframe<T>: Clone {
    /// Performs a linear interpolation over the component based on the ratio between the tick that the linear keyframe is on and the tick the next keyframe is on.
    /// The implementation decides which fields are interpolated and returns the result for use in systems
    fn lerp(&self, next_frame_state: &T, ratio: f64) -> T;
}
