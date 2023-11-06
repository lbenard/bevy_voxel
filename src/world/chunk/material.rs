use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::{
        mesh::MeshVertexAttribute,
        render_resource::{AsBindGroup, VertexBufferLayout, VertexStepMode},
    },
};

pub type TerrainMaterial = ExtendedMaterial<StandardMaterial, StandardMaterialExtension>;

pub const ATTRIBUTE_VOXEL_ID: MeshVertexAttribute = MeshVertexAttribute::new(
    "VertexId",
    10461101531982422,
    bevy::render::render_resource::VertexFormat::Uint32,
);

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct StandardMaterialExtension {}

impl MaterialExtension for StandardMaterialExtension {
    fn specialize(
        _pipeline: &bevy::pbr::MaterialExtensionPipeline,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        layout: &bevy::render::mesh::MeshVertexBufferLayout,
        _key: bevy::pbr::MaterialExtensionKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[ATTRIBUTE_VOXEL_ID.at_shader_location(8)])?;

        let new_buffer_layout: VertexBufferLayout = VertexBufferLayout {
            array_stride: descriptor.vertex.buffers[0].array_stride,
            step_mode: VertexStepMode::Vertex,
            attributes: [
                descriptor.vertex.buffers[0].attributes.clone(),
                vertex_layout.attributes,
            ]
            .concat(),
        };
        descriptor.vertex.buffers = [new_buffer_layout].into();

        Ok(())
    }

    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/terrain.wgsl".into()
    }

    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/terrain.wgsl".into()
    }
}
