use self::{material::Material, shape::Shape};

pub mod material;
pub mod shape;

#[derive(Copy, Clone)]
pub struct Block {
    pub shape: Shape,
    pub material: Material,
}
