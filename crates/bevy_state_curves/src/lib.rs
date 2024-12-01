//! # `bevy_state_curves`
//!
//! Bevy_State_Curves is an implementation of the state storage and interpolation system described in this [blog post](https://www.forrestthewoods.com/blog/tech_of_planetary_annihilation_chrono_cam/). This system was used in Planetary Annihilation and has some neat features that arise for mostly free from it.
//!
//! The implementation of this crate is focused on compile time curves and integration with the ECS. Compile time curves were chosen for the performance factors while ECS because ECS everything.
//!
//! ## Usage
//!
//! 1. Create a new curve keyframe type
//!
//! ```rust ignore
//! /// Only Clone is needed for the CurveKeyframes. I also recommend `Component` as it is
//! /// an ergonomic way to handle having the current interpolated state be the state thats
//! /// on the entity physically
//! #[derive(Clone)]
//! pub struct ObjectRadius {
//!     radius: f32,
//! }
//!
//! impl LinearKeyframe<ObjectRadius> for ObjectRadius {
//!     fn lerp(&self, next_frame_state: &ObjectRadius, ratio: f64) -> ObjectRadius {
//!         ObjectRadius {
//!             radius: self.radius + (next_frame_state.radius - self.radius) * ratio as f32,
//!         }
//!     }
//! }
//!
//! /// Generally only implement one Keyframe type for a specific type of state.
//! /// Nothing technically stops you from doing all three for one but theres absolutely no reason to do that.
//! impl SteppedKeyframe<ObjectRadius> for ObjectRadius {}
//!
//! impl PulseKeyframe<ObjectRadius> for ObjectRadius {}
//!
//! ```
//!
//! 2. Insert it into an entity using the right curve component type for your curve type. `LinearCurve<ObjectRadius>`, `PulseCurve<ObjectRadius>`, or `SteppedCurve<ObjectRadius>`.
//!
//! ```rust ignore
//!      commands.entity(entity).insert(LinearCurve::<ObjectRadius>::new());
//! ```
//!
//! 3. Add/remove keyframes using the curve as a normal component on an entity. Get state in a normal system using normal queries!
//!
//! ```rust ignore
//!     fn insert_keyframes(mut radius_query: Query<&mut LinearCurve<ObjectRadius>){
//!         for radius in radius_query.iter_mut(){
//!             radius.insert_keyframe(1, ObjectRadius{radius: 1.0});
//!             radius.insert_keyframe(10, ObjectRadius{radius: 2.0});
//!         }
//!     }
//!
//!     fn curve_entity_system(radius_query: Query<&LinearCurve<ObjectRadius>){
//!         for radius in radius_query.iter(){
//!             let radius_at_tick = radius.get_state(5);
//!             assert_eq!(radius_at_tick, 1.5);
//!         }
//!     }
//! ```
//!
//! ### GameTick
//!
//! This crate relies on an internal `GameTick` type alias of a u64 to create a sense of time. Higher ticks are considered later/older chronologically than lower numbered ticks. When using this crate you will have to decide what the ticks will look like in your crate and implement those systems yourself. This crate does nothing with `GameTick` other than provide it and use it as an identifier of state keyframes.
//!
//! See the [solar_system.rs](https://github.com/NoahShomette/bevy_state_curves/blob/main/crates/bevy_state_curves/examples/solar_system.rs) example for an example of using `GameTick` in a game concept.
//!
//! ## Curves
//!
//! This crate supports three types of curves. See the docs.rs documentation for each one for details on how they work. Each of these is a Bevy Component.
//!
//! - `LinearCurve<T: LinearKeyFrame>`
//!   - Linearly interpolates state between each keyframe on either side of it.
//! - `SteppedCurve<T: SteppedKeyFrame>`
//!   - Flat state between keyframes, state is always the same as the last keyframe.
//! - `PulseCurve<T: PulseKeyFrame>`
//!   - Keyframes are only valid on the tick that they exist on.

mod curves;
mod keyframe_trait;

pub mod prelude {
    pub use super::curves::{CurveTrait, LinearCurve, PulseCurve, SteppedCurve};
    pub use super::keyframe_trait::{LinearKeyframe, PulseKeyframe, SteppedKeyframe};
    pub use super::GameTick;
}

/// An alias for a u64 representing the type used to drive the state curves.
pub type GameTick = u64;
