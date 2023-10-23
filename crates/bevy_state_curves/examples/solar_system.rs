use bevy::{
    prelude::{App, Camera2dBundle, Commands, Component, ReflectComponent, Startup},
    reflect::Reflect,
    DefaultPlugins,
};
use bevy_state_curves::{
    keyframe_trait::{LerpCloneTrait, LinearKeyFrame, SteppedKeyframe},
    CurveKeyFrame, GameTick, ObjectState,
};

extern crate bevy_state_curves;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, setup);
    app.register_type::<BodyAngle>();
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(BodyAngle { angle: 20.0 });

    let mut object_state = ObjectState::new(1);

    object_state.add_keyframe(
        "BodyAngle",
        0,
        CurveKeyFrame::Linear(Box::new(BodyAngle { angle: 20.0 })),
    );

    object_state.add_keyframe(
        "BodyAngle",
        2,
        CurveKeyFrame::Linear(Box::new(BodyAngle { angle: 40.0 })),
    );

    let curve = object_state.get_curves("BodyAngle").unwrap();
    let calculated = curve.1.get_state(5).unwrap();

    let CurveKeyFrame::Linear(calculated) = calculated else {
        panic!("NO");
    };

    let Some(other_frame) = calculated.self_as_any().downcast_ref::<BodyAngle>() else {
        panic!("Did not have a valid keyframe of the same type");
    };

    assert_eq!(other_frame, &BodyAngle { angle: 40.0 });

    object_state.add_keyframe(
        "BodyOrbits",
        0,
        CurveKeyFrame::Stepped(Box::new(BodyOrbit { orbits: 0 })),
    );

    object_state.add_keyframe(
        "BodyOrbits",
        2,
        CurveKeyFrame::Stepped(Box::new(BodyOrbit { orbits: 1 })),
    );

    let curve = object_state.get_curves("BodyOrbits").unwrap();
    let calculated = curve.1.get_state(3).unwrap();

    let CurveKeyFrame::Stepped(calculated) = calculated else {
        panic!("NO");
    };

    let Some(other_frame) = calculated.self_as_any().downcast_ref::<BodyOrbit>() else {
        panic!("Did not have a valid keyframe of the same type");
    };

    assert_eq!(other_frame, &BodyOrbit { orbits: 1 });
}

#[derive(Reflect, Clone, Component, Default, PartialEq, Debug)]
#[reflect(Component)]
pub struct BodyAngle {
    angle: f64,
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
            angle: self.angle + (other_frame.angle - self.angle) * ratio as f64,
        })
    }
}

#[derive(Reflect, Clone, Component, Default, PartialEq, Debug)]
#[reflect(Component)]
pub struct BodyOrbit {
    orbits: u32,
}

impl SteppedKeyframe for BodyOrbit {}
