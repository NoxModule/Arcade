use bevy::{
    app::{App, Plugin},
    math::{
        bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
        Vec2,
    },
    prelude::{Commands, Component, Entity, Event, EventWriter, Query, Transform, With},
    sprite::Sprite,
};

use super::{
    ball_plugin::{Ball, BALL_SIZE},
    brick_plugin::Brick,
    components::Velocity,
};

#[derive(Component)]
pub struct Collider;

#[derive(Default, Event)]
pub struct CollisionEvent;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Collision {
    Bottom,
    Left,
    Right,
    Top,
}

pub struct ColliderPlugin;
impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();
    }
}

impl ColliderPlugin {
    pub fn check_collisions(
        mut commands: Commands,
        mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
        collider_query: Query<(Entity, &Sprite, &Transform, Option<&Brick>), With<Collider>>,
        mut collision_events: EventWriter<CollisionEvent>,
    ) {
        let (mut ball_velocity, ball_transform) = ball_query.single_mut();

        for (collider_entity, collider_sprite, collider_transform, maybe_brick) in &collider_query {
            let collision = ColliderPlugin::ball_collision(
                BoundingCircle::new(ball_transform.translation.truncate(), BALL_SIZE.y / 2.0),
                Aabb2d::new(
                    collider_transform.translation.truncate(),
                    collider_sprite.custom_size.unwrap_or(Vec2::splat(1.0)) / 2.0,
                ),
            );

            if let Some(collision) = collision {
                collision_events.send_default();

                if maybe_brick.is_some() {
                    commands.entity(collider_entity).despawn();
                }

                let mut reflect_x = false;
                let mut reflect_y = false;

                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.0,
                    Collision::Right => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                }

                if reflect_x {
                    ball_velocity.x = -ball_velocity.x;
                }

                if reflect_y {
                    ball_velocity.y = -ball_velocity.y;
                }
            }
        }
    }

    fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
        if !ball.intersects(&bounding_box) {
            return None;
        }

        let closest = bounding_box.closest_point(ball.center());
        let offset = ball.center() - closest;
        let side = if offset.x.abs() > offset.y.abs() {
            if offset.x < 0.0 {
                Collision::Left
            } else {
                Collision::Right
            }
        } else if offset.y > 0.0 {
            Collision::Top
        } else {
            Collision::Bottom
        };

        Some(side)
    }
}
