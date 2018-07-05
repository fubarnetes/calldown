extern crate bindgen;
use std::env;

fn main() {
    let out_file = env::current_dir()
        .unwrap()
        .join("src")
        .join("zfs")
        .join("libzfs_core_sys.rs");

    let bindings = bindgen::Builder::default()
        .header("src/zfs/libzfs.h")
        .whitelist_function("lzc_core_init")
        .whitelist_function("lzc_core_fini")
        .whitelist_function("lzc_remap")
        .whitelist_function("lzc_snapshot")
        .whitelist_function("lzc_create")
        .whitelist_function("lzc_clone")
        .whitelist_function("lzc_promote")
        .whitelist_function("lzc_destroy_snaps")
        .whitelist_function("lzc_bookmark")
        .whitelist_function("lzc_get_bookmarks")
        .whitelist_function("lzc_destroy_bookmarks")
        .whitelist_function("lzc_snaprange_space")
        .whitelist_function("lzc_hold")
        .whitelist_function("lzc_release")
        .whitelist_function("lzc_get_holds")
        .whitelist_function("lzc_send")
        .whitelist_function("lzc_send_resume")
        .whitelist_function("lzc_send_space")
        .whitelist_function("lzc_receive")
        .whitelist_function("lzc_receive_resumable")
        .whitelist_function("lzc_receive_with_header")
        .whitelist_function("lzc_exists")
        .whitelist_function("lzc_rollback")
        .whitelist_function("lzc_rollback_to")
        .whitelist_function("lzc_channel_program")
        .whitelist_function("lzc_channel_program_nosync")
        .whitelist_function("lzc_checkpoint")
        .whitelist_function("lzc_checkpoint_discard")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}

