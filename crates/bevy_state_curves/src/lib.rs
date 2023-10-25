use std::{collections::BTreeMap, ops::Bound};

pub mod curves;
pub mod helpers;
pub mod keyframe_trait;

use bevy::{
    prelude::{Component, Entity, Resource},
    reflect::Reflect,
    utils::HashMap,
};
use keyframe_trait::{LinearKeyFrame, PulseKeyframe, SteppedKeyframe};
/*
/// Different types of Curves that can be saved
pub enum CurveType {
    /// Curve without any interpolation. Each keyframe is distinct and there is no interpolation between it and the next keyframe
    Stepped,
    /// Instantaneous events with no duration
    Pulse,
    /// A curve that is linearly interpolated between each saved keyframe. If there is not a next keyframe than it repeats
    Linear,
}

/// All the keyframes in a curve
pub enum CurveData {
    /// Curve without any interpolation. Each keyframe is distinct and there is no interpolation between it and the next keyframe
    Stepped(BTreeMap<GameTick, Box<dyn SteppedKeyframe>>),
    /// Instantaneous events with no duration
    Pulse(BTreeMap<GameTick, Box<dyn PulseKeyframe>>),
    /// A curve that is linearly interpolated between each saved keyframe. If there is not a next keyframe than it repeats
    Linear(BTreeMap<GameTick, Box<dyn LinearKeyFrame>>),
}

impl CurveData {
    /// Inserts a [`CurveKeyFrame`] into itself
    pub fn insert(&mut self, tick: GameTick, data: CurveKeyFrame) {
        match (self, data) {
            (CurveData::Stepped(map), CurveKeyFrame::Stepped(data)) => {
                map.insert(tick, data);
            }
            (CurveData::Pulse(map), CurveKeyFrame::Pulse(data)) => {
                map.insert(tick, data);
            }
            (CurveData::Linear(map), CurveKeyFrame::Linear(data)) => {
                map.insert(tick, data);
            }
            _ => {}
        };
    }

    /// Removes a [`CurveKeyFrame`] if it matches the [`CurveData`]s [`CurveType`]
    pub fn remove(&mut self, tick: GameTick) {
        match self {
            CurveData::Stepped(map) => {
                map.remove(&tick);
            }
            CurveData::Pulse(map) => {
                map.remove(&tick);
            }
            CurveData::Linear(map) => {
                map.remove(&tick);
            }
        };
    }

    /// Returns this curves keyframe for the given tick, if it exists. Note that this is *NOTE* the curves state
    /// for this tick
    pub fn get_keyframe(&self, tick: GameTick) -> Option<CurveKeyFrame> {
        match self {
            CurveData::Stepped(map) => Some(CurveKeyFrame::Stepped(map.get(&tick)?.clone())),
            CurveData::Pulse(map) => Some(CurveKeyFrame::Pulse(map.get(&tick)?.clone())),
            CurveData::Linear(map) => Some(CurveKeyFrame::Linear(map.get(&tick)?.clone())),
        }
    }

    /// Returns this curves calculated state for the given tick.
    pub fn get_state(&self, tick: GameTick) -> Option<CurveKeyFrame> {
        match self {
            CurveData::Stepped(map) => {
                let data = match map.get(&tick) {
                    Some(frame) => frame.clone(),
                    None => match map.range((Bound::Unbounded, Bound::Excluded(&tick))).last() {
                        Some(data) => data.1.clone(),
                        None => return None,
                    },
                };
                Some(CurveKeyFrame::Stepped(data))
            }
            CurveData::Pulse(map) => Some(CurveKeyFrame::Pulse(map.get(&tick)?.clone())),
            CurveData::Linear(map) => {
                // we get the previous frame, if there is no previous frame then we try and get the current frame if we have one
                // if there is no current frame then we return none.
                let prev_frame = match map.range((Bound::Unbounded, Bound::Excluded(&tick))).last()
                {
                    Some(data) => data.clone(),
                    None => match map.get(&tick) {
                        Some(frame) => return Some(CurveKeyFrame::Linear(frame.clone())),
                        None => return None,
                    },
                };

                let next_frame = match map.get_key_value(&tick) {
                    Some(frame) => frame.clone(),
                    None => match map.range((Bound::Excluded(&tick), Bound::Unbounded)).next() {
                        Some(data) => data.clone(),
                        None => return Some(CurveKeyFrame::Linear(prev_frame.1.clone())),
                    },
                };
                let ratio = (tick as f32 - *prev_frame.0 as f32)
                    / (*next_frame.0 as f32 - *prev_frame.0 as f32);

                Some(CurveKeyFrame::Linear(
                    prev_frame.1.lerp(&next_frame.1, ratio),
                ))
            }
        }
    }

    /// Attempts to get the next curve from the requested tick. Returns the tick and the data if it exists
    pub fn iter_future_curves(
        &self,
        tick: GameTick,
    ) -> Box<dyn Iterator<Item = (GameTick, CurveKeyFrame)> + '_> {
        match self {
            CurveData::Stepped(map) => {
                let new_map = map
                    .range((Bound::Excluded(&tick), Bound::Unbounded))
                    .map(|(tick, data)| (tick.clone(), CurveKeyFrame::Stepped(data.clone())));
                Box::new(new_map)
            }
            CurveData::Pulse(map) => {
                let new_map = map
                    .range((Bound::Excluded(&tick), Bound::Unbounded))
                    .map(|(tick, data)| (tick.clone(), CurveKeyFrame::Pulse(data.clone())));
                Box::new(new_map)
            }
            CurveData::Linear(map) => {
                let new_map = map
                    .range((Bound::Excluded(&tick), Bound::Unbounded))
                    .map(|(tick, data)| (tick.clone(), CurveKeyFrame::Linear(data.clone())));
                Box::new(new_map)
            }
        }
    }

    /// Attempts to get the next curve from the requested tick. Returns the tick and the data if it exists
    pub fn next_curve(&self, tick: GameTick) -> Option<(GameTick, CurveKeyFrame)> {
        match self {
            CurveData::Stepped(map) => {
                let data = match map.range((Bound::Excluded(&tick), Bound::Unbounded)).next() {
                    Some(data) => data,
                    None => return None,
                };

                Some((data.0.clone(), CurveKeyFrame::Stepped(data.1.clone())))
            }
            CurveData::Pulse(map) => {
                let data = match map.range((Bound::Excluded(&tick), Bound::Unbounded)).next() {
                    Some(data) => data,
                    None => return None,
                };

                Some((data.0.clone(), CurveKeyFrame::Pulse(data.1.clone())))
            }
            CurveData::Linear(map) => {
                let data = match map.range((Bound::Excluded(&tick), Bound::Unbounded)).next() {
                    Some(data) => data,
                    None => return None,
                };

                Some((data.0.clone(), CurveKeyFrame::Linear(data.1.clone())))
            }
        }
    }

    /// Attempts to get the previous curve from the requested tick. Returns the tick and the data if it exists
    pub fn prev_curve(&self, tick: GameTick) -> Option<(GameTick, CurveKeyFrame)> {
        match self {
            CurveData::Stepped(map) => {
                let data = match map.range((Bound::Unbounded, Bound::Excluded(&tick))).next() {
                    Some(data) => data,
                    None => return None,
                };

                Some((data.0.clone(), CurveKeyFrame::Stepped(data.1.clone())))
            }
            CurveData::Pulse(map) => {
                let data = match map.range((Bound::Unbounded, Bound::Excluded(&tick))).next() {
                    Some(data) => data,
                    None => return None,
                };

                Some((data.0.clone(), CurveKeyFrame::Pulse(data.1.clone())))
            }
            CurveData::Linear(map) => {
                let data = match map.range((Bound::Unbounded, Bound::Excluded(&tick))).next() {
                    Some(data) => data,
                    None => return None,
                };

                Some((data.0.clone(), CurveKeyFrame::Linear(data.1.clone())))
            }
        }
    }
}

/// A keyframe in a curve
#[derive(Clone)]
pub enum CurveKeyFrame {
    /// Curve without any interpolation. Each keyframe is distinct and there is no interpolation between it and the next keyframe
    Stepped(Box<dyn SteppedKeyframe>),
    /// Instantaneous events with no duration
    Pulse(Box<dyn PulseKeyframe>),
    /// A curve that is linearly interpolated between each saved keyframe. If there is not a next keyframe than it repeats
    Linear(Box<dyn LinearKeyFrame>),
}

impl CurveKeyFrame {
    /// Returns the [`CurveType`] of the keyframe
    pub fn curve_type(&self) -> CurveType {
        match self {
            CurveKeyFrame::Stepped(_) => CurveType::Stepped,
            CurveKeyFrame::Pulse(_) => CurveType::Pulse,
            CurveKeyFrame::Linear(_) => CurveType::Linear,
        }
    }

    /// Clones the data in self and returns it as a Box<dyn Reflect>
    pub fn as_reflect(&self) -> Box<dyn Reflect> {
        match self {
            CurveKeyFrame::Stepped(data) => data.clone_value(),
            CurveKeyFrame::Pulse(data) => data.clone_value(),
            CurveKeyFrame::Linear(data) => data.clone_value(),
        }
    }

    /// Returns the type name of the underlying type of this keyframe
    pub fn type_name(&self) -> String {
        match self {
            CurveKeyFrame::Stepped(data) => data.type_name().to_owned(),
            CurveKeyFrame::Pulse(data) => data.type_name().to_owned(),
            CurveKeyFrame::Linear(data) => data.type_name().to_owned(),
        }
    }

    /// Downcasts to the given type. Panics if it fails
    pub fn downcast<T: 'static>(&self) -> &T {
        match self {
            CurveKeyFrame::Stepped(data) => data.self_as_any().downcast_ref::<T>().unwrap(),
            CurveKeyFrame::Pulse(data) => data.self_as_any().downcast_ref::<T>().unwrap(),
            CurveKeyFrame::Linear(data) => data.self_as_any().downcast_ref::<T>().unwrap(),
        }
    }

    /// Attempts to downcasts to the given type, returning Some if it succeds and None if it fails
    pub fn try_downcast<T: 'static>(&self) -> Option<&T> {
        match self {
            CurveKeyFrame::Stepped(data) => data.self_as_any().downcast_ref::<T>(),
            CurveKeyFrame::Pulse(data) => data.self_as_any().downcast_ref::<T>(),
            CurveKeyFrame::Linear(data) => data.self_as_any().downcast_ref::<T>(),
        }
    }
}

pub struct CurveInfo {
    pub component_type_name: String,
    pub curve_type: CurveType,
}

/// An id that represents a single objects state
#[derive(Component)]
pub struct ObjectId(pub u32);

/// A resource that contains when each StateId existed
#[derive(Resource)]
pub struct ObjectTimings {
    object_timings: HashMap<GameTick, Vec<ObjectId>>,
}

/// A component that contains all of this [`ObjectId`]s curves
#[derive(Component)]
pub struct ObjectState {
    /// The [`GameTick`] that this object was created
    object_created: GameTick,
    object_destroyed: Option<GameTick>,
    curves: HashMap<String, (CurveInfo, CurveData)>,
}

impl ObjectState {
    /// Constructs a new empty [`ObjectState`]
    pub fn new(tick: GameTick) -> Self {
        Self {
            object_created: tick,
            object_destroyed: None,
            curves: HashMap::new(),
        }
    }

    /// Returns the tick that this Object was created
    pub fn created_tick(&self) -> GameTick {
        self.object_created
    }

    /// Returns the tick that this Object was destroyed
    pub fn destroyed_tick(&self) -> Option<GameTick> {
        self.object_destroyed
    }

    /// Returns whether this object has been destroyed or not
    pub fn is_destroyed(&self) -> bool {
        self.object_destroyed.is_some()
    }

    /// Returns whether this object is still alive or not
    pub fn is_alive(&self) -> bool {
        self.object_destroyed.is_none()
    }

    /// Sets the tick that this object has been destroyed on
    pub fn destroy(&mut self, tick: GameTick) {
        self.object_destroyed = Some(tick);
    }

    /// Gets a reference to the [`CurveData`] of this object
    pub fn get_curves(&self, curve_name: &str) -> Option<&(CurveInfo, CurveData)> {
        self.curves.get(curve_name)
    }

    /// Registers a new curve under the given curve_name with the given curve_type for the given component_type_name. Is automatically called
    /// by [add_keyframe] as needed
    pub fn register_new_curve(
        &mut self,
        curve_name: &str,
        curve_type: CurveType,
        component_type_name: String,
    ) {
        if let None = self.curves.get(curve_name) {
            let map = match curve_type {
                CurveType::Stepped => CurveData::Stepped(BTreeMap::new()),
                CurveType::Pulse => CurveData::Pulse(BTreeMap::new()),
                CurveType::Linear => CurveData::Linear(BTreeMap::new()),
            };
            self.curves.insert(
                curve_name.to_owned(),
                (
                    CurveInfo {
                        component_type_name,
                        curve_type,
                    },
                    map,
                ),
            );
        }
    }

    /// Adds a new keyframe to the curve for the given [`GameTick`].
    ///
    /// # Panics
    /// - If the insert keyframe does not match the registered component type
    ///
    /// TODO: See if theres a way to make this generic to accept a T of any keyframe and it automatically inserts it
    pub fn add_keyframe(&mut self, curve_name: &str, tick: GameTick, keyframe: CurveKeyFrame) {
        match self.curves.get_mut(curve_name) {
            Some((info, map)) => {
                if info.component_type_name != keyframe.type_name() {
                    panic!(
                        "Attempted to insert a different component type into curve {:?}",
                        curve_name
                    );
                }
                map.insert(tick, keyframe);
            }
            None => {
                self.register_new_curve(curve_name, keyframe.curve_type(), keyframe.type_name());
                self.add_keyframe(curve_name, tick, keyframe);
            }
        }
    }

    /// Helper function to insert a [`SteppedKeyFrame`] into a curve for a specific tick
    pub fn add_stepped_keyframe<T: SteppedKeyframe>(
        &mut self,
        curve_name: &str,
        tick: GameTick,
        keyframe: T,
    ) {
        self.add_keyframe(curve_name, tick, CurveKeyFrame::Stepped(Box::new(keyframe)));
    }

    /// Helper function to insert a [`LinearKeyFrame`] into a curve for a specific tick
    pub fn add_linear_keyframe<T: LinearKeyFrame>(
        &mut self,
        curve_name: &str,
        tick: GameTick,
        keyframe: T,
    ) {
        self.add_keyframe(curve_name, tick, CurveKeyFrame::Linear(Box::new(keyframe)));
    }

    /// Helper function to insert a [`PulseKeyframe`] into a curve for a specific tick
    pub fn add_pulse_keyframe<T: PulseKeyframe>(
        &mut self,
        curve_name: &str,
        tick: GameTick,
        keyframe: T,
    ) {
        self.add_keyframe(curve_name, tick, CurveKeyFrame::Pulse(Box::new(keyframe)));
    }

    /// Removes the keyframe from the given [`GameTick`] and returns it if it exists.
    ///
    /// Silently fails if the curve has not been registered or if the keyframe does not exist
    pub fn remove_keyframe(&mut self, curve_name: &str, tick: GameTick) {
        if let Some((_, map)) = self.curves.get_mut(curve_name) {
            map.remove(tick);
        }
    }

    /// Returns all keyframes that exist in this tick.
    ///
    /// # Note
    ///
    /// This does *NOT* return an accurate state of the object for the given tick. This just returns all keyframes that exists
    /// in the given tick. Use [`ObjectState::get_object_state_for_tick`] to get the canonical state for a tick.
    pub fn get_keyframes_in_tick(&self, tick: GameTick) -> Vec<(String, CurveKeyFrame)> {
        let mut return_vec = vec![];
        for (curve_name, (curve_info, curve_map)) in self.curves.iter() {
            match curve_info.curve_type {
                CurveType::Stepped => {
                    if let Some(component) = curve_map.get_keyframe(tick.clone()) {
                        return_vec.push((curve_name.clone(), component.clone()))
                    }
                }
                CurveType::Pulse => {
                    if let Some(component) = curve_map.get_keyframe(tick.clone()) {
                        return_vec.push((curve_name.clone(), component.clone()))
                    }
                }
                CurveType::Linear => {
                    if let Some(component) = curve_map.get_keyframe(tick.clone()) {
                        return_vec.push((curve_name.clone(), component.clone()))
                    }
                }
            }
        }
        return_vec
    }

    /// Returns a vec of what the calculated objects state should be for this tick
    pub fn get_object_state_for_tick(&self, tick: GameTick) -> Vec<(String, CurveKeyFrame)> {
        let mut return_vec = vec![];
        for (curve_name, (_curve_info, curve_map)) in self.curves.iter() {
            if let Some(frame) = curve_map.get_state(tick) {
                return_vec.push((curve_name.clone(), frame));
            }
        }
        return_vec
    }
}

/// A resource that contains a map from an [`ObjectId`] to an entity that holds all of that [`ObjectId`] curves
#[derive(Resource)]
pub struct ObjectStateCurveMap {
    object_state: HashMap<ObjectId, Entity>,
}
*/
pub type GameTick = u64;
