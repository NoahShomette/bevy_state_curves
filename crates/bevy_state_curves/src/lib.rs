mod curves;
mod keyframe_trait;

use bevy::{
    prelude::{Component, Entity, Resource},
    utils::HashMap,
};

pub mod prelude {
    pub use super::GameTick;
    pub use super::curves::{CurveTrait, LinearCurve, PulseCurve, SteppedCurve};
    pub use super::keyframe_trait::{LinearKeyFrame, PulseKeyframe, SteppedKeyframe};
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
