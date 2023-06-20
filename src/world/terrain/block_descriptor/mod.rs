use self::{material::Material, shape::Shape};

pub mod material;
pub mod shape;

// TODO: Split into two structs, a block that represent any block (shape + material), and a world block (absolute position, properties, and shape + material)
#[derive(Copy, Clone, Debug)]
pub struct BlockDescriptor {
    pub shape: Shape,
    pub material: Material,
}
