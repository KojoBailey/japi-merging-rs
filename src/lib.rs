japi::register_mod! {
    title: "Merging",
    author: "Kojo Bailey",
    guid: "merging",
    version: "0.1.0",
    desc: "Allows for easy merging of otherwise incompatible mods."
}

#[unsafe(no_mangle)]
pub extern "C" fn ModInit() {
    japi::log_info!("Loaded!");
}
