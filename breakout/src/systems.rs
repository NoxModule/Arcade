use bevy::prelude::{Commands, Component, DespawnRecursiveExt, Entity, Query, With};

pub fn despawn_by<T>(mut commands: Commands, entities_query: Query<Entity, With<T>>)
where
    T: Component,
{
    for entity in &entities_query {
        commands.entity(entity).despawn_recursive();
    }
}
