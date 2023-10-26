//! # `bevy_state_curves`
//!
//!Bevy_State_Curves is an implementation of the state storage and interpolation system described in this [blog post](https://www.forrestthewoods.com/blog/tech_of_planetary_annihilation_chrono_cam/). This system was used in Planetary Annihilation and has some neat features that arise for mostly free from it.
//!
//!The implementation of this crate is focused on compile time curves and integration with the ECS. Compile time curves were chosen for the performance factors while ECS because ECS everything.
//!
//!## Usage
//!
//!1. Create a new curve component
//!
//!```rust
//!#[derive(Reflect, Clone, Component, Default, PartialEq, Debug)]
//!#[reflect(Component)]
//!pub struct ObjectRadius {
//!    radius: f32,
//!}
//!
//!impl LinearKeyFrame<ObjectRadius> for ObjectRadius {
//!    fn lerp(&self, next_frame_state: &ObjectRadius, ratio: f64) -> ObjectRadius {
//!        ObjectRadius {
//!            radius: self.radius + (next_frame_state.radius - self.radius) * ratio as f32,
//!        }
//!    }
//!}
//!```
//!
//!2. Insert it into an entity
//!
//!```rust
//!     commands.entity(entity).insert(LinearCurve<ObjectRadius>::new());
//!```
//!
//!3. Add/remove keyframes using the curve as a normal component on an entity. Get state in a normal system using normal queries!
//!
//!```rust
//!    fn insert_keyframes(mut radius_query: Query<&mut LinearCurve<ObjectRadius>, tick: Res<CurrentGameTick>){
//!        for radius in radius_query.iter_mut(){
//!            radius.insert_keyframe(1, ObjectRadius{radius: 1.0});
//!            radius.insert_keyframe(10, ObjectRadius{radius: 2.0});
//!        }
//!    }
//!
//!    fn curve_entity_system(radius_query: Query<&LinearCurve<ObjectRadius>){
//!        for radius in radius_query.iter(){
//!            let radius_at_tick = body.get_state(5);
//!            assert_eq!(radius_at_tick, 1.5);
//!        }
//!    }
//!```
mod curves;
mod keyframe_trait;

use bevy::{
    prelude::{Component, Entity, Resource},
    utils::HashMap,
};

pub mod prelude {
    pub use super::curves::{CurveTrait, LinearCurve, PulseCurve, SteppedCurve};
    pub use super::keyframe_trait::{LinearKeyframe, PulseKeyframe, SteppedKeyframe};
    pub use super::GameTick;
}

/// An id that represents a single objects state
#[derive(Component)]
struct ObjectId(pub u32);

/// A resource that contains when each StateId existed
#[derive(Resource)]
struct ObjectTimings {
    object_timings: HashMap<GameTick, Vec<ObjectId>>,
}

/// A resource that contains a map from an [`ObjectId`] to an entity that holds all of that [`ObjectId`] curves
#[derive(Resource)]
struct ObjectStateCurveMap {
    object_state: HashMap<ObjectId, Entity>,
}

pub type GameTick = u64;
