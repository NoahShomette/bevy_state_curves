use std::any::Any;

use bevy::reflect::{reflect_trait, Reflect};
pub trait SteppedKeyframe: Reflect + SteppedCloneTrait {
    fn self_as_any(&self) -> &dyn Any {
        SteppedCloneTrait::as_any(self)
    }
}

impl Clone for Box<dyn SteppedKeyframe> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait SteppedCloneTrait {
    fn clone_box(&self) -> Box<dyn SteppedKeyframe>;
    // An &Any can be cast to a reference to a concrete type.
    fn as_any(&self) -> &dyn Any;
}

impl<T> SteppedCloneTrait for T
where
    T: 'static + SteppedKeyframe + Clone + ?Sized,
{
    fn clone_box(&self) -> Box<dyn SteppedKeyframe> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait PulseKeyframe: Reflect + PulseCloneTrait {
    fn self_as_any(&self) -> &dyn Any {
        PulseCloneTrait::as_any(self)
    }
}

impl Clone for Box<dyn PulseKeyframe> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait PulseCloneTrait {
    fn clone_box(&self) -> Box<dyn PulseKeyframe>;
    // An &Any can be cast to a reference to a concrete type.
    fn as_any(&self) -> &dyn Any;
}

impl<T> PulseCloneTrait for T
where
    T: 'static + PulseKeyframe + Clone + ?Sized,
{
    fn clone_box(&self) -> Box<dyn PulseKeyframe> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Lerp should be given the last keyframe, the next keyframe, and then it returns the result for the requested [`GameTick`].
#[reflect_trait]
pub trait LinearKeyFrame: Reflect + LerpCloneTrait {
    /// Performs a linear interpolation over the component based on the ratio between starting tick and next_frame_tick.
    /// The implementation decides which fields are interpolated and returns the result for use in systems
    fn lerp(
        &self,
        next_frame_state: &Box<dyn LinearKeyFrame>,
        ratio: f32,
    ) -> Box<dyn LinearKeyFrame>;

    fn self_as_any(&self) -> &dyn Any {
        LerpCloneTrait::as_any(self)
    }
}

impl Clone for Box<dyn LinearKeyFrame> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait LerpCloneTrait {
    fn clone_box(&self) -> Box<dyn LinearKeyFrame>;
    // An &Any can be cast to a reference to a concrete type.
    fn as_any(&self) -> &dyn Any;
}

impl<T> LerpCloneTrait for T
where
    T: 'static + LinearKeyFrame + Clone + ?Sized,
{
    fn clone_box(&self) -> Box<dyn LinearKeyFrame> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
