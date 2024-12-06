use bevy::math::Vec2;

pub const BOTTOM_WALL_Y_POS: f32 = -300.0;
pub const LEFT_WALL_X_POS: f32 = -450.0;
pub const RIGHT_WALL_X_POS: f32 = 450.0;
pub const TOP_WALL_Y_POS: f32 = 300.0;
pub const WALL_THICKNESS: f32 = 10.0;

pub enum WallLocation {
    Bottom,
    Left,
    Right,
    Top,
}

impl WallLocation {
    pub fn position(&self) -> Vec2 {
        match self {
            WallLocation::Bottom => Vec2::new(0.0, BOTTOM_WALL_Y_POS),
            WallLocation::Left => Vec2::new(LEFT_WALL_X_POS, 0.0),
            WallLocation::Right => Vec2::new(RIGHT_WALL_X_POS, 0.0),
            WallLocation::Top => Vec2::new(0.0, TOP_WALL_Y_POS),
        }
    }

    pub fn size(&self) -> Vec2 {
        let field_height = TOP_WALL_Y_POS - BOTTOM_WALL_Y_POS;
        let field_width = RIGHT_WALL_X_POS - LEFT_WALL_X_POS;

        match self {
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(field_width + WALL_THICKNESS, WALL_THICKNESS)
            }
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, field_height + WALL_THICKNESS)
            }
        }
    }
}
