japi::register_mod! {
    title: "Merging",
    author: "Kojo Bailey",
    guid: "merging",
    version: "0.1.0",
    desc: "Allows for easy merging of otherwise incompatible mods."
}

static mut SUB_56C970_ORIGINAL: extern "fastcall" fn() = sub_56c970_hook;

extern "fastcall" fn sub_56c970_hook() {
    japi::log_debug!("Hooked the CPK loader.");
    
    unsafe { SUB_56C970_ORIGINAL(); }
}

#[unsafe(no_mangle)]
pub extern "stdcall" fn ModInit() {
    japi::log_debug!("Attempting to hook...");

    let result = japi::register_hook!(
        0x56C970,
        sub_56c970_hook,
        SUB_56C970_ORIGINAL,
        "ASBR_LoadPatchCPKs",
        true
    );

    let Some(_) = result else {
        japi::log_fatal!("Failed to hook!");
        return;
    };

    japi::log_info!("Loaded!");
}
