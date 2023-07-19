// use bevy::asset::Handle;
// use bevy::math::Vec4;
// use bevy::pbr::{
//     AlphaMode, Material, MaterialPipeline, MaterialPipelineKey, ParallaxMappingMethod,
//     PBR_PREPASS_SHADER_HANDLE, PBR_SHADER_HANDLE,
// };
// use bevy::prelude::Mesh;
// use bevy::reflect::{std_traits::ReflectDefault, Reflect, TypeUuid};
// use bevy::render::mesh::{GpuMesh, MeshVertexAttribute};
// use bevy::render::{
//     color::Color, mesh::MeshVertexBufferLayout, render_asset::RenderAssets, render_resource::*,
//     texture::Image,
// };

// pub const ATTRIBUTE_BASE_VOXEL_ID: MeshVertexAttribute =
//     MeshVertexAttribute::new("BaseVoxelIndices", 281114372, VertexFormat::Uint32);

// #[derive(AsBindGroup, Reflect, Debug, Clone, TypeUuid)]
// #[uuid = "3e82975c-30cf-48bf-a825-9f21d0749070"]
// #[bind_group_data(TerrainMaterialKey)]
// #[uniform(0, TerrainMaterialUniform)]
// #[reflect(Default, Debug)]
// pub struct TerrainMaterial {
//     pub base_color: Color,

//     #[texture(1)]
//     #[sampler(2)]
//     pub base_color_texture: Option<Handle<Image>>,
//     pub emissive: Color,

//     #[texture(3)]
//     #[sampler(4)]
//     pub emissive_texture: Option<Handle<Image>>,
//     pub perceptual_roughness: f32,
//     pub metallic: f32,

//     #[texture(5)]
//     #[sampler(6)]
//     pub metallic_roughness_texture: Option<Handle<Image>>,
//     #[doc(alias = "specular_intensity")]
//     pub reflectance: f32,

//     #[texture(9)]
//     #[sampler(10)]
//     pub normal_map_texture: Option<Handle<Image>>,
//     pub flip_normal_map_y: bool,
//     #[texture(7)]
//     #[sampler(8)]
//     pub occlusion_texture: Option<Handle<Image>>,
//     pub double_sided: bool,

//     #[reflect(ignore)]
//     pub cull_mode: Option<Face>,
//     pub unlit: bool,
//     pub fog_enabled: bool,
//     pub alpha_mode: AlphaMode,
//     pub depth_bias: f32,

//     #[texture(11)]
//     #[sampler(12)]
//     pub depth_map: Option<Handle<Image>>,
//     pub parallax_depth_scale: f32,
//     pub parallax_mapping_method: ParallaxMappingMethod,
//     pub max_parallax_layer_count: f32,
// }

// impl Default for TerrainMaterial {
//     fn default() -> Self {
//         TerrainMaterial {
//             // White because it gets multiplied with texture values if someone uses
//             // a texture.
//             base_color: Color::rgb(1.0, 1.0, 1.0),
//             base_color_texture: None,
//             emissive: Color::BLACK,
//             emissive_texture: None,
//             // Matches Blender's default roughness.
//             perceptual_roughness: 0.5,
//             // Metallic should generally be set to 0.0 or 1.0.
//             metallic: 0.0,
//             metallic_roughness_texture: None,
//             // Minimum real-world reflectance is 2%, most materials between 2-5%
//             // Expressed in a linear scale and equivalent to 4% reflectance see
//             // <https://google.github.io/filament/Material%20Properties.pdf>
//             reflectance: 0.5,
//             occlusion_texture: None,
//             normal_map_texture: None,
//             flip_normal_map_y: false,
//             double_sided: false,
//             cull_mode: Some(Face::Back),
//             unlit: false,
//             fog_enabled: true,
//             alpha_mode: AlphaMode::Opaque,
//             depth_bias: 0.0,
//             depth_map: None,
//             parallax_depth_scale: 0.1,
//             max_parallax_layer_count: 16.0,
//             parallax_mapping_method: ParallaxMappingMethod::Occlusion,
//         }
//     }
// }

// impl From<Color> for TerrainMaterial {
//     fn from(color: Color) -> Self {
//         TerrainMaterial {
//             base_color: color,
//             alpha_mode: if color.a() < 1.0 {
//                 AlphaMode::Blend
//             } else {
//                 AlphaMode::Opaque
//             },
//             ..Default::default()
//         }
//     }
// }

// impl From<Handle<Image>> for TerrainMaterial {
//     fn from(texture: Handle<Image>) -> Self {
//         TerrainMaterial {
//             base_color_texture: Some(texture),
//             ..Default::default()
//         }
//     }
// }

// // NOTE: These must match the bit flags in bevy_pbr/src/render/pbr_types.wgsl!
// bitflags::bitflags! {
//     #[repr(transparent)]
//     pub struct TerrainMaterialFlags: u32 {
//         const BASE_COLOR_TEXTURE         = (1 << 0);
//         const EMISSIVE_TEXTURE           = (1 << 1);
//         const METALLIC_ROUGHNESS_TEXTURE = (1 << 2);
//         const OCCLUSION_TEXTURE          = (1 << 3);
//         const DOUBLE_SIDED               = (1 << 4);
//         const UNLIT                      = (1 << 5);
//         const TWO_COMPONENT_NORMAL_MAP   = (1 << 6);
//         const FLIP_NORMAL_MAP_Y          = (1 << 7);
//         const FOG_ENABLED                = (1 << 8);
//         const DEPTH_MAP                  = (1 << 9); // Used for parallax mapping
//         const ALPHA_MODE_RESERVED_BITS   = (Self::ALPHA_MODE_MASK_BITS << Self::ALPHA_MODE_SHIFT_BITS); // ← Bitmask reserving bits for the `AlphaMode`
//         const ALPHA_MODE_OPAQUE          = (0 << Self::ALPHA_MODE_SHIFT_BITS);                          // ← Values are just sequential values bitshifted into
//         const ALPHA_MODE_MASK            = (1 << Self::ALPHA_MODE_SHIFT_BITS);                          //   the bitmask, and can range from 0 to 7.
//         const ALPHA_MODE_BLEND           = (2 << Self::ALPHA_MODE_SHIFT_BITS);                          //
//         const ALPHA_MODE_PREMULTIPLIED   = (3 << Self::ALPHA_MODE_SHIFT_BITS);                          //
//         const ALPHA_MODE_ADD             = (4 << Self::ALPHA_MODE_SHIFT_BITS);                          //   Right now only values 0–5 are used, which still gives
//         const ALPHA_MODE_MULTIPLY        = (5 << Self::ALPHA_MODE_SHIFT_BITS);                          // ← us "room" for two more modes without adding more bits
//         const NONE                       = 0;
//         const UNINITIALIZED              = 0xFFFF;
//     }
// }

// impl TerrainMaterialFlags {
//     const ALPHA_MODE_MASK_BITS: u32 = 0b111;
//     const ALPHA_MODE_SHIFT_BITS: u32 = 32 - Self::ALPHA_MODE_MASK_BITS.count_ones();
// }

// #[derive(Clone, Default, ShaderType)]
// pub struct TerrainMaterialUniform {
//     pub base_color: Vec4,
//     pub emissive: Vec4,
//     pub roughness: f32,
//     pub metallic: f32,
//     pub reflectance: f32,
//     pub flags: u32,
//     pub alpha_cutoff: f32,
//     pub parallax_depth_scale: f32,
//     pub max_parallax_layer_count: f32,
//     pub max_relief_mapping_search_steps: u32,
// }

// impl AsBindGroupShaderType<TerrainMaterialUniform> for TerrainMaterial {
//     fn as_bind_group_shader_type(&self, images: &RenderAssets<Image>) -> TerrainMaterialUniform {
//         let mut flags = TerrainMaterialFlags::NONE;
//         if self.base_color_texture.is_some() {
//             flags |= TerrainMaterialFlags::BASE_COLOR_TEXTURE;
//         }
//         if self.emissive_texture.is_some() {
//             flags |= TerrainMaterialFlags::EMISSIVE_TEXTURE;
//         }
//         if self.metallic_roughness_texture.is_some() {
//             flags |= TerrainMaterialFlags::METALLIC_ROUGHNESS_TEXTURE;
//         }
//         if self.occlusion_texture.is_some() {
//             flags |= TerrainMaterialFlags::OCCLUSION_TEXTURE;
//         }
//         if self.double_sided {
//             flags |= TerrainMaterialFlags::DOUBLE_SIDED;
//         }
//         if self.unlit {
//             flags |= TerrainMaterialFlags::UNLIT;
//         }
//         if self.fog_enabled {
//             flags |= TerrainMaterialFlags::FOG_ENABLED;
//         }
//         if self.depth_map.is_some() {
//             flags |= TerrainMaterialFlags::DEPTH_MAP;
//         }
//         let has_normal_map = self.normal_map_texture.is_some();
//         if has_normal_map {
//             if let Some(texture) = images.get(self.normal_map_texture.as_ref().unwrap()) {
//                 match texture.texture_format {
//                     // All 2-component unorm formats
//                     TextureFormat::Rg8Unorm
//                     | TextureFormat::Rg16Unorm
//                     | TextureFormat::Bc5RgUnorm
//                     | TextureFormat::EacRg11Unorm => {
//                         flags |= TerrainMaterialFlags::TWO_COMPONENT_NORMAL_MAP;
//                     }
//                     _ => {}
//                 }
//             }
//             if self.flip_normal_map_y {
//                 flags |= TerrainMaterialFlags::FLIP_NORMAL_MAP_Y;
//             }
//         }
//         // NOTE: 0.5 is from the glTF default - do we want this?
//         let mut alpha_cutoff = 0.5;
//         match self.alpha_mode {
//             AlphaMode::Opaque => flags |= TerrainMaterialFlags::ALPHA_MODE_OPAQUE,
//             AlphaMode::Mask(c) => {
//                 alpha_cutoff = c;
//                 flags |= TerrainMaterialFlags::ALPHA_MODE_MASK;
//             }
//             AlphaMode::Blend => flags |= TerrainMaterialFlags::ALPHA_MODE_BLEND,
//             AlphaMode::Premultiplied => flags |= TerrainMaterialFlags::ALPHA_MODE_PREMULTIPLIED,
//             AlphaMode::Add => flags |= TerrainMaterialFlags::ALPHA_MODE_ADD,
//             AlphaMode::Multiply => flags |= TerrainMaterialFlags::ALPHA_MODE_MULTIPLY,
//         };

//         TerrainMaterialUniform {
//             base_color: self.base_color.as_linear_rgba_f32().into(),
//             emissive: self.emissive.as_linear_rgba_f32().into(),
//             roughness: self.perceptual_roughness,
//             metallic: self.metallic,
//             reflectance: self.reflectance,
//             flags: flags.bits(),
//             alpha_cutoff,
//             parallax_depth_scale: self.parallax_depth_scale,
//             max_parallax_layer_count: self.max_parallax_layer_count,
//             max_relief_mapping_search_steps: 0,
//             // max_relief_mapping_search_steps: self.parallax_mapping_method.max_steps(),
//         }
//     }
// }

// #[derive(Clone, PartialEq, Eq, Hash)]
// pub struct TerrainMaterialKey {
//     normal_map: bool,
//     cull_mode: Option<Face>,
//     depth_bias: i32,
//     relief_mapping: bool,
// }

// impl From<&TerrainMaterial> for TerrainMaterialKey {
//     fn from(material: &TerrainMaterial) -> Self {
//         TerrainMaterialKey {
//             normal_map: material.normal_map_texture.is_some(),
//             cull_mode: material.cull_mode,
//             depth_bias: material.depth_bias as i32,
//             relief_mapping: matches!(
//                 material.parallax_mapping_method,
//                 ParallaxMappingMethod::Relief { .. }
//             ),
//         }
//     }
// }

// // impl Material for TerrainMaterial {
// //     fn specialize(
// //         _pipeline: &MaterialPipeline<Self>,
// //         descriptor: &mut RenderPipelineDescriptor,
// //         _layout: &MeshVertexBufferLayout,
// //         key: MaterialPipelineKey<Self>,
// //     ) -> Result<(), SpecializedMeshPipelineError> {
// //         if let Some(fragment) = descriptor.fragment.as_mut() {
// //             let shader_defs = &mut fragment.shader_defs;

// //             if key.bind_group_data.normal_map {
// //                 shader_defs.push("STANDARDMATERIAL_NORMAL_MAP".into());
// //             }
// //             if key.bind_group_data.relief_mapping {
// //                 shader_defs.push("RELIEF_MAPPING".into());
// //             }
// //         }
// //         descriptor.primitive.cull_mode = key.bind_group_data.cull_mode;
// //         if let Some(label) = &mut descriptor.label {
// //             *label = format!("pbr_{}", *label).into();
// //         }
// //         if let Some(depth_stencil) = descriptor.depth_stencil.as_mut() {
// //             depth_stencil.bias.constant = key.bind_group_data.depth_bias;
// //         }
// //         Ok(())
// //     }

// //     fn prepass_fragment_shader() -> ShaderRef {
// //         PBR_PREPASS_SHADER_HANDLE.typed().into()
// //     }

// //     fn fragment_shader() -> ShaderRef {
// //         PBR_SHADER_HANDLE.typed().into()
// //     }

// //     #[inline]
// //     fn alpha_mode(&self) -> AlphaMode {
// //         self.alpha_mode
// //     }

// //     #[inline]
// //     fn depth_bias(&self) -> f32 {
// //         self.depth_bias
// //     }
// // }

// impl Material for TerrainMaterial {
//     fn specialize(
//         _pipeline: &MaterialPipeline<Self>,
//         descriptor: &mut RenderPipelineDescriptor,
//         layout: &MeshVertexBufferLayout,
//         key: MaterialPipelineKey<Self>,
//     ) -> Result<(), SpecializedMeshPipelineError> {
//         if let Some(fragment) = descriptor.fragment.as_mut() {
//             if key.bind_group_data.normal_map {
//                 fragment
//                     .shader_defs
//                     .push("STANDARDMATERIAL_NORMAL_MAP".into());
//             }
//             if layout.contains(Mesh::ATTRIBUTE_UV_0) {
//                 fragment.shader_defs.push("VERTEX_UVS".into());
//             }
//             // if layout.contains(ATTRIBUTE_BASE_VOXEL_ID) {
//             //     println!("Using voxel ids in fragment shader");
//             //     fragment.shader_defs.push("VOXEL_IDS".into());
//             // }
//         }
//         descriptor.primitive.cull_mode = key.bind_group_data.cull_mode;
//         if let Some(label) = &mut descriptor.label {
//             *label = format!("pbr_{}", *label).into();
//         }
//         if let Some(depth_stencil) = descriptor.depth_stencil.as_mut() {
//             depth_stencil.bias.constant = key.bind_group_data.depth_bias;
//         }
//         let mut vertex_attributes = Vec::new();
//         vertex_attributes.push(Mesh::ATTRIBUTE_POSITION.at_shader_location(0));
//         vertex_attributes.push(Mesh::ATTRIBUTE_NORMAL.at_shader_location(1));
//         if layout.contains(Mesh::ATTRIBUTE_UV_0) {
//             descriptor.vertex.shader_defs.push("VERTEX_UVS".into());
//             vertex_attributes.push(Mesh::ATTRIBUTE_UV_0.at_shader_location(2));
//         }
//         // if layout.contains(ATTRIBUTE_BASE_VOXEL_ID) {
//         //     println!("Using voxel ids in vertex shader");
//         //     descriptor.vertex.shader_defs.push("VOXEL_IDS".into());
//         //     vertex_attributes.push(ATTRIBUTE_BASE_VOXEL_ID.at_shader_location(7));
//         // }
//         let vertex_layout = layout.get_layout(&vertex_attributes)?;
//         descriptor.vertex.buffers = vec![vertex_layout];
//         Ok(())
//     }

//     fn prepass_fragment_shader() -> ShaderRef {
//         PBR_PREPASS_SHADER_HANDLE.typed().into()
//     }

//     fn vertex_shader() -> ShaderRef {
//         "shaders/pbr.wgsl".into()
//     }

//     fn fragment_shader() -> ShaderRef {
//         "shaders/pbr.wgsl".into()
//     }

//     #[inline]
//     fn alpha_mode(&self) -> AlphaMode {
//         self.alpha_mode
//     }

//     #[inline]
//     fn depth_bias(&self) -> f32 {
//         self.depth_bias
//     }
// }

use bevy::asset::Handle;
use bevy::math::Vec4;
use bevy::pbr::{
    AlphaMode, Material, MaterialPipeline, MaterialPipelineKey, ParallaxMappingMethod,
    PBR_PREPASS_SHADER_HANDLE,
};
use bevy::reflect::{std_traits::ReflectDefault, Reflect, TypeUuid};
use bevy::render::{
    color::Color, mesh::MeshVertexBufferLayout, render_asset::RenderAssets, render_resource::*,
    texture::Image,
};

#[derive(AsBindGroup, Reflect, Debug, Clone, TypeUuid)]
#[uuid = "3e82975c-30cf-48bf-a825-9f21d0749070"]
#[bind_group_data(TerrainMaterialKey)]
#[uniform(0, TerrainMaterialUniform)]
#[reflect(Default, Debug)]
pub struct TerrainMaterial {
    pub base_color: Color,

    /// The texture component of the material's color before lighting.
    /// The actual pre-lighting color is `base_color * this_texture`.
    ///
    /// See [`base_color`] for details.
    ///
    /// You should set `base_color` to [`Color::WHITE`] (the default)
    /// if you want the texture to show as-is.
    ///
    /// Setting `base_color` to something else than white will tint
    /// the texture. For example, setting `base_color` to pure red will
    /// tint the texture red.
    ///
    /// [`base_color`]: TerrainMaterial::base_color
    #[texture(1)]
    #[sampler(2)]
    pub base_color_texture: Option<Handle<Image>>,

    // Use a color for user friendliness even though we technically don't use the alpha channel
    // Might be used in the future for exposure correction in HDR
    /// Color the material "emits" to the camera.
    ///
    /// This is typically used for monitor screens or LED lights.
    /// Anything that can be visible even in darkness.
    ///
    /// The emissive color is added to what would otherwise be the material's visible color.
    /// This means that for a light emissive value, in darkness,
    /// you will mostly see the emissive component.
    ///
    /// The default emissive color is black, which doesn't add anything to the material color.
    ///
    /// Note that **an emissive material won't light up surrounding areas like a light source**,
    /// it just adds a value to the color seen on screen.
    pub emissive: Color,

    /// The emissive map, multiplies pixels with [`emissive`]
    /// to get the final "emitting" color of a surface.
    ///
    /// This color is multiplied by [`emissive`] to get the final emitted color.
    /// Meaning that you should set [`emissive`] to [`Color::WHITE`]
    /// if you want to use the full range of color of the emissive texture.
    ///
    /// [`emissive`]: TerrainMaterial::emissive
    #[texture(3)]
    #[sampler(4)]
    pub emissive_texture: Option<Handle<Image>>,

    /// Linear perceptual roughness, clamped to `[0.089, 1.0]` in the shader.
    ///
    /// Defaults to `0.5`.
    ///
    /// Low values result in a "glossy" material with specular highlights,
    /// while values close to `1` result in rough materials.
    ///
    /// If used together with a roughness/metallic texture, this is factored into the final base
    /// color as `roughness * roughness_texture_value`.
    ///
    /// 0.089 is the minimum floating point value that won't be rounded down to 0 in the
    /// calculations used.
    //
    // Technically for 32-bit floats, 0.045 could be used.
    // See <https://google.github.io/filament/Filament.html#materialsystem/parameterization/>
    pub perceptual_roughness: f32,

    /// How "metallic" the material appears, within `[0.0, 1.0]`.
    ///
    /// This should be set to 0.0 for dielectric materials or 1.0 for metallic materials.
    /// For a hybrid surface such as corroded metal, you may need to use in-between values.
    ///
    /// Defaults to `0.00`, for dielectric.
    ///
    /// If used together with a roughness/metallic texture, this is factored into the final base
    /// color as `metallic * metallic_texture_value`.
    pub metallic: f32,

    /// Metallic and roughness maps, stored as a single texture.
    ///
    /// The blue channel contains metallic values,
    /// and the green channel contains the roughness values.
    /// Other channels are unused.
    ///
    /// Those values are multiplied by the scalar ones of the material,
    /// see [`metallic`] and [`perceptual_roughness`] for details.
    ///
    /// Note that with the default values of [`metallic`] and [`perceptual_roughness`],
    /// setting this texture has no effect. If you want to exclusively use the
    /// `metallic_roughness_texture` values for your material, make sure to set [`metallic`]
    /// and [`perceptual_roughness`] to `1.0`.
    ///
    /// [`metallic`]: TerrainMaterial::metallic
    /// [`perceptual_roughness`]: TerrainMaterial::perceptual_roughness
    #[texture(5)]
    #[sampler(6)]
    pub metallic_roughness_texture: Option<Handle<Image>>,

    /// Specular intensity for non-metals on a linear scale of `[0.0, 1.0]`.
    ///
    /// Use the value as a way to control the intensity of the
    /// specular highlight of the material, i.e. how reflective is the material,
    /// rather than the physical property "reflectance."
    ///
    /// Set to `0.0`, no specular highlight is visible, the highlight is strongest
    /// when `reflectance` is set to `1.0`.
    ///
    /// Defaults to `0.5` which is mapped to 4% reflectance in the shader.
    #[doc(alias = "specular_intensity")]
    pub reflectance: f32,

    /// Used to fake the lighting of bumps and dents on a material.
    ///
    /// A typical usage would be faking cobblestones on a flat plane mesh in 3D.
    ///
    /// # Notes
    ///
    /// Normal mapping with `TerrainMaterial` and the core bevy PBR shaders requires:
    /// - A normal map texture
    /// - Vertex UVs
    /// - Vertex tangents
    /// - Vertex normals
    ///
    /// Tangents do not have to be stored in your model,
    /// they can be generated using the [`Mesh::generate_tangents`] method.
    /// If your material has a normal map, but still renders as a flat surface,
    /// make sure your meshes have their tangents set.
    ///
    /// [`Mesh::generate_tangents`]: bevy_render::mesh::Mesh::generate_tangents
    #[texture(9)]
    #[sampler(10)]
    pub normal_map_texture: Option<Handle<Image>>,

    /// Normal map textures authored for DirectX have their y-component flipped. Set this to flip
    /// it to right-handed conventions.
    pub flip_normal_map_y: bool,

    /// Specifies the level of exposure to ambient light.
    ///
    /// This is usually generated and stored automatically ("baked") by 3D-modelling software.
    ///
    /// Typically, steep concave parts of a model (such as the armpit of a shirt) are darker,
    /// because they have little exposure to light.
    /// An occlusion map specifies those parts of the model that light doesn't reach well.
    ///
    /// The material will be less lit in places where this texture is dark.
    /// This is similar to ambient occlusion, but built into the model.
    #[texture(7)]
    #[sampler(8)]
    pub occlusion_texture: Option<Handle<Image>>,

    /// Support two-sided lighting by automatically flipping the normals for "back" faces
    /// within the PBR lighting shader.
    ///
    /// Defaults to `false`.
    /// This does not automatically configure backface culling,
    /// which can be done via `cull_mode`.
    pub double_sided: bool,

    /// Whether to cull the "front", "back" or neither side of a mesh.
    /// If set to `None`, the two sides of the mesh are visible.
    ///
    /// Defaults to `Some(Face::Back)`.
    /// In bevy, the order of declaration of a triangle's vertices
    /// in [`Mesh`] defines the triangle's front face.
    ///
    /// When a triangle is in a viewport,
    /// if its vertices appear counter-clockwise from the viewport's perspective,
    /// then the viewport is seeing the triangle's front face.
    /// Conversely, if the vertices appear clockwise, you are seeing the back face.
    ///
    /// In short, in bevy, front faces winds counter-clockwise.
    ///
    /// Your 3D editing software should manage all of that.
    ///
    /// [`Mesh`]: bevy_render::mesh::Mesh
    // TODO: include this in reflection somehow (maybe via remote types like serde https://serde.rs/remote-derive.html)
    #[reflect(ignore)]
    pub cull_mode: Option<Face>,

    /// Whether to apply only the base color to this material.
    ///
    /// Normals, occlusion textures, roughness, metallic, reflectance, emissive,
    /// shadows, alpha mode and ambient light are ignored if this is set to `true`.
    pub unlit: bool,

    /// Whether to enable fog for this material.
    pub fog_enabled: bool,

    /// How to apply the alpha channel of the `base_color_texture`.
    ///
    /// See [`AlphaMode`] for details. Defaults to [`AlphaMode::Opaque`].
    pub alpha_mode: AlphaMode,

    /// Adjust rendered depth.
    ///
    /// A material with a positive depth bias will render closer to the
    /// camera while negative values cause the material to render behind
    /// other objects. This is independent of the viewport.
    ///
    /// `depth_bias` affects render ordering and depth write operations
    /// using the `wgpu::DepthBiasState::Constant` field.
    ///
    /// [z-fighting]: https://en.wikipedia.org/wiki/Z-fighting
    pub depth_bias: f32,

    /// The depth map used for [parallax mapping].
    ///
    /// It is a greyscale image where white represents bottom and black the top.
    /// If this field is set, bevy will apply [parallax mapping].
    /// Parallax mapping, unlike simple normal maps, will move the texture
    /// coordinate according to the current perspective,
    /// giving actual depth to the texture.
    ///
    /// The visual result is similar to a displacement map,
    /// but does not require additional geometry.
    ///
    /// Use the [`parallax_depth_scale`] field to control the depth of the parallax.
    ///
    /// ## Limitations
    ///
    /// - It will look weird on bent/non-planar surfaces.
    /// - The depth of the pixel does not reflect its visual position, resulting
    ///   in artifacts for depth-dependent features such as fog or SSAO.
    /// - For the same reason, the the geometry silhouette will always be
    ///   the one of the actual geometry, not the parallaxed version, resulting
    ///   in awkward looks on intersecting parallaxed surfaces.
    ///
    /// ## Performance
    ///
    /// Parallax mapping requires multiple texture lookups, proportional to
    /// [`max_parallax_layer_count`], which might be costly.
    ///
    /// Use the [`parallax_mapping_method`] and [`max_parallax_layer_count`] fields
    /// to tweak the shader, trading graphical quality for performance.
    ///
    /// To improve performance, set your `depth_map`'s [`Image::sampler_descriptor`]
    /// filter mode to `FilterMode::Nearest`, as [this paper] indicates, it improves
    /// performance a bit.
    ///
    /// To reduce artifacts, avoid steep changes in depth, blurring the depth
    /// map helps with this.
    ///
    /// Larger depth maps haves a disproportionate performance impact.
    ///
    /// [this paper]: https://www.diva-portal.org/smash/get/diva2:831762/FULLTEXT01.pdf
    /// [parallax mapping]: https://en.wikipedia.org/wiki/Parallax_mapping
    /// [`parallax_depth_scale`]: TerrainMaterial::parallax_depth_scale
    /// [`parallax_mapping_method`]: TerrainMaterial::parallax_mapping_method
    /// [`max_parallax_layer_count`]: TerrainMaterial::max_parallax_layer_count
    #[texture(11)]
    #[sampler(12)]
    pub depth_map: Option<Handle<Image>>,

    /// How deep the offset introduced by the depth map should be.
    ///
    /// Default is `0.1`, anything over that value may look distorted.
    /// Lower values lessen the effect.
    ///
    /// The depth is relative to texture size. This means that if your texture
    /// occupies a surface of `1` world unit, and `parallax_depth_scale` is `0.1`, then
    /// the in-world depth will be of `0.1` world units.
    /// If the texture stretches for `10` world units, then the final depth
    /// will be of `1` world unit.
    pub parallax_depth_scale: f32,

    /// Which parallax mapping method to use.
    ///
    /// We recommend that all objects use the same [`ParallaxMappingMethod`], to avoid
    /// duplicating and running two shaders.
    pub parallax_mapping_method: ParallaxMappingMethod,

    /// In how many layers to split the depth maps for parallax mapping.
    ///
    /// If you are seeing jaggy edges, increase this value.
    /// However, this incurs a performance cost.
    ///
    /// Dependent on the situation, switching to [`ParallaxMappingMethod::Relief`]
    /// and keeping this value low might have better performance than increasing the
    /// layer count while using [`ParallaxMappingMethod::Occlusion`].
    ///
    /// Default is `16.0`.
    pub max_parallax_layer_count: f32,
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
            depth_map: None,
            parallax_depth_scale: 0.1,
            max_parallax_layer_count: 16.0,
            parallax_mapping_method: ParallaxMappingMethod::Occlusion,
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
    /// Bitflags info about the material a shader is currently rendering.
    /// This is accessible in the shader in the [`TerrainMaterialUniform`]
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
        const DEPTH_MAP                  = (1 << 9); // Used for parallax mapping
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

/// The GPU representation of the uniform data of a [`TerrainMaterial`].
#[derive(Clone, Default, ShaderType)]
pub struct TerrainMaterialUniform {
    /// Doubles as diffuse albedo for non-metallic, specular for metallic and a mix for everything
    /// in between.
    pub base_color: Vec4,
    // Use a color for user friendliness even though we technically don't use the alpha channel
    // Might be used in the future for exposure correction in HDR
    pub emissive: Vec4,
    /// Linear perceptual roughness, clamped to [0.089, 1.0] in the shader
    /// Defaults to minimum of 0.089
    pub roughness: f32,
    /// From [0.0, 1.0], dielectric to pure metallic
    pub metallic: f32,
    /// Specular intensity for non-metals on a linear scale of [0.0, 1.0]
    /// defaults to 0.5 which is mapped to 4% reflectance in the shader
    pub reflectance: f32,
    /// The [`TerrainMaterialFlags`] accessible in the `wgsl` shader.
    pub flags: u32,
    /// When the alpha mode mask flag is set, any base color alpha above this cutoff means fully opaque,
    /// and any below means fully transparent.
    pub alpha_cutoff: f32,
    /// The depth of the [`TerrainMaterial::depth_map`] to apply.
    pub parallax_depth_scale: f32,
    /// In how many layers to split the depth maps for Steep parallax mapping.
    ///
    /// If your `parallax_depth_scale` is >0.1 and you are seeing jaggy edges,
    /// increase this value. However, this incurs a performance cost.
    pub max_parallax_layer_count: f32,
    /// Using [`ParallaxMappingMethod::Relief`], how many additional
    /// steps to use at most to find the depth value.
    pub max_relief_mapping_search_steps: u32,
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
        if self.depth_map.is_some() {
            flags |= TerrainMaterialFlags::DEPTH_MAP;
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
            parallax_depth_scale: self.parallax_depth_scale,
            max_parallax_layer_count: self.max_parallax_layer_count,
            max_relief_mapping_search_steps: 0,
        }
    }
}

/// The pipeline key for [`TerrainMaterial`].
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TerrainMaterialKey {
    normal_map: bool,
    cull_mode: Option<Face>,
    depth_bias: i32,
    relief_mapping: bool,
}

impl From<&TerrainMaterial> for TerrainMaterialKey {
    fn from(material: &TerrainMaterial) -> Self {
        TerrainMaterialKey {
            normal_map: material.normal_map_texture.is_some(),
            cull_mode: material.cull_mode,
            depth_bias: material.depth_bias as i32,
            relief_mapping: matches!(
                material.parallax_mapping_method,
                ParallaxMappingMethod::Relief { .. }
            ),
        }
    }
}

impl Material for TerrainMaterial {
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(fragment) = descriptor.fragment.as_mut() {
            let shader_defs = &mut fragment.shader_defs;

            if key.bind_group_data.normal_map {
                shader_defs.push("STANDARDMATERIAL_NORMAL_MAP".into());
            }
            if key.bind_group_data.relief_mapping {
                shader_defs.push("RELIEF_MAPPING".into());
            }
        }
        descriptor.primitive.cull_mode = key.bind_group_data.cull_mode;
        if let Some(label) = &mut descriptor.label {
            *label = format!("pbr_{}", *label).into();
        }
        if let Some(depth_stencil) = descriptor.depth_stencil.as_mut() {
            depth_stencil.bias.constant = key.bind_group_data.depth_bias;
        }
        Ok(())
    }

    fn prepass_fragment_shader() -> ShaderRef {
        PBR_PREPASS_SHADER_HANDLE.typed().into()
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
