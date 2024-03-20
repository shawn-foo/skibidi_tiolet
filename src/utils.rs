use bevy::math::bounding::Aabb2d;
use bevy::math::bounding::IntersectsVolume;
use bevy::math::bounding::BoundingVolume;

use std::fmt;
use std::fmt::write;

#[macro_export]
macro_rules! console_log {
    ( $( $x:expr ),* ) => {
        {
            use {web_sys::console::log_1, convert_js::ToJs, dynfmt::{Format, SimpleCurlyFormat}};
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.to_string());
            )*
            let first_element = temp_vec.remove(0);
            log_1(&SimpleCurlyFormat.format(first_element.as_str(), temp_vec).expect("error in converting to js into js").to_js())
        }
    };
}

pub struct Size{
    pub width: f32,
    pub height: f32
}
impl Size{
    pub fn new(width: f32, height: f32) -> Self{
        Self { width, height }
    }
}
pub enum Collision{
    LEFT,
    TOP,
    BOTTOM,
    RIGHT
}
impl fmt::Display for Collision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Collision::BOTTOM => write!(f, "bottom"),
            Collision::TOP => write!(f, "top"),
            Collision::LEFT => write!(f, "left"),
            Collision::RIGHT => write!(f, "right"),
        }
    }
}
pub fn check_collision(ball: Aabb2d, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest = wall.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::LEFT
        } else {
            Collision::RIGHT
        }
    } else if offset.y > 0. {
        Collision::TOP
    } else {
        Collision::BOTTOM
    };

    Some(side)
}