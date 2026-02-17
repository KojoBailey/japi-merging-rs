japi::register_mod! {
    title: "Merging",
    author: "Kojo Bailey",
    guid: "merging",
    version: "0.1.0",
    desc: "Allows for easy merging of otherwise incompatible mods."
}

struct GameLanguage {
    steam_title: &'static str,
    code_3: &'static str,
}
static mut GAME_LANGUAGE: Option<GameLanguage> = None;

static mut GET_GAME_LANGUAGE_ORIGINAL: extern "fastcall" fn(*const u64, *const i32) -> *const u64 = get_game_language_hook;

extern "fastcall" fn get_game_language_hook(a1: *const u64, language_index_ptr: *const i32) -> *const u64 {
    let language_index: i32 = unsafe { *language_index_ptr };

    unsafe {
        GAME_LANGUAGE = Some(match language_index {
            0 => GameLanguage {
                steam_title: "japanese",
                code_3: "jpn",
            },
            2 => GameLanguage {
                steam_title: "french",
                code_3: "fre",
            },
            3 => GameLanguage {
                steam_title: "spanish",
                code_3: "spa",
            },
            4 => GameLanguage {
                steam_title: "german",
                code_3: "ger",
            },
            5 => GameLanguage {
                steam_title: "italian",
                code_3: "ita",
            },
            9 => GameLanguage {
                steam_title: "koreana",
                code_3: "kor",
            },
            10 => GameLanguage {
                steam_title: "tchinese",
                code_3: "cht",
            },
            11 => GameLanguage {
                steam_title: "schinese",
                code_3: "chs",
            },
            _ => GameLanguage {
                steam_title: "english",
                code_3: "eng",
            }
        });

        if let Some(ref lang) = GAME_LANGUAGE {
            japi::log_info!("Game language: {} ({})", lang.steam_title, lang.code_3);
        }
    }

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
