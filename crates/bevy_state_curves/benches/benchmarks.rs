use bevy::{
    prelude::{Component, ReflectComponent},
    reflect::Reflect,
};
use bevy_state_curves::prelude::{
    CurveTrait, LinearCurve, LinearKeyframe, SteppedCurve, SteppedKeyframe,
};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
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
    object_state.rotation_point.insert_keyframe(
        0,
        BodyRotationPoint {
            point_x: 0.0,
            point_y: 0.0,
        },
    );
    c.bench_function("fib 20", |b| {
        b.iter(|| {
            let _angle: BodyAngle = object_state.angle.get_state(20).unwrap();
            let _radius: BodyRadius = object_state.radius.get_state(20).unwrap();
            let _rotation: BodyRotationPoint = object_state.rotation_point.get_state(20).unwrap();
            let _speed: BodySpeed = object_state.speed.get_state(20).unwrap();
            let _orbit: BodyOrbit = object_state.orbit.get_state(20).unwrap();
        })
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
}

/// This component tracks the current angle of the body
#[derive(Reflect, Clone, Component, Default, PartialEq, Debug)]
#[reflect(Component)]
pub struct BodyAngle {
    angle: f32,
}

impl LinearKeyframe<BodyAngle> for BodyAngle {
    fn lerp(&self, next_frame_state: &BodyAngle, ratio: f64) -> BodyAngle {
        BodyAngle {
            angle: self.angle + (next_frame_state.angle - self.angle) * ratio as f32,
        }

    }
}

/// This component tracks the current angle of the body
#[derive(Reflect, Clone, Component, Default, PartialEq, Debug)]
#[reflect(Component)]
pub struct BodyRadius {
    radius: f32,
}

impl LinearKeyframe<BodyRadius> for BodyRadius {
    fn lerp(&self, next_frame_state: &BodyRadius, ratio: f64) -> BodyRadius {
        BodyRadius {
            radius: self.radius + (next_frame_state.radius - self.radius) * ratio as f32,
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

impl LinearKeyframe<BodyRotationPoint> for BodyRotationPoint {
    fn lerp(&self, next_frame_state: &BodyRotationPoint, ratio: f64) -> BodyRotationPoint {
        BodyRotationPoint {
            point_x: self.point_x + (next_frame_state.point_x - self.point_x) * ratio as f32,
            point_y: self.point_y + (next_frame_state.point_y - self.point_y) * ratio as f32,
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
