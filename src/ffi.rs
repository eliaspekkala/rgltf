/* automatically generated by rust-bindgen 0.55.1 */

pub type size_t = ::std::os::raw::c_ulong;
pub type cgltf_size = size_t;
pub type cgltf_float = f32;
pub type cgltf_int = ::std::os::raw::c_int;
pub type cgltf_uint = ::std::os::raw::c_uint;
pub type cgltf_bool = ::std::os::raw::c_int;
pub const cgltf_file_type_cgltf_file_type_invalid: cgltf_file_type = 0;
pub const cgltf_file_type_cgltf_file_type_gltf: cgltf_file_type = 1;
pub const cgltf_file_type_cgltf_file_type_glb: cgltf_file_type = 2;
pub type cgltf_file_type = ::std::os::raw::c_uint;
pub const cgltf_result_cgltf_result_success: cgltf_result = 0;
pub const cgltf_result_cgltf_result_data_too_short: cgltf_result = 1;
pub const cgltf_result_cgltf_result_unknown_format: cgltf_result = 2;
pub const cgltf_result_cgltf_result_invalid_json: cgltf_result = 3;
pub const cgltf_result_cgltf_result_invalid_gltf: cgltf_result = 4;
pub const cgltf_result_cgltf_result_invalid_options: cgltf_result = 5;
pub const cgltf_result_cgltf_result_file_not_found: cgltf_result = 6;
pub const cgltf_result_cgltf_result_io_error: cgltf_result = 7;
pub const cgltf_result_cgltf_result_out_of_memory: cgltf_result = 8;
pub const cgltf_result_cgltf_result_legacy_gltf: cgltf_result = 9;
pub type cgltf_result = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_memory_options {
    pub alloc: ::std::option::Option<
        unsafe extern "C" fn(
            user: *mut ::std::os::raw::c_void,
            size: cgltf_size,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub free: ::std::option::Option<
        unsafe extern "C" fn(user: *mut ::std::os::raw::c_void, ptr: *mut ::std::os::raw::c_void),
    >,
    pub user_data: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_file_options {
    pub read: ::std::option::Option<
        unsafe extern "C" fn(
            memory_options: *const cgltf_memory_options,
            file_options: *const cgltf_file_options,
            path: *const ::std::os::raw::c_char,
            size: *mut cgltf_size,
            data: *mut *mut ::std::os::raw::c_void,
        ) -> cgltf_result,
    >,
    pub release: ::std::option::Option<
        unsafe extern "C" fn(
            memory_options: *const cgltf_memory_options,
            file_options: *const cgltf_file_options,
            data: *mut ::std::os::raw::c_void,
        ),
    >,
    pub user_data: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_options {
    pub type_: cgltf_file_type,
    pub json_token_count: cgltf_size,
    pub memory: cgltf_memory_options,
    pub file: cgltf_file_options,
}
pub const cgltf_buffer_view_type_cgltf_buffer_view_type_invalid: cgltf_buffer_view_type = 0;
pub const cgltf_buffer_view_type_cgltf_buffer_view_type_indices: cgltf_buffer_view_type = 1;
pub const cgltf_buffer_view_type_cgltf_buffer_view_type_vertices: cgltf_buffer_view_type = 2;
pub type cgltf_buffer_view_type = ::std::os::raw::c_uint;
pub const cgltf_attribute_type_cgltf_attribute_type_invalid: cgltf_attribute_type = 0;
pub const cgltf_attribute_type_cgltf_attribute_type_position: cgltf_attribute_type = 1;
pub const cgltf_attribute_type_cgltf_attribute_type_normal: cgltf_attribute_type = 2;
pub const cgltf_attribute_type_cgltf_attribute_type_tangent: cgltf_attribute_type = 3;
pub const cgltf_attribute_type_cgltf_attribute_type_texcoord: cgltf_attribute_type = 4;
pub const cgltf_attribute_type_cgltf_attribute_type_color: cgltf_attribute_type = 5;
pub const cgltf_attribute_type_cgltf_attribute_type_joints: cgltf_attribute_type = 6;
pub const cgltf_attribute_type_cgltf_attribute_type_weights: cgltf_attribute_type = 7;
pub type cgltf_attribute_type = ::std::os::raw::c_uint;
pub const cgltf_component_type_cgltf_component_type_invalid: cgltf_component_type = 0;
pub const cgltf_component_type_cgltf_component_type_r_8: cgltf_component_type = 1;
pub const cgltf_component_type_cgltf_component_type_r_8u: cgltf_component_type = 2;
pub const cgltf_component_type_cgltf_component_type_r_16: cgltf_component_type = 3;
pub const cgltf_component_type_cgltf_component_type_r_16u: cgltf_component_type = 4;
pub const cgltf_component_type_cgltf_component_type_r_32u: cgltf_component_type = 5;
pub const cgltf_component_type_cgltf_component_type_r_32f: cgltf_component_type = 6;
pub type cgltf_component_type = ::std::os::raw::c_uint;
pub const cgltf_type_cgltf_type_invalid: cgltf_type = 0;
pub const cgltf_type_cgltf_type_scalar: cgltf_type = 1;
pub const cgltf_type_cgltf_type_vec2: cgltf_type = 2;
pub const cgltf_type_cgltf_type_vec3: cgltf_type = 3;
pub const cgltf_type_cgltf_type_vec4: cgltf_type = 4;
pub const cgltf_type_cgltf_type_mat2: cgltf_type = 5;
pub const cgltf_type_cgltf_type_mat3: cgltf_type = 6;
pub const cgltf_type_cgltf_type_mat4: cgltf_type = 7;
pub type cgltf_type = ::std::os::raw::c_uint;
pub const cgltf_primitive_type_cgltf_primitive_type_points: cgltf_primitive_type = 0;
pub const cgltf_primitive_type_cgltf_primitive_type_lines: cgltf_primitive_type = 1;
pub const cgltf_primitive_type_cgltf_primitive_type_line_loop: cgltf_primitive_type = 2;
pub const cgltf_primitive_type_cgltf_primitive_type_line_strip: cgltf_primitive_type = 3;
pub const cgltf_primitive_type_cgltf_primitive_type_triangles: cgltf_primitive_type = 4;
pub const cgltf_primitive_type_cgltf_primitive_type_triangle_strip: cgltf_primitive_type = 5;
pub const cgltf_primitive_type_cgltf_primitive_type_triangle_fan: cgltf_primitive_type = 6;
pub type cgltf_primitive_type = ::std::os::raw::c_uint;
pub const cgltf_alpha_mode_cgltf_alpha_mode_opaque: cgltf_alpha_mode = 0;
pub const cgltf_alpha_mode_cgltf_alpha_mode_mask: cgltf_alpha_mode = 1;
pub const cgltf_alpha_mode_cgltf_alpha_mode_blend: cgltf_alpha_mode = 2;
pub type cgltf_alpha_mode = ::std::os::raw::c_uint;
pub const cgltf_animation_path_type_cgltf_animation_path_type_invalid: cgltf_animation_path_type =
    0;
pub const cgltf_animation_path_type_cgltf_animation_path_type_translation:
    cgltf_animation_path_type = 1;
pub const cgltf_animation_path_type_cgltf_animation_path_type_rotation: cgltf_animation_path_type =
    2;
pub const cgltf_animation_path_type_cgltf_animation_path_type_scale: cgltf_animation_path_type = 3;
pub const cgltf_animation_path_type_cgltf_animation_path_type_weights: cgltf_animation_path_type =
    4;
pub type cgltf_animation_path_type = ::std::os::raw::c_uint;
pub const cgltf_interpolation_type_cgltf_interpolation_type_linear: cgltf_interpolation_type = 0;
pub const cgltf_interpolation_type_cgltf_interpolation_type_step: cgltf_interpolation_type = 1;
pub const cgltf_interpolation_type_cgltf_interpolation_type_cubic_spline: cgltf_interpolation_type =
    2;
pub type cgltf_interpolation_type = ::std::os::raw::c_uint;
pub const cgltf_camera_type_cgltf_camera_type_invalid: cgltf_camera_type = 0;
pub const cgltf_camera_type_cgltf_camera_type_perspective: cgltf_camera_type = 1;
pub const cgltf_camera_type_cgltf_camera_type_orthographic: cgltf_camera_type = 2;
pub type cgltf_camera_type = ::std::os::raw::c_uint;
pub const cgltf_light_type_cgltf_light_type_invalid: cgltf_light_type = 0;
pub const cgltf_light_type_cgltf_light_type_directional: cgltf_light_type = 1;
pub const cgltf_light_type_cgltf_light_type_point: cgltf_light_type = 2;
pub const cgltf_light_type_cgltf_light_type_spot: cgltf_light_type = 3;
pub type cgltf_light_type = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_extras {
    pub start_offset: cgltf_size,
    pub end_offset: cgltf_size,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_extension {
    pub name: *mut ::std::os::raw::c_char,
    pub data: *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_buffer {
    pub size: cgltf_size,
    pub uri: *mut ::std::os::raw::c_char,
    pub data: *mut ::std::os::raw::c_void,
    pub extras: cgltf_extras,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_buffer_view {
    pub buffer: *mut cgltf_buffer,
    pub offset: cgltf_size,
    pub size: cgltf_size,
    pub stride: cgltf_size,
    pub type_: cgltf_buffer_view_type,
    pub extras: cgltf_extras,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_accessor_sparse {
    pub count: cgltf_size,
    pub indices_buffer_view: *mut cgltf_buffer_view,
    pub indices_byte_offset: cgltf_size,
    pub indices_component_type: cgltf_component_type,
    pub values_buffer_view: *mut cgltf_buffer_view,
    pub values_byte_offset: cgltf_size,
    pub extras: cgltf_extras,
    pub indices_extras: cgltf_extras,
    pub values_extras: cgltf_extras,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
    pub indices_extensions_count: cgltf_size,
    pub indices_extensions: *mut cgltf_extension,
    pub values_extensions_count: cgltf_size,
    pub values_extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_accessor {
    pub component_type: cgltf_component_type,
    pub normalized: cgltf_bool,
    pub type_: cgltf_type,
    pub offset: cgltf_size,
    pub count: cgltf_size,
    pub stride: cgltf_size,
    pub buffer_view: *mut cgltf_buffer_view,
    pub has_min: cgltf_bool,
    pub min: [cgltf_float; 16usize],
    pub has_max: cgltf_bool,
    pub max: [cgltf_float; 16usize],
    pub is_sparse: cgltf_bool,
    pub sparse: cgltf_accessor_sparse,
    pub extras: cgltf_extras,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_attribute {
    pub name: *mut ::std::os::raw::c_char,
    pub type_: cgltf_attribute_type,
    pub index: cgltf_int,
    pub data: *mut cgltf_accessor,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_image {
    pub name: *mut ::std::os::raw::c_char,
    pub uri: *mut ::std::os::raw::c_char,
    pub buffer_view: *mut cgltf_buffer_view,
    pub mime_type: *mut ::std::os::raw::c_char,
    pub extras: cgltf_extras,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_sampler {
    pub mag_filter: cgltf_int,
    pub min_filter: cgltf_int,
    pub wrap_s: cgltf_int,
    pub wrap_t: cgltf_int,
    pub extras: cgltf_extras,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_texture {
    pub name: *mut ::std::os::raw::c_char,
    pub image: *mut cgltf_image,
    pub sampler: *mut cgltf_sampler,
    pub extras: cgltf_extras,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_texture_transform {
    pub offset: [cgltf_float; 2usize],
    pub rotation: cgltf_float,
    pub scale: [cgltf_float; 2usize],
    pub texcoord: cgltf_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_texture_view {
    pub texture: *mut cgltf_texture,
    pub texcoord: cgltf_int,
    pub scale: cgltf_float,
    pub has_transform: cgltf_bool,
    pub transform: cgltf_texture_transform,
    pub extras: cgltf_extras,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_pbr_metallic_roughness {
    pub base_color_texture: cgltf_texture_view,
    pub metallic_roughness_texture: cgltf_texture_view,
    pub base_color_factor: [cgltf_float; 4usize],
    pub metallic_factor: cgltf_float,
    pub roughness_factor: cgltf_float,
    pub extras: cgltf_extras,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_pbr_specular_glossiness {
    pub diffuse_texture: cgltf_texture_view,
    pub specular_glossiness_texture: cgltf_texture_view,
    pub diffuse_factor: [cgltf_float; 4usize],
    pub specular_factor: [cgltf_float; 3usize],
    pub glossiness_factor: cgltf_float,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_clearcoat {
    pub clearcoat_texture: cgltf_texture_view,
    pub clearcoat_roughness_texture: cgltf_texture_view,
    pub clearcoat_normal_texture: cgltf_texture_view,
    pub clearcoat_factor: cgltf_float,
    pub clearcoat_roughness_factor: cgltf_float,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_transmission {
    pub transmission_texture: cgltf_texture_view,
    pub transmission_factor: cgltf_float,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_ior {
    pub ior: cgltf_float,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_specular {
    pub specular_texture: cgltf_texture_view,
    pub specular_color_factor: [cgltf_float; 3usize],
    pub specular_factor: cgltf_float,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_material {
    pub name: *mut ::std::os::raw::c_char,
    pub has_pbr_metallic_roughness: cgltf_bool,
    pub has_pbr_specular_glossiness: cgltf_bool,
    pub has_clearcoat: cgltf_bool,
    pub has_transmission: cgltf_bool,
    pub has_ior: cgltf_bool,
    pub has_specular: cgltf_bool,
    pub pbr_metallic_roughness: cgltf_pbr_metallic_roughness,
    pub pbr_specular_glossiness: cgltf_pbr_specular_glossiness,
    pub clearcoat: cgltf_clearcoat,
    pub ior: cgltf_ior,
    pub specular: cgltf_specular,
    pub transmission: cgltf_transmission,
    pub normal_texture: cgltf_texture_view,
    pub occlusion_texture: cgltf_texture_view,
    pub emissive_texture: cgltf_texture_view,
    pub emissive_factor: [cgltf_float; 3usize],
    pub alpha_mode: cgltf_alpha_mode,
    pub alpha_cutoff: cgltf_float,
    pub double_sided: cgltf_bool,
    pub unlit: cgltf_bool,
    pub extras: cgltf_extras,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_morph_target {
    pub attributes: *mut cgltf_attribute,
    pub attributes_count: cgltf_size,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_draco_mesh_compression {
    pub buffer_view: *mut cgltf_buffer_view,
    pub attributes: *mut cgltf_attribute,
    pub attributes_count: cgltf_size,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_primitive {
    pub type_: cgltf_primitive_type,
    pub indices: *mut cgltf_accessor,
    pub material: *mut cgltf_material,
    pub attributes: *mut cgltf_attribute,
    pub attributes_count: cgltf_size,
    pub targets: *mut cgltf_morph_target,
    pub targets_count: cgltf_size,
    pub extras: cgltf_extras,
    pub has_draco_mesh_compression: cgltf_bool,
    pub draco_mesh_compression: cgltf_draco_mesh_compression,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_mesh {
    pub name: *mut ::std::os::raw::c_char,
    pub primitives: *mut cgltf_primitive,
    pub primitives_count: cgltf_size,
    pub weights: *mut cgltf_float,
    pub weights_count: cgltf_size,
    pub target_names: *mut *mut ::std::os::raw::c_char,
    pub target_names_count: cgltf_size,
    pub extras: cgltf_extras,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_skin {
    pub name: *mut ::std::os::raw::c_char,
    pub joints: *mut *mut cgltf_node,
    pub joints_count: cgltf_size,
    pub skeleton: *mut cgltf_node,
    pub inverse_bind_matrices: *mut cgltf_accessor,
    pub extras: cgltf_extras,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_camera_perspective {
    pub aspect_ratio: cgltf_float,
    pub yfov: cgltf_float,
    pub zfar: cgltf_float,
    pub znear: cgltf_float,
    pub extras: cgltf_extras,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_camera_orthographic {
    pub xmag: cgltf_float,
    pub ymag: cgltf_float,
    pub zfar: cgltf_float,
    pub znear: cgltf_float,
    pub extras: cgltf_extras,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgltf_camera {
    pub name: *mut ::std::os::raw::c_char,
    pub type_: cgltf_camera_type,
    pub data: cgltf_camera__bindgen_ty_1,
    pub extras: cgltf_extras,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union cgltf_camera__bindgen_ty_1 {
    pub perspective: cgltf_camera_perspective,
    pub orthographic: cgltf_camera_orthographic,
    _bindgen_union_align: [u64; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_light {
    pub name: *mut ::std::os::raw::c_char,
    pub color: [cgltf_float; 3usize],
    pub intensity: cgltf_float,
    pub type_: cgltf_light_type,
    pub range: cgltf_float,
    pub spot_inner_cone_angle: cgltf_float,
    pub spot_outer_cone_angle: cgltf_float,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_node {
    pub name: *mut ::std::os::raw::c_char,
    pub parent: *mut cgltf_node,
    pub children: *mut *mut cgltf_node,
    pub children_count: cgltf_size,
    pub skin: *mut cgltf_skin,
    pub mesh: *mut cgltf_mesh,
    pub camera: *mut cgltf_camera,
    pub light: *mut cgltf_light,
    pub weights: *mut cgltf_float,
    pub weights_count: cgltf_size,
    pub has_translation: cgltf_bool,
    pub has_rotation: cgltf_bool,
    pub has_scale: cgltf_bool,
    pub has_matrix: cgltf_bool,
    pub translation: [cgltf_float; 3usize],
    pub rotation: [cgltf_float; 4usize],
    pub scale: [cgltf_float; 3usize],
    pub matrix: [cgltf_float; 16usize],
    pub extras: cgltf_extras,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_scene {
    pub name: *mut ::std::os::raw::c_char,
    pub nodes: *mut *mut cgltf_node,
    pub nodes_count: cgltf_size,
    pub extras: cgltf_extras,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_animation_sampler {
    pub input: *mut cgltf_accessor,
    pub output: *mut cgltf_accessor,
    pub interpolation: cgltf_interpolation_type,
    pub extras: cgltf_extras,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_animation_channel {
    pub sampler: *mut cgltf_animation_sampler,
    pub target_node: *mut cgltf_node,
    pub target_path: cgltf_animation_path_type,
    pub extras: cgltf_extras,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_animation {
    pub name: *mut ::std::os::raw::c_char,
    pub samplers: *mut cgltf_animation_sampler,
    pub samplers_count: cgltf_size,
    pub channels: *mut cgltf_animation_channel,
    pub channels_count: cgltf_size,
    pub extras: cgltf_extras,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_asset {
    pub copyright: *mut ::std::os::raw::c_char,
    pub generator: *mut ::std::os::raw::c_char,
    pub version: *mut ::std::os::raw::c_char,
    pub min_version: *mut ::std::os::raw::c_char,
    pub extras: cgltf_extras,
    pub extensions_count: cgltf_size,
    pub extensions: *mut cgltf_extension,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgltf_data {
    pub file_type: cgltf_file_type,
    pub file_data: *mut ::std::os::raw::c_void,
    pub asset: cgltf_asset,
    pub meshes: *mut cgltf_mesh,
    pub meshes_count: cgltf_size,
    pub materials: *mut cgltf_material,
    pub materials_count: cgltf_size,
    pub accessors: *mut cgltf_accessor,
    pub accessors_count: cgltf_size,
    pub buffer_views: *mut cgltf_buffer_view,
    pub buffer_views_count: cgltf_size,
    pub buffers: *mut cgltf_buffer,
    pub buffers_count: cgltf_size,
    pub images: *mut cgltf_image,
    pub images_count: cgltf_size,
    pub textures: *mut cgltf_texture,
    pub textures_count: cgltf_size,
    pub samplers: *mut cgltf_sampler,
    pub samplers_count: cgltf_size,
    pub skins: *mut cgltf_skin,
    pub skins_count: cgltf_size,
    pub cameras: *mut cgltf_camera,
    pub cameras_count: cgltf_size,
    pub lights: *mut cgltf_light,
    pub lights_count: cgltf_size,
    pub nodes: *mut cgltf_node,
    pub nodes_count: cgltf_size,
    pub scenes: *mut cgltf_scene,
    pub scenes_count: cgltf_size,
    pub scene: *mut cgltf_scene,
    pub animations: *mut cgltf_animation,
    pub animations_count: cgltf_size,
    pub extras: cgltf_extras,
    pub data_extensions_count: cgltf_size,
    pub data_extensions: *mut cgltf_extension,
    pub extensions_used: *mut *mut ::std::os::raw::c_char,
    pub extensions_used_count: cgltf_size,
    pub extensions_required: *mut *mut ::std::os::raw::c_char,
    pub extensions_required_count: cgltf_size,
    pub json: *const ::std::os::raw::c_char,
    pub json_size: cgltf_size,
    pub bin: *const ::std::os::raw::c_void,
    pub bin_size: cgltf_size,
    pub memory: cgltf_memory_options,
    pub file: cgltf_file_options,
}
extern "C" {
    pub fn cgltf_parse(
        options: *const cgltf_options,
        data: *const ::std::os::raw::c_void,
        size: cgltf_size,
        out_data: *mut *mut cgltf_data,
    ) -> cgltf_result;
}
extern "C" {
    pub fn cgltf_parse_file(
        options: *const cgltf_options,
        path: *const ::std::os::raw::c_char,
        out_data: *mut *mut cgltf_data,
    ) -> cgltf_result;
}
extern "C" {
    pub fn cgltf_load_buffers(
        options: *const cgltf_options,
        data: *mut cgltf_data,
        gltf_path: *const ::std::os::raw::c_char,
    ) -> cgltf_result;
}
extern "C" {
    pub fn cgltf_load_buffer_base64(
        options: *const cgltf_options,
        size: cgltf_size,
        base64: *const ::std::os::raw::c_char,
        out_data: *mut *mut ::std::os::raw::c_void,
    ) -> cgltf_result;
}
extern "C" {
    pub fn cgltf_decode_uri(uri: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn cgltf_validate(data: *mut cgltf_data) -> cgltf_result;
}
extern "C" {
    pub fn cgltf_free(data: *mut cgltf_data);
}
extern "C" {
    pub fn cgltf_node_transform_local(node: *const cgltf_node, out_matrix: *mut cgltf_float);
}
extern "C" {
    pub fn cgltf_node_transform_world(node: *const cgltf_node, out_matrix: *mut cgltf_float);
}
extern "C" {
    pub fn cgltf_accessor_read_float(
        accessor: *const cgltf_accessor,
        index: cgltf_size,
        out: *mut cgltf_float,
        element_size: cgltf_size,
    ) -> cgltf_bool;
}
extern "C" {
    pub fn cgltf_accessor_read_uint(
        accessor: *const cgltf_accessor,
        index: cgltf_size,
        out: *mut cgltf_uint,
        element_size: cgltf_size,
    ) -> cgltf_bool;
}
extern "C" {
    pub fn cgltf_accessor_read_index(
        accessor: *const cgltf_accessor,
        index: cgltf_size,
    ) -> cgltf_size;
}
extern "C" {
    pub fn cgltf_num_components(type_: cgltf_type) -> cgltf_size;
}
extern "C" {
    pub fn cgltf_accessor_unpack_floats(
        accessor: *const cgltf_accessor,
        out: *mut cgltf_float,
        float_count: cgltf_size,
    ) -> cgltf_size;
}
extern "C" {
    pub fn cgltf_copy_extras_json(
        data: *const cgltf_data,
        extras: *const cgltf_extras,
        dest: *mut ::std::os::raw::c_char,
        dest_size: *mut cgltf_size,
    ) -> cgltf_result;
}
extern "C" {
    pub fn cgltf_write_file(
        options: *const cgltf_options,
        path: *const ::std::os::raw::c_char,
        data: *const cgltf_data,
    ) -> cgltf_result;
}
extern "C" {
    pub fn cgltf_write(
        options: *const cgltf_options,
        buffer: *mut ::std::os::raw::c_char,
        size: cgltf_size,
        data: *const cgltf_data,
    ) -> cgltf_size;
}
