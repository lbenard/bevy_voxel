// #import bevy_pbr::mesh_view_bindings
// #import bevy_pbr::pbr_bindings
// #import bevy_pbr::mesh_bindings

// #import bevy_pbr::utils
// #import bevy_pbr::clustered_forward
// #import bevy_pbr::lighting
// #import bevy_pbr::pbr_ambient
// #import bevy_pbr::shadows
// #import bevy_pbr::fog
// #import bevy_pbr::pbr_functions

// fn gold_noise(in: vec3<f32>, seed: f32) -> vec3<f32> {
//     return fract(tan(distance(in * 1.61803398874989484820459, in) * seed) * in.x).xyz;
// }

// struct Vertex {
//     @location(0) position: vec3<f32>,
//     @location(1) normal: vec3<f32>,
// #ifdef VERTEX_UVS
//     @location(2) uv: vec2<f32>,
// #endif
// #ifdef VOXEL_IDS
//     @location(7) voxel_id: u32,
// #endif
// };
struct Vertex {
    @location(0) position: vec3<f32>,

#ifdef VERTEX_UVS
    @location(1) uv: vec2<f32>,
#endif // VERTEX_UVS

#ifdef NORMAL_PREPASS
    @location(2) normal: vec3<f32>,
#ifdef VERTEX_TANGENTS
    @location(3) tangent: vec4<f32>,
#endif // VERTEX_TANGENTS
#endif // NORMAL_PREPASS

#ifdef SKINNED
    @location(4) joint_indices: vec4<u32>,
    @location(5) joint_weights: vec4<f32>,
#endif // SKINNED

#ifdef MORPH_TARGETS
    @builtin(vertex_index) index: u32,
#endif // MORPH_TARGETS
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,

#ifdef VERTEX_UVS
    @location(0) uv: vec2<f32>,
#endif // VERTEX_UVS

#ifdef NORMAL_PREPASS
    @location(1) world_normal: vec3<f32>,
#ifdef VERTEX_TANGENTS
    @location(2) world_tangent: vec4<f32>,
#endif // VERTEX_TANGENTS
#endif // NORMAL_PREPASS

#ifdef MOTION_VECTOR_PREPASS
    @location(3) world_position: vec4<f32>,
    @location(4) previous_world_position: vec4<f32>,
#endif // MOTION_VECTOR_PREPASS

#ifdef DEPTH_CLAMP_ORTHO
    @location(5) clip_position_unclamped: vec4<f32>,
#endif // DEPTH_CLAMP_ORTHO
}

#import bevy_pbr::mesh_functions

@vertex
fn vertex(vertex_no_morph: Vertex) -> VertexOutput {
    var out: VertexOutput;

#ifdef MORPH_TARGETS
    var vertex = morph_vertex(vertex_no_morph);
#else
    var vertex = vertex_no_morph;
#endif

#ifdef SKINNED
    var model = bevy_pbr::skinning::skin_model(vertex.joint_indices, vertex.joint_weights);
#else // SKINNED
    var model = mesh.model;
#endif // SKINNED

    out.clip_position = bevy_pbr::mesh_functions::mesh_position_local_to_clip(model, vec4(vertex.position, 1.0));
#ifdef DEPTH_CLAMP_ORTHO
    out.clip_position_unclamped = out.clip_position;
    out.clip_position.z = min(out.clip_position.z, 1.0);
#endif // DEPTH_CLAMP_ORTHO

#ifdef VERTEX_UVS
    out.uv = vertex.uv;
#endif // VERTEX_UVS

#ifdef NORMAL_PREPASS
#ifdef SKINNED
    out.world_normal = bevy_pbr::skinning::skin_normals(model, vertex.normal);
#else // SKINNED
    out.world_normal = bevy_pbr::mesh_functions::mesh_normal_local_to_world(vertex.normal);
#endif // SKINNED

#ifdef VERTEX_TANGENTS
    out.world_tangent = bevy_pbr::mesh_functions::mesh_tangent_local_to_world(model, vertex.tangent);
#endif // VERTEX_TANGENTS
#endif // NORMAL_PREPASS

#ifdef MOTION_VECTOR_PREPASS
    out.world_position = bevy_pbr::mesh_functions::mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.previous_world_position = bevy_pbr::mesh_functions::mesh_position_local_to_world(mesh.previous_model, vec4<f32>(vertex.position, 1.0));
#endif // MOTION_VECTOR_PREPASS

    return out;
}

// struct FragmentInput {
//     @builtin(front_facing) is_front: bool,
//     @builtin(position) frag_coord: vec4<f32>,
//     #import bevy_pbr::mesh_vertex_output
//     @location(5) voxel_id: u32,
// };

// @fragment
// fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
// #ifdef VOXEL_IDS
//     var output_color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
//     if (in.voxel_id == 0u) {
//         output_color = vec4<f32>(0.0, 0.6, 0.1, 1.0);
//     }
//     if (in.voxel_id == 1u) {
//         output_color = vec4<f32>(0.6, 0.3, 0.1, 1.0);
//     }
//     if (in.voxel_id == 2u) {
//         output_color = vec4<f32>(0.5, 0.5, 0.5, 1.0);
//     }
// #else
//     var output_color: vec4<f32> = material.base_color;
// #endif
// #ifdef VERTEX_COLORS
//     output_color = output_color * in.color;
// #endif
// #ifdef VERTEX_UVS
//     if ((material.flags & STANDARD_MATERIAL_FLAGS_BASE_COLOR_TEXTURE_BIT) != 0u) {
//         output_color = output_color * textureSample(base_color_texture, base_color_sampler, in.uv);
//     }
// #endif

//     // NOTE: Unlit bit not set means == 0 is true, so the true case is if lit
//     if ((material.flags & STANDARD_MATERIAL_FLAGS_UNLIT_BIT) == 0u) {
//         // Prepare a 'processed' StandardMaterial by sampling all textures to resolve
//         // the material members
//         var pbr_input: PbrInput;

//         pbr_input.material.base_color = output_color;
//         pbr_input.material.reflectance = material.reflectance;
//         pbr_input.material.flags = material.flags;
//         pbr_input.material.alpha_cutoff = material.alpha_cutoff;

//         // TODO use .a for exposure compensation in HDR
//         var emissive: vec4<f32> = material.emissive;
// #ifdef VERTEX_UVS
//         if ((material.flags & STANDARD_MATERIAL_FLAGS_EMISSIVE_TEXTURE_BIT) != 0u) {
//             emissive = vec4<f32>(emissive.rgb * textureSample(emissive_texture, emissive_sampler, in.uv).rgb, 1.0);
//         }
// #endif
//         pbr_input.material.emissive = emissive;

//         var metallic: f32 = material.metallic;
//         var perceptual_roughness: f32 = material.perceptual_roughness;
// #ifdef VERTEX_UVS
//         if ((material.flags & STANDARD_MATERIAL_FLAGS_METALLIC_ROUGHNESS_TEXTURE_BIT) != 0u) {
//             let metallic_roughness = textureSample(metallic_roughness_texture, metallic_roughness_sampler, in.uv);
//             // Sampling from GLTF standard channels for now
//             metallic = metallic * metallic_roughness.b;
//             perceptual_roughness = perceptual_roughness * metallic_roughness.g;
//         }
// #endif
//         pbr_input.material.metallic = metallic;
//         pbr_input.material.perceptual_roughness = perceptual_roughness;

//         var occlusion: f32 = 1.0;
// #ifdef VERTEX_UVS
//         if ((material.flags & STANDARD_MATERIAL_FLAGS_OCCLUSION_TEXTURE_BIT) != 0u) {
//             occlusion = textureSample(occlusion_texture, occlusion_sampler, in.uv).r;
//         }
// #endif
//         pbr_input.frag_coord = in.frag_coord;
//         pbr_input.world_position = in.world_position;
//         pbr_input.world_normal = prepare_world_normal(
//             in.world_normal,
//             (material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u,
//             in.is_front,
//         );

//         pbr_input.is_orthographic = view.projection[3].w == 1.0;

//         pbr_input.N = apply_normal_mapping(
//             material.flags,
//             pbr_input.world_normal,
// #ifdef VERTEX_TANGENTS
// #ifdef STANDARDMATERIAL_NORMAL_MAP
//             in.world_tangent,
// #endif
// #endif
// #ifdef VERTEX_UVS
//             in.uv,
// #endif
//         );
//         pbr_input.V = calculate_view(in.world_position, pbr_input.is_orthographic);
//         pbr_input.occlusion = occlusion;

//         pbr_input.flags = mesh.flags;

//         output_color = pbr(pbr_input);
//     } else {
//         output_color = alpha_discard(material, output_color);
//     }

//     // fog
//     if (fog.mode != FOG_MODE_OFF && (material.flags & STANDARD_MATERIAL_FLAGS_FOG_ENABLED_BIT) != 0u) {
//         output_color = apply_fog(output_color, in.world_position.xyz, view.world_position.xyz);
//     }

// #ifdef TONEMAP_IN_SHADER
//     output_color = tone_mapping(output_color);
// #ifdef DEBAND_DITHER
//     var output_rgb = output_color.rgb;
//     output_rgb = powsafe(output_rgb, 1.0 / 2.2);
//     output_rgb = output_rgb + screen_space_dither(in.frag_coord.xy);
//     // This conversion back to linear space is required because our output texture format is
//     // SRGB; the GPU will assume our output is linear and will apply an SRGB conversion.
//     output_rgb = powsafe(output_rgb, 2.2);
//     output_color = vec4(output_rgb, output_color.a);
// #endif
// #endif
// #ifdef PREMULTIPLY_ALPHA
//     output_color = premultiply_alpha(material.flags, output_color);
// #endif
//     return output_color;
// }

#import bevy_pbr::pbr_functions as pbr_functions
#import bevy_pbr::pbr_bindings as pbr_bindings
#import bevy_pbr::pbr_types as pbr_types
#import bevy_pbr::prepass_utils

#import bevy_pbr::mesh_vertex_output       MeshVertexOutput
#import bevy_pbr::mesh_bindings            mesh
#import bevy_pbr::mesh_view_bindings       view, fog, screen_space_ambient_occlusion_texture
#import bevy_pbr::mesh_view_types          FOG_MODE_OFF
#import bevy_core_pipeline::tonemapping    screen_space_dither, powsafe, tone_mapping
#import bevy_pbr::parallax_mapping         parallaxed_uv

#import bevy_pbr::prepass_utils

#ifdef SCREEN_SPACE_AMBIENT_OCCLUSION
#import bevy_pbr::gtao_utils gtao_multibounce
#endif

@fragment
fn fragment(
    in: MeshVertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {
    var output_color: vec4<f32> = pbr_bindings::material.base_color;

    let is_orthographic = view.projection[3].w == 1.0;
    let V = pbr_functions::calculate_view(in.world_position, is_orthographic);
#ifdef VERTEX_UVS
    var uv = in.uv;
#ifdef VERTEX_TANGENTS
    if ((pbr_bindings::material.flags & pbr_types::STANDARD_MATERIAL_FLAGS_DEPTH_MAP_BIT) != 0u) {
        let N = in.world_normal;
        let T = in.world_tangent.xyz;
        let B = in.world_tangent.w * cross(N, T);
        // Transform V from fragment to camera in world space to tangent space.
        let Vt = vec3(dot(V, T), dot(V, B), dot(V, N));
        uv = parallaxed_uv(
            pbr_bindings::material.parallax_depth_scale,
            pbr_bindings::material.max_parallax_layer_count,
            pbr_bindings::material.max_relief_mapping_search_steps,
            uv,
            // Flip the direction of Vt to go toward the surface to make the
            // parallax mapping algorithm easier to understand and reason
            // about.
            -Vt,
        );
    }
#endif
#endif

#ifdef VERTEX_COLORS
    output_color = output_color * in.color;
#endif
#ifdef VERTEX_UVS
    if ((pbr_bindings::material.flags & pbr_types::STANDARD_MATERIAL_FLAGS_BASE_COLOR_TEXTURE_BIT) != 0u) {
        output_color = output_color * textureSampleBias(pbr_bindings::base_color_texture, pbr_bindings::base_color_sampler, uv, view.mip_bias);
    }
#endif

    // NOTE: Unlit bit not set means == 0 is true, so the true case is if lit
    if ((pbr_bindings::material.flags & pbr_types::STANDARD_MATERIAL_FLAGS_UNLIT_BIT) == 0u) {
        // Prepare a 'processed' StandardMaterial by sampling all textures to resolve
        // the material members
        var pbr_input: pbr_functions::PbrInput;

        pbr_input.material.base_color = output_color;
        pbr_input.material.reflectance = pbr_bindings::material.reflectance;
        pbr_input.material.flags = pbr_bindings::material.flags;
        pbr_input.material.alpha_cutoff = pbr_bindings::material.alpha_cutoff;

        // TODO use .a for exposure compensation in HDR
        var emissive: vec4<f32> = pbr_bindings::material.emissive;
#ifdef VERTEX_UVS
        if ((pbr_bindings::material.flags & pbr_types::STANDARD_MATERIAL_FLAGS_EMISSIVE_TEXTURE_BIT) != 0u) {
            emissive = vec4<f32>(emissive.rgb * textureSampleBias(pbr_bindings::emissive_texture, pbr_bindings::emissive_sampler, uv, view.mip_bias).rgb, 1.0);
        }
#endif
        pbr_input.material.emissive = emissive;

        var metallic: f32 = pbr_bindings::material.metallic;
        var perceptual_roughness: f32 = pbr_bindings::material.perceptual_roughness;
#ifdef VERTEX_UVS
        if ((pbr_bindings::material.flags & pbr_types::STANDARD_MATERIAL_FLAGS_METALLIC_ROUGHNESS_TEXTURE_BIT) != 0u) {
            let metallic_roughness = textureSampleBias(pbr_bindings::metallic_roughness_texture, pbr_bindings::metallic_roughness_sampler, uv, view.mip_bias);
            // Sampling from GLTF standard channels for now
            metallic = metallic * metallic_roughness.b;
            perceptual_roughness = perceptual_roughness * metallic_roughness.g;
        }
#endif
        pbr_input.material.metallic = metallic;
        pbr_input.material.perceptual_roughness = perceptual_roughness;

        // TODO: Split into diffuse/specular occlusion?
        var occlusion: vec3<f32> = vec3(1.0);
#ifdef VERTEX_UVS
        if ((pbr_bindings::material.flags & pbr_types::STANDARD_MATERIAL_FLAGS_OCCLUSION_TEXTURE_BIT) != 0u) {
            occlusion = vec3(textureSampleBias(pbr_bindings::occlusion_texture, pbr_bindings::occlusion_sampler, uv, view.mip_bias).r);
        }
#endif
#ifdef SCREEN_SPACE_AMBIENT_OCCLUSION
        let ssao = pow(textureLoad(screen_space_ambient_occlusion_texture, vec2<i32>(in.position.xy), 0i).r, 2.0);
        let ssao_multibounce = gtao_multibounce(ssao, pbr_input.material.base_color.rgb);
        occlusion = min(occlusion, ssao_multibounce);
#endif
        pbr_input.occlusion = occlusion;

        pbr_input.frag_coord = in.position;
        pbr_input.world_position = in.world_position;

        pbr_input.world_normal = pbr_functions::prepare_world_normal(
            in.world_normal,
            (pbr_bindings::material.flags & pbr_types::STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u,
            is_front,
        );

        pbr_input.is_orthographic = is_orthographic;

#ifdef LOAD_PREPASS_NORMALS
        pbr_input.N = bevy_pbr::prepass_utils::prepass_normal(in.position, 0u);
#else
        pbr_input.N = pbr_functions::apply_normal_mapping(
            pbr_bindings::material.flags,
            pbr_input.world_normal,
#ifdef VERTEX_TANGENTS
#ifdef STANDARDMATERIAL_NORMAL_MAP
            in.world_tangent,
#endif
#endif
#ifdef VERTEX_UVS
            uv,
#endif
            view.mip_bias,
        );
#endif

        pbr_input.V = V;
        pbr_input.occlusion = occlusion;

        pbr_input.flags = mesh.flags;

        output_color = pbr_functions::pbr(pbr_input);
    } else {
        output_color = pbr_functions::alpha_discard(pbr_bindings::material, output_color);
    }

    // fog
    if (fog.mode != FOG_MODE_OFF && (pbr_bindings::material.flags & pbr_types::STANDARD_MATERIAL_FLAGS_FOG_ENABLED_BIT) != 0u) {
        output_color = pbr_functions::apply_fog(fog, output_color, in.world_position.xyz, view.world_position.xyz);
    }

#ifdef TONEMAP_IN_SHADER
    output_color = tone_mapping(output_color, view.color_grading);
#ifdef DEBAND_DITHER
    var output_rgb = output_color.rgb;
    output_rgb = powsafe(output_rgb, 1.0 / 2.2);
    output_rgb = output_rgb + screen_space_dither(in.position.xy);
    // This conversion back to linear space is required because our output texture format is
    // SRGB; the GPU will assume our output is linear and will apply an SRGB conversion.
    output_rgb = powsafe(output_rgb, 2.2);
    output_color = vec4(output_rgb, output_color.a);
#endif
#endif
#ifdef PREMULTIPLY_ALPHA
    output_color = pbr_functions::premultiply_alpha(pbr_bindings::material.flags, output_color);
#endif
    return output_color;
}
