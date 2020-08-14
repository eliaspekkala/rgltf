use rgltf::ffi;

fn main() {
    unsafe {
        let path = std::ffi::CString::new("./cube.glb").unwrap();
        let path: *const std::os::raw::c_char = path.as_ptr();

        let options: *const ffi::cgltf_options =
            std::mem::MaybeUninit::<ffi::cgltf_options>::zeroed().as_ptr();

        let mut out_data: *mut ffi::cgltf_data = std::ptr::null_mut();

        let result: ffi::cgltf_result = ffi::cgltf_parse_file(options, path, &mut out_data);

        if result == ffi::cgltf_result_cgltf_result_success {
            println!("SUCCESS: {:#?}", *out_data);

            ffi::cgltf_free(out_data);
        } else {
            println!("FAIL: {}", result);
        }
    }
}
