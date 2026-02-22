mod load_order;
mod player_color_param_hook;

use player_color_param_hook::{parse_playercolorparam_hook, NuccMemVector, NuccMemPlayerColorParam};

use std::ffi::c_char;

japi::register_mod! {
    title: "Merging",
    author: "Kojo Bailey",
    guid: "merging",
    version: "0.1.0",
    desc: "Allows for easy merging of otherwise incompatible mods."
}

pub static MERGING_ROOT_PATH: &str = "japi/merging/";

pub static mut NUCC_HASH: Option<unsafe extern "C" fn(*const c_char) -> u32> = None;
pub static mut RGBA_INT_TO_FLOAT: Option<unsafe extern "C" fn(*const f32, u32) -> *const f32> = None;
pub static mut ALLOCATE_PLAYERCOLORPARAM_DATA: Option<unsafe extern "C" fn(
    *const NuccMemVector, *const NuccMemPlayerColorParam, *const NuccMemPlayerColorParam) -> *const u64> = None;

// pub static mut GET_GAME_LANGUAGE_ORIGINAL: extern "C" fn(*const u64, *const i32) -> *const u64 = get_game_language_hook;

pub static mut PARSE_PLAYERCOLORPARAM_ORIGINAL: extern "C" fn(*mut u64) -> *const u64 = parse_playercolorparam_hook;

pub static mut LOAD_NUCCBINARY_ORIGINAL: extern "C" fn(*const c_char, *const c_char) -> *const u64 = load_nuccbinary_hook;

extern "C" fn load_nuccbinary_hook(xfbin_path: *const c_char, chunk_name_buffer: *const c_char) -> *const u64 {
    unsafe { LOAD_NUCCBINARY_ORIGINAL(xfbin_path, chunk_name_buffer) }
}

#[unsafe(no_mangle)]
pub extern "C" fn ModInit() {
    unsafe {
        NUCC_HASH = Some(std::mem::transmute(japi::offset_to_module_address(0x6C92A0)));
        RGBA_INT_TO_FLOAT = Some(std::mem::transmute(japi::offset_to_module_address(0x6DC840)));
        ALLOCATE_PLAYERCOLORPARAM_DATA = Some(std::mem::transmute(japi::offset_to_module_address(0x47EB58)));
    }

    // let Some(_) = japi::register_hook!(
    //     0x6F1970,
    //     get_game_language_hook,
    //     GET_GAME_LANGUAGE_ORIGINAL,
    //     "get_game_language",
    //     true
    // ) else {
    //     japi::log_fatal!("Failed to hook!");
    //     return;
    // };

    let Some(_) = japi::register_hook!(
        0x671C30,
        load_nuccbinary_hook,
        LOAD_NUCCBINARY_ORIGINAL,
        "load_nuccbinary",
        true
    ) else {
        japi::log_fatal!("Failed to hook!");
        return;
    };

    let Some(_) = japi::register_hook!(
        0x47F114,
        parse_playercolorparam_hook,
        PARSE_PLAYERCOLORPARAM_ORIGINAL,
        "parse_playercolorparam",
        true
    ) else {
        japi::log_fatal!("Failed to hook!");
        return;
    };

    japi::log_info!("Loaded!");
}
