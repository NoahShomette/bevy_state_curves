use std::{collections::BTreeMap, ops::Bound};

use bevy::prelude::Component;

use crate::{
    keyframe_trait::{LinearKeyframe, PulseKeyframe, SteppedKeyframe},
    GameTick,
};

/// The generic curve storage. This backs all the specific curve types storages internally
///
/// # Implementing a new curve
/// If you'd like to implement a new curve its as easy as implementing the [`CurveTrait`]
/// on your new curve type. It's recommended to use this [`Curve`] struct as the basis for your curve storage but
/// that is not required if you don't want to.
///
/// ```rust,ignore
///
/// /// Your custom curve type. You're welcome to use your own T: trait bounds.
/// #[derive(Component)]
/// pub struct CustomLinearCurve<T: LinearKeyFrame<T>> {
///     curve: Curve<T>,
/// }
///
/// /// Just implement the [`CurveTrait`] and you'll get the standard curve interactions
/// impl<T: LinearKeyFrame<T>> CurveTrait<T> for CustomLinearCurve<T> {
///    /// ... Implementation skipped for brevity. See source docs for examples ...
///}
/// ```
pub struct Curve<V> {
    map: BTreeMap<GameTick, V>,
}

impl<T> Curve<T> {
    pub fn new() -> Curve<T> {
        Curve {
            map: BTreeMap::new(),
        }
    }
    /// Inserts a keyframe of the given type into the storage at the given [`GameTick`]
    pub fn insert_keyframe(&mut self, tick: GameTick, keyframe: T) {
        self.map.insert(tick, keyframe);
    }

    /// Removes the keyframe at the given [`GameTick`] if there is one
    pub fn remove_keyframe(&mut self, tick: GameTick) {
        self.map.remove(&tick);
    }

    /// Gets the keyframe at the given [`GameTick`] if there is one
    pub fn get_keyframe(&self, tick: GameTick) -> Option<&T> {
        self.map.get(&tick)
    }

    /// Returns a vec of all keyframes that come ***AFTER*** the given [`GameTick`], excluding any keyframe that may exist on the requested tick
    pub fn iter_future_curves(&self, tick: GameTick) -> Vec<(&GameTick, &T)> {
        self.map
            .range((Bound::Excluded(&tick), Bound::Unbounded))
            .collect::<Vec<(&GameTick, &T)>>()
    }

    /// Returns the kext keyframe, if it exists, that comes after the given [`GameTick`]
    pub fn next_keyframe(&self, tick: GameTick) -> Option<(&GameTick, &T)> {
        self.map
            .range((Bound::Excluded(&tick), Bound::Unbounded))
            .next()
    }

    /// Returns a vec of all keyframes that come ***BEFORE*** the given [`GameTick`], excluding any keyframe that may exist on the requested tick
    pub fn iter_prev_curves(&self, tick: GameTick) -> Vec<(&GameTick, &T)> {
        self.map
            .range((Bound::Unbounded, Bound::Excluded(&tick)))
            .collect::<Vec<(&GameTick, &T)>>()
    }

    /// Returns the previous keyframe, if it exists, that comes before the given [`GameTick`]
    pub fn prev_keyframe(&self, tick: GameTick) -> Option<(&GameTick, &T)> {
        self.map
            .range((Bound::Unbounded, Bound::Excluded(&tick)))
            .next_back()
    }
}

/// Core trait used to interact with all Curves
pub trait CurveTrait<T> {
    /// Creates a new [`Self`]
    fn new() -> Self;

    /// Inserts a keyframe of the given type into the storage at the given [`GameTick`]
    fn insert_keyframe(&mut self, tick: GameTick, keyframe: T);

    /// Removes the keyframe at the given [`GameTick`] if there is one
    fn remove_keyframe(&mut self, tick: GameTick);

    /// Gets the keyframe at the given [`GameTick`] if there is one
    fn get_keyframe(&self, tick: GameTick) -> Option<&T>;

    /// Returns a vec of all keyframes that come ***AFTER*** the given [`GameTick`], excluding any keyframe that may exist on the requested tick
    fn iter_future_curves(&self, tick: GameTick) -> Vec<(&GameTick, &T)>;

    /// Returns the kext keyframe, if it exists, that comes after the given [`GameTick`]
    fn next_keyframe(&self, tick: GameTick) -> Option<(&GameTick, &T)>;

    /// Returns a vec of all keyframes that come ***BEFORE*** the given [`GameTick`], excluding any keyframe that may exist on the requested tick
    fn iter_prev_curves(&self, tick: GameTick) -> Vec<(&GameTick, &T)>;

    /// Returns the previous keyframe, if it exists, that comes before the given [`GameTick`]
    fn prev_keyframe(&self, tick: GameTick) -> Option<(&GameTick, &T)>;

    /// Returns the state of the curve at the given [`GameTick`].
    ///
    /// The implementation and logic of the returned state depends on the exact type of curve. See the curve object for details
    fn get_state(&self, tick: GameTick) -> Option<T>;
}

/// A Linear curve containing all keyframes that implement [`LinearKeyFrame`]
///
/// ## Explanation:
///
/// State in a linear curve is derived via a linear interpolation between the last keyframe and the next keyframe.
///
/// - If a request for state falls exactly on a keyframe than that keyframe is simply returned.
/// - If there are no future keyframes then the last keyframe is used.
/// - If there are no past keyframes then no state is returned.
/// - Otherwise the returned state is a lerped representation of what the state should be on that tick.
#[derive(Component)]
pub struct LinearCurve<T: LinearKeyframe<T>> {
    curve: Curve<T>,
}

impl<T: LinearKeyframe<T>> CurveTrait<T> for LinearCurve<T> {
    fn new() -> LinearCurve<T> {
        LinearCurve {
            curve: Curve::new(),
        }
    }

    fn insert_keyframe(&mut self, tick: GameTick, keyframe: T) {
        self.curve.insert_keyframe(tick, keyframe);
    }

    fn remove_keyframe(&mut self, tick: GameTick) {
        self.curve.remove_keyframe(tick);
    }

    fn get_keyframe(&self, tick: GameTick) -> Option<&T> {
        self.curve.get_keyframe(tick)
    }

    fn iter_future_curves(&self, tick: GameTick) -> Vec<(&GameTick, &T)> {
        self.curve.iter_future_curves(tick)
    }

    fn next_keyframe(&self, tick: GameTick) -> Option<(&GameTick, &T)> {
        self.curve.next_keyframe(tick)
    }

    fn iter_prev_curves(&self, tick: GameTick) -> Vec<(&GameTick, &T)> {
        self.curve.iter_prev_curves(tick)
    }

    fn prev_keyframe(&self, tick: GameTick) -> Option<(&GameTick, &T)> {
        self.curve.prev_keyframe(tick)
    }

    fn get_state(&self, tick: GameTick) -> Option<T> {
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
            (tick as f64 - *prev_frame.0 as f64) / (*next_frame.0 as f64 - *prev_frame.0 as f64);
        Some(prev_frame.1.lerp(next_frame.1, ratio))
    }
}

impl<T: LinearKeyframe<T>> LinearCurve<T> {}

/// A stepped curve containing all keyframes that implement [`SteppedKeyframe`]
///
/// ## Explanation:
///
/// State in a Stepped curve is a state with no interpolation. All ticks after a keyframe will contain the same state as that single keyframe
/// until another keyframe is inserted. At that point the state following that keyframe will be that keyframe
///
/// - State is the last keyframe before that [`GameTick`]
#[derive(Component)]
pub struct SteppedCurve<T: SteppedKeyframe<T>> {
    curve: Curve<T>,
}

impl<T: SteppedKeyframe<T>> CurveTrait<T> for SteppedCurve<T> {
    fn new() -> SteppedCurve<T> {
        SteppedCurve {
            curve: Curve::new(),
        }
    }

    fn insert_keyframe(&mut self, tick: GameTick, keyframe: T) {
        self.curve.insert_keyframe(tick, keyframe);
    }

    fn remove_keyframe(&mut self, tick: GameTick) {
        self.curve.remove_keyframe(tick);
    }

    fn get_keyframe(&self, tick: GameTick) -> Option<&T> {
        self.curve.get_keyframe(tick)
    }

    fn iter_future_curves(&self, tick: GameTick) -> Vec<(&GameTick, &T)> {
        self.curve.iter_future_curves(tick)
    }

    fn next_keyframe(&self, tick: GameTick) -> Option<(&GameTick, &T)> {
        self.curve.next_keyframe(tick)
    }

    fn iter_prev_curves(&self, tick: GameTick) -> Vec<(&GameTick, &T)> {
        self.curve.iter_prev_curves(tick)
    }

    fn prev_keyframe(&self, tick: GameTick) -> Option<(&GameTick, &T)> {
        self.curve.prev_keyframe(tick)
    }

    fn get_state(&self, tick: GameTick) -> Option<T> {
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

/// A pulse curve containing all keyframes that implement [`PulseKeyframe`]
///
/// ## Explanation:
///
/// State in a Pulse curve is instantanious. State is only valid on the [`GameTick`] that it exists on.
///
/// - State only exists on the [`GameTick`] that it was saved under
#[derive(Component)]
pub struct PulseCurve<T: PulseKeyframe<T>> {
    curve: Curve<T>,
}

impl<T: PulseKeyframe<T>> CurveTrait<T> for PulseCurve<T> {
    fn new() -> PulseCurve<T> {
        PulseCurve {
            curve: Curve::new(),
        }
    }

    fn insert_keyframe(&mut self, tick: GameTick, keyframe: T) {
        self.curve.insert_keyframe(tick, keyframe);
    }

    fn remove_keyframe(&mut self, tick: GameTick) {
        self.curve.remove_keyframe(tick);
    }

    fn get_keyframe(&self, tick: GameTick) -> Option<&T> {
        self.curve.get_keyframe(tick)
    }

    fn iter_future_curves(&self, tick: GameTick) -> Vec<(&GameTick, &T)> {
        self.curve.iter_future_curves(tick)
    }

    fn next_keyframe(&self, tick: GameTick) -> Option<(&GameTick, &T)> {
        self.curve.next_keyframe(tick)
    }

    fn iter_prev_curves(&self, tick: GameTick) -> Vec<(&GameTick, &T)> {
        self.curve.iter_prev_curves(tick)
    }

    fn prev_keyframe(&self, tick: GameTick) -> Option<(&GameTick, &T)> {
        self.curve.prev_keyframe(tick)
    }

    fn get_state(&self, tick: GameTick) -> Option<T> {
        self.get_keyframe(tick).cloned()
    }
}
