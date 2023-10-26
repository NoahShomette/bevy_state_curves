# `bevy_state_curves`

Bevy_State_Curves is an implementation of the state storage and interpolation system described in this [blog post](https://www.forrestthewoods.com/blog/tech_of_planetary_annihilation_chrono_cam/). This system was used in Planetary Annihilation and has some neat features that arise for mostly free from it.

The implementation of this crate is focused on compile time curves and integration with the ECS. Compile time curves were chosen for the performance factors while ECS because ECS everything.

## Usage

1. Create a new curve component

```rust
#[derive(Reflect, Clone, Component, Default, PartialEq, Debug)]
#[reflect(Component)]
pub struct ObjectRadius {
    radius: f32,
}

impl LinearKeyFrame<ObjectRadius> for ObjectRadius {
    fn lerp(&self, next_frame_state: &ObjectRadius, ratio: f64) -> ObjectRadius {
        ObjectRadius {
            radius: self.radius + (next_frame_state.radius - self.radius) * ratio as f32,
        }
    }
}
```

2. Insert it into an entity

```rust
     commands.entity(entity).insert(LinearCurve<ObjectRadius>::new());
```

3. Add/remove keyframes using the curve as a normal component on an entity. Get state in a normal system using normal queries!

```rust
    fn insert_keyframes(mut radius_query: Query<&mut LinearCurve<ObjectRadius>, tick: Res<CurrentGameTick>){
        for radius in radius_query.iter_mut(){
            radius.insert_keyframe(1, ObjectRadius{radius: 1.0});
            radius.insert_keyframe(10, ObjectRadius{radius: 2.0});
        }
    }

    fn curve_entity_system(radius_query: Query<&LinearCurve<ObjectRadius>){
        for radius in radius_query.iter(){
            let radius_at_tick = body.get_state(5);
            assert_eq!(radius_at_tick, 1.5);
        }
    }
```

## Future Plans

`Bevy_State_Curves` is created for a specific open source project idea I have. As that project takes shape I will eventually link it here. Because of this features to this crate will be driven by features needed for that project. If you randomly and terribly decide to use this crate, let me know if theres something wrong, it needs updating, or even better, make prs and issues as needed!

At this time, current _potential_ ideas for features are:

- A custom `SystemParam` that is used to spawn and manage curves. Used to drive other features
- A concept of a `StateLifetime`. Essentially when a state exists in the world. This would be used to drive filtering of global state concepts. Eg reset the world to this tick filtering states by only those that "existed" at this time.
- More `CurveTrait` functions. No clue yet but I'm sure some more will be needed eventually
- Tests!!!

Other than Tests and `CurveTrait` functions, these features will most likely not materialize in this crate itself. They are too specific and easier implemented in whatever project is using this crate manually.
