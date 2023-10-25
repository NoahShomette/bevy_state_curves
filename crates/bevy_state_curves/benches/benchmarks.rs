use bevy::{
    prelude::{Component, ReflectComponent},
    reflect::Reflect,
};
use bevy_state_curves::{
    keyframe_trait::{LinearKeyFrame, SteppedKeyframe},
    ObjectState,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut object_state = ObjectState::new(0);
    object_state.add_stepped_keyframe("BodySpeed", 0, BodySpeed { speed: 60 });
    object_state.add_linear_keyframe("BodyRadius", 0, BodyRadius { radius: 50.0 });

    object_state.add_linear_keyframe("BodyAngle", 0, BodyAngle { angle: 0.0 });
    object_state.add_linear_keyframe("BodyAngle", 60, BodyAngle { angle: 6.0 });

    object_state.add_stepped_keyframe("BodyOrbit", 0, BodyOrbit { orbits: 0 });
    object_state.add_stepped_keyframe("BodyOrbit", 60, BodyOrbit { orbits: 1 });
    c.bench_function("fib 20", |b| {
        b.iter(|| object_state.get_object_state_for_tick(20))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

/// This component tracks the current angle of the body
#[derive(Reflect, Clone, Component, Default, PartialEq, Debug)]
#[reflect(Component)]
pub struct BodyAngle {
    angle: f32,
}

impl LinearKeyFrame for BodyAngle {
    fn lerp(
        &self,
        next_frame_state: &Box<dyn LinearKeyFrame>,
        ratio: f32,
    ) -> Box<dyn LinearKeyFrame> {
        let Some(other_frame) = next_frame_state.self_as_any().downcast_ref::<Self>() else {
            panic!("Did not have a valid keyframe of the same type");
        };
        Box::new(BodyAngle {
            angle: self.angle + (other_frame.angle - self.angle) * ratio as f32,
        })
    }
}

/// This component tracks the current angle of the body
#[derive(Reflect, Clone, Component, Default, PartialEq, Debug)]
#[reflect(Component)]
pub struct BodyRadius {
    radius: f32,
}

impl LinearKeyFrame for BodyRadius {
    fn lerp(
        &self,
        next_frame_state: &Box<dyn LinearKeyFrame>,
        ratio: f32,
    ) -> Box<dyn LinearKeyFrame> {
        let Some(other_frame) = next_frame_state.self_as_any().downcast_ref::<Self>() else {
            panic!("Did not have a valid keyframe of the same type");
        };
        Box::new(BodyRadius {
            radius: self.radius + (other_frame.radius - self.radius) * ratio as f32,
        })
    }
}

/// This component tracks the position that this body is rotating around
#[derive(Reflect, Clone, Component, Default, PartialEq, Debug)]
#[reflect(Component)]
pub struct BodyRotationPoint {
    point_x: f32,
    point_y: f32,
}

impl LinearKeyFrame for BodyRotationPoint {
    fn lerp(
        &self,
        next_frame_state: &Box<dyn LinearKeyFrame>,
        ratio: f32,
    ) -> Box<dyn LinearKeyFrame> {
        let Some(other_frame) = next_frame_state.self_as_any().downcast_ref::<Self>() else {
            panic!("Did not have a valid keyframe of the same type");
        };
        Box::new(BodyRotationPoint {
            point_x: self.point_x + (other_frame.point_x - self.point_x) * ratio as f32,
            point_y: self.point_y + (other_frame.point_y - self.point_y) * ratio as f32,
        })
    }
}

/// This component represents how many ticks it takes to complete a full orbit
#[derive(Reflect, Clone, Component, Default, PartialEq, Debug)]
#[reflect(Component)]
pub struct BodySpeed {
    speed: u32,
}

impl SteppedKeyframe for BodySpeed {}

/// This component tracks how many orbits the object has done
#[derive(Reflect, Clone, Component, Default, PartialEq, Debug)]
#[reflect(Component)]
pub struct BodyOrbit {
    orbits: u32,
}

impl SteppedKeyframe for BodyOrbit {}
