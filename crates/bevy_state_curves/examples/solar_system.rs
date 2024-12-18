use std::time::Duration;

use bevy::{
    prelude::{
        default, App, AssetServer, Camera2d, Commands, Component, Entity, FixedUpdate, Query, Res,
        ResMut, Resource, Startup, Transform, Update, Vec2, Vec3,
    },
    sprite::Sprite,
    time::{Fixed, Time},
    DefaultPlugins,
};
use bevy_state_curves::prelude::{
    CurveTrait, GameTick, LinearCurve, LinearKeyframe, SteppedCurve, SteppedKeyframe,
};

use bevy_egui::{
    egui::{self, Align2, Color32, Frame, Margin, Stroke},
    EguiContexts, EguiPlugin,
};

extern crate bevy_state_curves;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, EguiPlugin));

    app.insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f32(0.1)));
    app.add_systems(Startup, setup);
    app.add_systems(
        Update,
        (
            update_body_position,
            update_body_components_for_viewed_tick,
            ui,
        ),
    );
    app.add_systems(
        FixedUpdate,
        (update_viewed_tick, simulation_tick, simulate_bodies),
    );

    app.insert_resource(ViewedTick(0));
    app.insert_resource(DisplayTimeForward);
    app.insert_resource(SimulationTick(0));
    app.insert_resource(SimulateGame);
    app.insert_resource(LastStateUpdatedTick(999));

    app.run();
}

/// The max angle radius to almost do a full circle
const CIRCLE_ANGLE: f32 = 6.0;
/// How much orbit should be increased every time a body goes all the way around
const ORBIT_AMOUNT: u32 = 1;
/// How many ticks into the future the game should simulate and have valid state for.
///
/// The game will simulate ahead of it in order to make sure that state is valid through the entire ticks
const FUTURE_SIMULATION_TICKS: u64 = 300;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    {
        let mut object_state = BodyCurves::new();
        object_state.rotation_point.insert_keyframe(
            0,
            BodyRotationPoint {
                point_x: 0.0,
                point_y: 0.0,
            },
        );
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

        let body = commands.spawn_empty().id();

        let frames = object_state.get_object_state_for_tick(0);

        commands.entity(body).insert((
            object_state,
            Sprite {
                image: asset_server.load("planet_three.png"),
                ..default()
            },
            Transform::from_scale(Vec3::new(3.0, 3.0, 3.0)),
            frames,
        ));
    }

    {
        let mut object_state = BodyCurves::new();
        object_state.rotation_point.insert_keyframe(
            0,
            BodyRotationPoint {
                point_x: 0.0,
                point_y: 0.0,
            },
        );
        object_state
            .speed
            .insert_keyframe(0, BodySpeed { speed: 30 });
        object_state
            .radius
            .insert_keyframe(0, BodyRadius { radius: 100.0 });
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

        let body = commands.spawn_empty().id();

        let frames = object_state.get_object_state_for_tick(0);

        commands.entity(body).insert((
            object_state,
            Sprite {
                image: asset_server.load("planet_two.png"),
                ..default()
            },
            Transform::from_scale(Vec3::new(4.0, 4.0, 4.0)),
            frames,
        ));
    }

    {
        let mut object_state = BodyCurves::new();
        object_state.rotation_point.insert_keyframe(
            0,
            BodyRotationPoint {
                point_x: 0.0,
                point_y: 0.0,
            },
        );
        object_state
            .speed
            .insert_keyframe(0, BodySpeed { speed: 120 });
        object_state
            .radius
            .insert_keyframe(0, BodyRadius { radius: 150.0 });
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

        let body = commands.spawn_empty().id();

        let frames = object_state.get_object_state_for_tick(0);

        commands.entity(body).insert((
            object_state,
            Sprite {
                image: asset_server.load("planet_one.png"),
                ..default()
            },
            Transform::from_scale(Vec3::new(2.0, 2.0, 2.0)),
            frames,
        ));
    }

    {
        let mut object_state = BodyCurves::new();
        object_state.rotation_point.insert_keyframe(
            0,
            BodyRotationPoint {
                point_x: 0.0,
                point_y: 0.0,
            },
        );
        object_state
            .speed
            .insert_keyframe(0, BodySpeed { speed: 25 });
        object_state
            .radius
            .insert_keyframe(0, BodyRadius { radius: 300.0 });
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

        let body = commands.spawn_empty().id();

        let frames = object_state.get_object_state_for_tick(0);

        commands.entity(body).insert((
            object_state,
            Sprite {
                image: asset_server.load("moon.png"),
                ..default()
            },
            Transform::from_scale(Vec3::new(2.0, 2.0, 2.0)),
            frames,
        ));
    }
}

/// Ticks the simulation tick every fixed timestep no matter what
fn simulation_tick(
    mut simulation_tick: ResMut<SimulationTick>,
    maybe_play: Option<Res<SimulateGame>>,
) {
    if maybe_play.is_some() {
        simulation_tick.0 += 1;
    }
}

/// Updates the viewed tick every fixed timestep if [`DisplayTimeForward`] exists.
fn update_viewed_tick(
    mut viewed_tick: ResMut<ViewedTick>,
    maybe_play: Option<Res<DisplayTimeForward>>,
) {
    if maybe_play.is_some() {
        viewed_tick.0 += 1;
    }
}

/// This simulates all the bodies into the future every tick.
///
/// It needs to iterate through every body and get each bodies curves info for current tick + 300.
/// It then checks if there are any keyframes that exist past that tick. If there are not then it goes to the furthest tick and
/// simulates the correct distance after and so on
fn simulate_bodies(
    simulation_tick: Res<SimulationTick>,
    mut bodies: Query<(Entity, &mut BodyCurves)>,
) {
    for (_entity, mut object_state) in bodies.iter_mut() {
        let mut farthest_state = simulation_tick.0;
        let mut last_angle = 0f32;

        for (tick, frame) in object_state.angle.iter_future_curves(simulation_tick.0) {
            farthest_state = *tick;
            last_angle = frame.angle;
        }

        if farthest_state < farthest_state + FUTURE_SIMULATION_TICKS {
            let body_orbit = object_state.orbit.get_state(farthest_state).unwrap();

            let mut last_orbit_count = body_orbit.orbits;

            let object_speed = object_state.speed.get_state(farthest_state).unwrap();

            let mut i: u64 = farthest_state + object_speed.speed as u64;
            while i < farthest_state + FUTURE_SIMULATION_TICKS {
                last_angle = last_angle + CIRCLE_ANGLE;
                last_orbit_count = last_orbit_count + ORBIT_AMOUNT;
                object_state
                    .angle
                    .insert_keyframe(i, BodyAngle { angle: last_angle });
                object_state.orbit.insert_keyframe(
                    i,
                    BodyOrbit {
                        orbits: last_orbit_count,
                    },
                );

                let object_speed = object_state.speed.get_state(i).unwrap();
                i += object_speed.speed as u64;
            }
        }
    }
}

/// Updates all bodies positions to the correct spot based on their current components.
fn update_body_position(mut bodies: Query<(&mut Transform, &BodyAngle, &BodyOrbit, &BodyRadius)>) {
    for (mut transform, angle, _orbit, radius) in bodies.iter_mut() {
        let x = angle.angle.cos() * radius.radius;
        let y = angle.angle.sin() * radius.radius;
        transform.translation = Vec2::new(x, y).extend(1.0)
    }
}

/// Updates all bodies to have the correct components for the given tick
fn update_body_components_for_viewed_tick(
    bodies: Query<(Entity, &BodyCurves)>,
    viewed_tick: Res<ViewedTick>,
    mut last_state_updated_tick: ResMut<LastStateUpdatedTick>,
    mut commands: Commands,
) {
    if viewed_tick.0 == last_state_updated_tick.0 {
        return;
    }
    for (entity, object_state) in bodies.iter() {
        let frames = object_state.get_object_state_for_tick(viewed_tick.0);
        commands.entity(entity).insert(frames);
    }

    last_state_updated_tick.0 = viewed_tick.0;
}

#[derive(Resource)]
pub struct ViewedTick(GameTick);

#[derive(Resource)]
pub struct DisplayTimeForward;

#[derive(Resource)]
pub struct SimulationTick(GameTick);

#[derive(Resource)]
pub struct SimulateGame;

#[derive(Resource)]
pub struct LastStateUpdatedTick(GameTick);

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

    fn get_object_state_for_tick(
        &self,
        tick: GameTick,
    ) -> (
        BodyAngle,
        BodyRadius,
        BodyRotationPoint,
        BodySpeed,
        BodyOrbit,
    ) {
        (
            self.angle.get_state(tick).unwrap(),
            self.radius.get_state(tick).unwrap(),
            self.rotation_point.get_state(tick).unwrap(),
            self.speed.get_state(tick).unwrap(),
            self.orbit.get_state(tick).unwrap(),
        )
    }
}

/// This component tracks the current angle of the body
#[derive(Clone, Component)]
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
#[derive(Clone, Component)]
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
#[derive(Clone, Component)]
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
#[derive(Clone, Component)]
pub struct BodySpeed {
    speed: u32,
}

impl SteppedKeyframe<BodySpeed> for BodySpeed {}

/// This component tracks how many orbits the object has done
#[derive(Clone, Component)]
pub struct BodyOrbit {
    orbits: u32,
}

impl SteppedKeyframe<BodyOrbit> for BodyOrbit {}

fn ui(
    mut egui_context: EguiContexts,
    option_playing: Option<Res<DisplayTimeForward>>,
    option_simulate: Option<Res<SimulateGame>>,
    simulation_tick: Res<SimulationTick>,
    viewed_tick: Res<ViewedTick>,
    mut commands: Commands,
) {
    let settings_frame = Frame {
        fill: Color32::from_rgba_unmultiplied(0, 0, 0, 255),
        stroke: Stroke::new(0., Color32::WHITE),
        inner_margin: Margin {
            left: 5.0,
            right: 5.0,
            top: 5.0,
            bottom: 5.0,
        },
        ..default()
    };
    egui::Window::new("Settings")
        .frame(settings_frame)
        .anchor(Align2::CENTER_TOP, egui::Vec2 { x: 0.0, y: 32. })
        .resizable(true)
        .collapsible(true)
        .title_bar(true)
        .show(egui_context.ctx_mut(), |ui| {
            ui.label(format!("Simulation Tick: {:?}", simulation_tick.0));
            ui.label(format!("Viewed Tick: {:?}", viewed_tick.0));

            if option_playing.is_none() {
                if ui.button("Animate Simulation").clicked() {
                    commands.insert_resource(DisplayTimeForward);
                }
            } else {
                if ui.button("Stop Animating Simulation").clicked() {
                    commands.remove_resource::<DisplayTimeForward>();
                }
            }
            if option_simulate.is_none() {
                if ui.button("Start Simulation").clicked() {
                    commands.insert_resource(SimulateGame);
                }
            } else {
                if ui.button("Stop Simulation").clicked() {
                    commands.remove_resource::<SimulateGame>();
                }
            }

            let response = ui
                .button("Skip Forward")
                .interact(egui::Sense::click_and_drag());

            if response.dragged() {
                commands.insert_resource(ViewedTick(
                    (viewed_tick.0 + 1).clamp(0, simulation_tick.0 + FUTURE_SIMULATION_TICKS),
                ));
            }

            let response = ui
                .button("Skip Back")
                .interact(egui::Sense::click_and_drag());

            if response.dragged() {
                commands.insert_resource(ViewedTick(
                    (viewed_tick.0.saturating_sub(1))
                        .clamp(0, simulation_tick.0 + FUTURE_SIMULATION_TICKS),
                ));
            }

            if simulation_tick.0 != viewed_tick.0 {
                if ui.button("View Current Animation").clicked() {
                    commands.insert_resource(ViewedTick(simulation_tick.0));
                }
            }
        });
}
