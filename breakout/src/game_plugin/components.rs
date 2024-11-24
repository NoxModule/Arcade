use bevy::{
    math::Vec2,
    prelude::{Component, Deref, DerefMut},
};

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);
