use bevy::asset::Handle;
use bevy::math::Vec4;
use bevy::pbr::{
    AlphaMode, Material, MaterialPipeline, MaterialPipelineKey, PBR_PREPASS_SHADER_HANDLE,
};
use bevy::prelude::Mesh;
use bevy::reflect::{std_traits::ReflectDefault, FromReflect, Reflect, TypeUuid};
use bevy::render::mesh::MeshVertexAttribute;
use bevy::render::{
    color::Color, mesh::MeshVertexBufferLayout, render_asset::RenderAssets, render_resource::*,
    texture::Image,
};

pub const ATTRIBUTE_BASE_VOXEL_ID: MeshVertexAttribute =
    MeshVertexAttribute::new("BaseVoxelIndices", 281114372, VertexFormat::Uint32);

#[derive(AsBindGroup, Reflect, Debug, Clone, TypeUuid)]
#[uuid = "3e82975c-30cf-48bf-a825-9f21d0749070"]
#[bind_group_data(TerrainMaterialKey)]
#[uniform(0, TerrainMaterialUniform)]
#[reflect(Default, Debug)]
pub struct TerrainMaterial {
    pub base_color: Color,

    #[texture(1)]
    #[sampler(2)]
    pub base_color_texture: Option<Handle<Image>>,

    // Use a color for user friendliness even though we technically don't use the alpha channel
    // Might be used in the future for exposure correction in HDR
    pub emissive: Color,

    #[texture(3)]
    #[sampler(4)]
    pub emissive_texture: Option<Handle<Image>>,

    //
    // Technically for 32-bit floats, 0.045 could be used.
    // See <https://google.github.io/filament/Filament.html#materialsystem/parameterization/>
    pub perceptual_roughness: f32,

    pub metallic: f32,

    #[texture(5)]
    #[sampler(6)]
    pub metallic_roughness_texture: Option<Handle<Image>>,

    #[doc(alias = "specular_intensity")]
    pub reflectance: f32,

    #[texture(9)]
    #[sampler(10)]
    pub normal_map_texture: Option<Handle<Image>>,

    pub flip_normal_map_y: bool,

    #[texture(7)]
    #[sampler(8)]
    pub occlusion_texture: Option<Handle<Image>>,

    pub double_sided: bool,

    // TODO: include this in reflection somehow (maybe via remote types like serde https://serde.rs/remote-derive.html)
    #[reflect(ignore)]
    pub cull_mode: Option<Face>,

    pub unlit: bool,

    pub fog_enabled: bool,

    pub alpha_mode: AlphaMode,

    pub depth_bias: f32,
}

impl Default for TerrainMaterial {
    fn default() -> Self {
        TerrainMaterial {
            // White because it gets multiplied with texture values if someone uses
            // a texture.
            base_color: Color::rgb(1.0, 1.0, 1.0),
            base_color_texture: None,
            emissive: Color::BLACK,
            emissive_texture: None,
            // Matches Blender's default roughness.
            perceptual_roughness: 0.5,
            // Metallic should generally be set to 0.0 or 1.0.
            metallic: 0.0,
            metallic_roughness_texture: None,
            // Minimum real-world reflectance is 2%, most materials between 2-5%
            // Expressed in a linear scale and equivalent to 4% reflectance see
            // <https://google.github.io/filament/Material%20Properties.pdf>
            reflectance: 0.5,
            occlusion_texture: None,
            normal_map_texture: None,
            flip_normal_map_y: false,
            double_sided: false,
            cull_mode: Some(Face::Back),
            unlit: false,
            fog_enabled: true,
            alpha_mode: AlphaMode::Opaque,
            depth_bias: 0.0,
        }
    }
}

impl From<Color> for TerrainMaterial {
    fn from(color: Color) -> Self {
        TerrainMaterial {
            base_color: color,
            alpha_mode: if color.a() < 1.0 {
                AlphaMode::Blend
            } else {
                AlphaMode::Opaque
            },
            ..Default::default()
        }
    }
}

impl From<Handle<Image>> for TerrainMaterial {
    fn from(texture: Handle<Image>) -> Self {
        TerrainMaterial {
            base_color_texture: Some(texture),
            ..Default::default()
        }
    }
}

// NOTE: These must match the bit flags in bevy_pbr/src/render/pbr_types.wgsl!
bitflags::bitflags! {
    #[repr(transparent)]
    pub struct TerrainMaterialFlags: u32 {
        const BASE_COLOR_TEXTURE         = (1 << 0);
        const EMISSIVE_TEXTURE           = (1 << 1);
        const METALLIC_ROUGHNESS_TEXTURE = (1 << 2);
        const OCCLUSION_TEXTURE          = (1 << 3);
        const DOUBLE_SIDED               = (1 << 4);
        const UNLIT                      = (1 << 5);
        const TWO_COMPONENT_NORMAL_MAP   = (1 << 6);
        const FLIP_NORMAL_MAP_Y          = (1 << 7);
        const FOG_ENABLED                = (1 << 8);
        const ALPHA_MODE_RESERVED_BITS   = (Self::ALPHA_MODE_MASK_BITS << Self::ALPHA_MODE_SHIFT_BITS); // ← Bitmask reserving bits for the `AlphaMode`
        const ALPHA_MODE_OPAQUE          = (0 << Self::ALPHA_MODE_SHIFT_BITS);                          // ← Values are just sequential values bitshifted into
        const ALPHA_MODE_MASK            = (1 << Self::ALPHA_MODE_SHIFT_BITS);                          //   the bitmask, and can range from 0 to 7.
        const ALPHA_MODE_BLEND           = (2 << Self::ALPHA_MODE_SHIFT_BITS);                          //
        const ALPHA_MODE_PREMULTIPLIED   = (3 << Self::ALPHA_MODE_SHIFT_BITS);                          //
        const ALPHA_MODE_ADD             = (4 << Self::ALPHA_MODE_SHIFT_BITS);                          //   Right now only values 0–5 are used, which still gives
        const ALPHA_MODE_MULTIPLY        = (5 << Self::ALPHA_MODE_SHIFT_BITS);                          // ← us "room" for two more modes without adding more bits
        const NONE                       = 0;
        const UNINITIALIZED              = 0xFFFF;
    }
}

impl TerrainMaterialFlags {
    const ALPHA_MODE_MASK_BITS: u32 = 0b111;
    const ALPHA_MODE_SHIFT_BITS: u32 = 32 - Self::ALPHA_MODE_MASK_BITS.count_ones();
}

#[derive(Clone, Default, ShaderType)]
pub struct TerrainMaterialUniform {
    pub base_color: Vec4,
    // Use a color for user friendliness even though we technically don't use the alpha channel
    // Might be used in the future for exposure correction in HDR
    pub emissive: Vec4,
    pub roughness: f32,
    pub metallic: f32,
    pub reflectance: f32,
    pub flags: u32,
    pub alpha_cutoff: f32,
}

impl AsBindGroupShaderType<TerrainMaterialUniform> for TerrainMaterial {
    fn as_bind_group_shader_type(&self, images: &RenderAssets<Image>) -> TerrainMaterialUniform {
        let mut flags = TerrainMaterialFlags::NONE;
        if self.base_color_texture.is_some() {
            flags |= TerrainMaterialFlags::BASE_COLOR_TEXTURE;
        }
        if self.emissive_texture.is_some() {
            flags |= TerrainMaterialFlags::EMISSIVE_TEXTURE;
        }
        if self.metallic_roughness_texture.is_some() {
            flags |= TerrainMaterialFlags::METALLIC_ROUGHNESS_TEXTURE;
        }
        if self.occlusion_texture.is_some() {
            flags |= TerrainMaterialFlags::OCCLUSION_TEXTURE;
        }
        if self.double_sided {
            flags |= TerrainMaterialFlags::DOUBLE_SIDED;
        }
        if self.unlit {
            flags |= TerrainMaterialFlags::UNLIT;
        }
        if self.fog_enabled {
            flags |= TerrainMaterialFlags::FOG_ENABLED;
        }
        let has_normal_map = self.normal_map_texture.is_some();
        if has_normal_map {
            if let Some(texture) = images.get(self.normal_map_texture.as_ref().unwrap()) {
                match texture.texture_format {
                    // All 2-component unorm formats
                    TextureFormat::Rg8Unorm
                    | TextureFormat::Rg16Unorm
                    | TextureFormat::Bc5RgUnorm
                    | TextureFormat::EacRg11Unorm => {
                        flags |= TerrainMaterialFlags::TWO_COMPONENT_NORMAL_MAP;
                    }
                    _ => {}
                }
            }
            if self.flip_normal_map_y {
                flags |= TerrainMaterialFlags::FLIP_NORMAL_MAP_Y;
            }
        }
        // NOTE: 0.5 is from the glTF default - do we want this?
        let mut alpha_cutoff = 0.5;
        match self.alpha_mode {
            AlphaMode::Opaque => flags |= TerrainMaterialFlags::ALPHA_MODE_OPAQUE,
            AlphaMode::Mask(c) => {
                alpha_cutoff = c;
                flags |= TerrainMaterialFlags::ALPHA_MODE_MASK;
            }
            AlphaMode::Blend => flags |= TerrainMaterialFlags::ALPHA_MODE_BLEND,
            AlphaMode::Premultiplied => flags |= TerrainMaterialFlags::ALPHA_MODE_PREMULTIPLIED,
            AlphaMode::Add => flags |= TerrainMaterialFlags::ALPHA_MODE_ADD,
            AlphaMode::Multiply => flags |= TerrainMaterialFlags::ALPHA_MODE_MULTIPLY,
        };

        TerrainMaterialUniform {
            base_color: self.base_color.as_linear_rgba_f32().into(),
            emissive: self.emissive.as_linear_rgba_f32().into(),
            roughness: self.perceptual_roughness,
            metallic: self.metallic,
            reflectance: self.reflectance,
            flags: flags.bits(),
            alpha_cutoff,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TerrainMaterialKey {
    normal_map: bool,
    cull_mode: Option<Face>,
    depth_bias: i32,
}

impl From<&TerrainMaterial> for TerrainMaterialKey {
    fn from(material: &TerrainMaterial) -> Self {
        TerrainMaterialKey {
            normal_map: material.normal_map_texture.is_some(),
            cull_mode: material.cull_mode,
            depth_bias: material.depth_bias as i32,
        }
    }
}

impl Material for TerrainMaterial {
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(fragment) = descriptor.fragment.as_mut() {
            if key.bind_group_data.normal_map {
                fragment
                    .shader_defs
                    .push("STANDARDMATERIAL_NORMAL_MAP".into());
            }
            if layout.contains(Mesh::ATTRIBUTE_UV_0) {
                fragment.shader_defs.push("VERTEX_UVS".into());
            }
            if layout.contains(ATTRIBUTE_BASE_VOXEL_ID) {
                println!("Using voxel ids in fragment shader");
                fragment.shader_defs.push("VOXEL_IDS".into());
            }
        }
        descriptor.primitive.cull_mode = key.bind_group_data.cull_mode;
        if let Some(label) = &mut descriptor.label {
            *label = format!("pbr_{}", *label).into();
        }
        if let Some(depth_stencil) = descriptor.depth_stencil.as_mut() {
            depth_stencil.bias.constant = key.bind_group_data.depth_bias;
        }
        let mut vertex_attributes = Vec::new();
        vertex_attributes.push(Mesh::ATTRIBUTE_POSITION.at_shader_location(0));
        vertex_attributes.push(Mesh::ATTRIBUTE_NORMAL.at_shader_location(1));
        if layout.contains(Mesh::ATTRIBUTE_UV_0) {
            descriptor.vertex.shader_defs.push("VERTEX_UVS".into());
            vertex_attributes.push(Mesh::ATTRIBUTE_UV_0.at_shader_location(2));
        }
        if layout.contains(ATTRIBUTE_BASE_VOXEL_ID) {
            println!("Using voxel ids in vertex shader");
            descriptor.vertex.shader_defs.push("VOXEL_IDS".into());
            vertex_attributes.push(ATTRIBUTE_BASE_VOXEL_ID.at_shader_location(7));
        }
        let vertex_layout = layout.get_layout(&vertex_attributes)?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }

    fn prepass_fragment_shader() -> ShaderRef {
        PBR_PREPASS_SHADER_HANDLE.typed().into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/pbr.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/pbr.wgsl".into()
    }

    #[inline]
    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    #[inline]
    fn depth_bias(&self) -> f32 {
        self.depth_bias
    }
}
