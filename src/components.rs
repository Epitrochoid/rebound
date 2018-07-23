use ggez::nalgebra as na;

use specs::{
    Component,
    VecStorage
};

use drawables;

#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Position(pub na::Vector2<f32>);

#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Kinematics {
    pub velocity: na::Vector2<f32>,
    pub acceleration: na::Vector2<f32>
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Draw(pub drawables::Ball);
