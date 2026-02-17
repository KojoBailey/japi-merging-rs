japi::register_mod! {
    title: "Merging",
    author: "Kojo Bailey",
    guid: "merging",
    version: "0.1.0",
    desc: "Allows for easy merging of otherwise incompatible mods."
}

struct GameLanguage {
    steam_title: String,
    code_3: String,
}

static mut GET_GAME_LANGUAGE_ORIGINAL: extern "fastcall" fn(*const u64, *const i32) -> *const u64 = get_game_language_hook;

extern "fastcall" fn get_game_language_hook(a1: *const u64, language_index_ptr: *const i32) -> *const u64 {
    let language_index: i32 = unsafe { *language_index_ptr };

    let game_language = match language_index {
        0 => GameLanguage {
            steam_title: String::from("japanese"),
            code_3: String::from("jpn"),
        },
        _ => GameLanguage {
            steam_title: String::from("english"),
            code_3: String::from("eng"),
        }
    };

    japi::log_info!("Game language: {} ({})", game_language.steam_title, game_language.code_3);

    unsafe { GET_GAME_LANGUAGE_ORIGINAL(a1, language_index_ptr) }
}

#[unsafe(no_mangle)]
pub extern "stdcall" fn ModInit() {
    japi::log_debug!("Attempting to hook...");

    let Some(_) = japi::register_hook!(
        0x6F1970,
        get_game_language_hook,
        GET_GAME_LANGUAGE_ORIGINAL,
        "get_game_language",
        true
    ) else {
        japi::log_fatal!("Failed to hook!");
        return;
    };

    japi::log_info!("Loaded!");
}
