fn main() {
    bindgen::builder()
        .header("vendor/cgltf/cgltf.h")
        .header("vendor/cgltf/cgltf_write.h")
        .whitelist_type("cgltf.*")
        .whitelist_function("cgltf.*")
        .whitelist_var("cgltf.*")
        .layout_tests(false)
        .generate()
        .expect("Could not generate bindings")
        .write_to_file("src/ffi.rs")
        .expect("Could not write bindings to file");

    cc::Build::new()
        .file("cgltf.c")
        .file("cgltf_write.c")
        .compile("cgltf");
}
