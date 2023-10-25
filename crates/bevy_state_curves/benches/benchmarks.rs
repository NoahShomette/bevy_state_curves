use bevy::{
    prelude::{Component, ReflectComponent},
    reflect::Reflect,
};
use bevy_state_curves::{
    curves::{LinearCurve, SteppedCurve},
    keyframe_trait::{LinearKeyFrame, SteppedKeyframe},
    *,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut object_state_vec = Vec::with_capacity(1000);
    for i in 0..1000 {
        let mut object_state = BodyCurves::new();
        object_state
            .speed
            .insert_keyframe(0, BodySpeed { speed: 60 });
        object_state
            .radius
            .insert_keyframe(0, BodyRadius { radius: 50.0 });
        object_state
            .angle
            .insert_keyframe(0, BodyAngle { angle: 0.0 });
        object_state
            .angle
            .insert_keyframe(60, BodyAngle { angle: 6.0 });

        object_state
            .orbit
            .insert_keyframe(0, BodyOrbit { orbits: 0 });
        object_state
            .orbit
            .insert_keyframe(60, BodyOrbit { orbits: 1 });
        object_state_vec.push(object_state);
    }
    let mut object_state = BodyCurves::new();
    object_state
        .speed
        .insert_keyframe(0, BodySpeed { speed: 60 });
    object_state
        .radius
        .insert_keyframe(0, BodyRadius { radius: 50.0 });
    object_state
        .angle
        .insert_keyframe(0, BodyAngle { angle: 0.0 });
    object_state
        .angle
        .insert_keyframe(60, BodyAngle { angle: 6.0 });

    object_state
        .orbit
        .insert_keyframe(0, BodyOrbit { orbits: 0 });
    object_state
        .orbit
        .insert_keyframe(60, BodyOrbit { orbits: 1 });
    c.bench_function("fib 20", |b| {
        b.iter(|| object_state.get_object_state_for_tick(20))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

#[derive(Component)]
struct BodyCurves {
    angle: LinearCurve<BodyAngle>,
    radius: LinearCurve<BodyRadius>,
    rotation_point: LinearCurve<BodyRotationPoint>,
    speed: SteppedCurve<BodySpeed>,
    orbit: SteppedCurve<BodyOrbit>,
}

impl BodyCurves {
    fn new() -> BodyCurves {
        BodyCurves {
            angle: LinearCurve::new(),
            radius: LinearCurve::new(),
            rotation_point: LinearCurve::new(),
            speed: SteppedCurve::new(),
            orbit: SteppedCurve::new(),
        }
    }

    fn get_object_state_for_tick(&self, tick: GameTick) -> Vec<Box<dyn Reflect>> {
        let mut vec = Vec::with_capacity(5);
        vec.push(Box::new(self.angle.get_state(tick).unwrap()) as Box<dyn Reflect>);
        vec.push(Box::new(self.radius.get_state(tick).unwrap()) as Box<dyn Reflect>);
        //vec.push(Box::new(self.rotation_point.get_state(tick).unwrap()) as Box<dyn Reflect>);
        vec.push(Box::new(self.speed.get_state(tick).unwrap()) as Box<dyn Reflect>);
        vec.push(Box::new(self.orbit.get_state(tick).unwrap()) as Box<dyn Reflect>);
        vec
    }
}

/// This component tracks the current angle of the body
#[derive(Reflect, Clone, Component, Default, PartialEq, Debug)]
#[reflect(Component)]
pub struct BodyAngle {
    angle: f32,
}

impl LinearKeyFrame<BodyAngle> for BodyAngle {
    fn lerp(&self, next_frame_state: &BodyAngle, ratio: f32) -> BodyAngle {
        let Some(other_frame) = next_frame_state.self_as_any().downcast_ref::<Self>() else {
            panic!("Did not have a valid keyframe of the same type");
        };
        BodyAngle {
            angle: self.angle + (other_frame.angle - self.angle) * ratio as f32,
        }
    }
}

/// This component tracks the current angle of the body
#[derive(Reflect, Clone, Component, Default, PartialEq, Debug)]
#[reflect(Component)]
pub struct BodyRadius {
    radius: f32,
}

impl LinearKeyFrame<BodyRadius> for BodyRadius {
    fn lerp(&self, next_frame_state: &BodyRadius, ratio: f32) -> BodyRadius {
        let Some(other_frame) = next_frame_state.self_as_any().downcast_ref::<Self>() else {
            panic!("Did not have a valid keyframe of the same type");
        };
        BodyRadius {
            radius: self.radius + (other_frame.radius - self.radius) * ratio as f32,
        }
    }
}

/// This component tracks the position that this body is rotating around
#[derive(Reflect, Clone, Component, Default, PartialEq, Debug)]
#[reflect(Component)]
pub struct BodyRotationPoint {
    point_x: f32,
    point_y: f32,
}

impl LinearKeyFrame<BodyRotationPoint> for BodyRotationPoint {
    fn lerp(&self, next_frame_state: &BodyRotationPoint, ratio: f32) -> BodyRotationPoint {
        let Some(other_frame) = next_frame_state.self_as_any().downcast_ref::<Self>() else {
            panic!("Did not have a valid keyframe of the same type");
        };
        BodyRotationPoint {
            point_x: self.point_x + (other_frame.point_x - self.point_x) * ratio as f32,
            point_y: self.point_y + (other_frame.point_y - self.point_y) * ratio as f32,
        }
    }
}

/// This component represents how many ticks it takes to complete a full orbit
#[derive(Reflect, Clone, Component, Default, PartialEq, Debug)]
#[reflect(Component)]
pub struct BodySpeed {
    speed: u32,
}

impl SteppedKeyframe<BodySpeed> for BodySpeed {}

/// This component tracks how many orbits the object has done
#[derive(Reflect, Clone, Component, Default, PartialEq, Debug)]
#[reflect(Component)]
pub struct BodyOrbit {
    orbits: u32,
}

impl SteppedKeyframe<BodyOrbit> for BodyOrbit {}
