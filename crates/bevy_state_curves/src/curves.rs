use std::{collections::BTreeMap, ops::Bound};

use bevy::prelude::Component;

use crate::{
    keyframe_trait::{LinearKeyFrame, SteppedKeyframe},
    GameTick,
};

/// The generic curve storage. This backs all the specific curve types storages internally
pub struct Curve<T> {
    frames: BTreeMap<GameTick, T>,
}

impl<T> Curve<T> {
    pub fn new() -> Curve<T> {
        Curve {
            frames: BTreeMap::new(),
        }
    }
    /// Inserts a keyframe of the given type into the storage at the given tick
    pub fn insert_keyframe(&mut self, tick: GameTick, keyframe: T) {
        self.frames.insert(tick, keyframe);
    }

    pub fn remove_keyframe(&mut self, tick: GameTick) {
        self.frames.remove(&tick);
    }

    pub fn get_keyframe(&self, tick: GameTick) -> Option<&T> {
        self.frames.get(&tick)
    }

    pub fn iter_future_curves(&self, tick: GameTick) -> impl Iterator<Item = (&GameTick, &T)> {
        self.frames
            .range((Bound::Excluded(&tick), Bound::Unbounded))
    }

    pub fn next_keyframe(&self, tick: GameTick) -> Option<(&GameTick, &T)> {
        self.frames
            .range((Bound::Excluded(&tick), Bound::Unbounded))
            .next()
    }

    pub fn iter_prev_curves(&self, tick: GameTick) -> impl Iterator<Item = (&GameTick, &T)> {
        self.frames
            .range((Bound::Unbounded, Bound::Excluded(&tick)))
    }

    pub fn prev_keyframe(&self, tick: GameTick) -> Option<(&GameTick, &T)> {
        self.frames
            .range((Bound::Unbounded, Bound::Excluded(&tick)))
            .rev()
            .next()
    }
}

#[derive(Component)]
pub struct LinearCurve<T: LinearKeyFrame<T>> {
    curve: Curve<T>,
}

impl<T: LinearKeyFrame<T>> LinearCurve<T> {
    pub fn new() -> LinearCurve<T> {
        LinearCurve {
            curve: Curve::new(),
        }
    }

    pub fn insert_keyframe(&mut self, tick: GameTick, keyframe: T) {
        self.curve.insert_keyframe(tick, keyframe);
    }

    pub fn remove_keyframe(&mut self, tick: GameTick) {
        self.curve.remove_keyframe(tick);
    }

    pub fn get_keyframe(&self, tick: GameTick) -> Option<&T> {
        self.curve.get_keyframe(tick)
    }

    pub fn iter_future_curves(&self, tick: GameTick) -> impl Iterator<Item = (&GameTick, &T)> {
        self.curve.iter_future_curves(tick)
    }

    pub fn next_keyframe(&self, tick: GameTick) -> Option<(&GameTick, &T)> {
        self.curve.next_keyframe(tick)
    }

    pub fn iter_prev_curves(&self, tick: GameTick) -> impl Iterator<Item = (&GameTick, &T)> {
        self.curve.iter_prev_curves(tick)
    }

    pub fn prev_keyframe(&self, tick: GameTick) -> Option<(&GameTick, &T)> {
        self.curve.prev_keyframe(tick)
    }

    pub fn get_state(&self, tick: GameTick) -> Option<T> {
        // we get the previous frame, if there is no previous frame then we try and get the current frame if we have one
        // if there is no current frame then we return none.

        let prev_frame = match self.prev_keyframe(tick) {
            Some(data) => data,
            None => match self.get_keyframe(tick) {
                Some(data) => return Some(data.clone()),
                None => return None,
            },
        };

        let next_frame = match self.get_keyframe(tick) {
            Some(data) => (&tick, data),
            None => match self.next_keyframe(tick) {
                Some(data) => data,
                None => return Some(prev_frame.1.clone()),
            },
        };

        let ratio =
            (tick as f32 - *prev_frame.0 as f32) / (*next_frame.0 as f32 - *prev_frame.0 as f32);
        Some(prev_frame.1.lerp(&next_frame.1, ratio))
    }
}

#[derive(Component)]
pub struct SteppedCurve<T: SteppedKeyframe<T>> {
    curve: Curve<T>,
}

impl<T: SteppedKeyframe<T>> SteppedCurve<T> {
    pub fn new() -> SteppedCurve<T> {
        SteppedCurve {
            curve: Curve::new(),
        }
    }

    pub fn insert_keyframe(&mut self, tick: GameTick, keyframe: T) {
        self.curve.insert_keyframe(tick, keyframe);
    }

    pub fn remove_keyframe(&mut self, tick: GameTick) {
        self.curve.remove_keyframe(tick);
    }

    pub fn get_keyframe(&self, tick: GameTick) -> Option<&T> {
        self.curve.get_keyframe(tick)
    }

    pub fn iter_future_curves(&self, tick: GameTick) -> impl Iterator<Item = (&GameTick, &T)> {
        self.curve.iter_future_curves(tick)
    }

    pub fn next_keyframe(&self, tick: GameTick) -> Option<(&GameTick, &T)> {
        self.curve.next_keyframe(tick)
    }

    pub fn iter_prev_curves(&self, tick: GameTick) -> impl Iterator<Item = (&GameTick, &T)> {
        self.curve.iter_prev_curves(tick)
    }

    pub fn prev_keyframe(&self, tick: GameTick) -> Option<(&GameTick, &T)> {
        self.curve.prev_keyframe(tick)
    }

    pub fn get_state(&self, tick: GameTick) -> Option<T> {
        // we get the previous frame, if there is no previous frame then we try and get the current frame if we have one
        // if there is no current frame then we return none.

        let data = match self.get_keyframe(tick) {
            Some(frame) => frame.clone(),
            None => match self.prev_keyframe(tick) {
                Some(data) => data.1.clone(),
                None => return None,
            },
        };
        Some(data.clone())
    }
}
