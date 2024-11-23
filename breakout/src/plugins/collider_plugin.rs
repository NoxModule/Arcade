use bevy::{
    app::{App, Plugin},
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::{Component, Event, EventWriter, Query, Transform, With},
};

use crate::components::Velocity;

use super::ball_plugin::{Ball, BALL_DIAMETER};

#[derive(Component)]
pub struct Collider;

#[derive(Default, Event)]
pub struct CollisionEvent;

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Collision {
    Bottom,
    Left,
    Right,
    Top,
}

impl ColliderPlugin {
    pub fn check_collisions(
        mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
        collider_query: Query<&Transform, With<Collider>>,
        mut collision_events: EventWriter<CollisionEvent>,
    ) {
        let (mut ball_velocity, ball_transform) = ball_query.single_mut();

        for collider_transform in &collider_query {
            let collision = ColliderPlugin::ball_collision(
                BoundingCircle::new(ball_transform.translation.truncate(), BALL_DIAMETER / 2.0),
                Aabb2d::new(
                    collider_transform.translation.truncate(),
                    collider_transform.scale.truncate() / 2.0,
                ),
            );

            if let Some(collision) = collision {
                collision_events.send_default();

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
