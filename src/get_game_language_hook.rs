pub struct GameLanguage {
    steam_title: &'static str,
    code_3: &'static str,
}
pub static mut GAME_LANGUAGE: Option<GameLanguage> = None;

pub extern "C" fn get_game_language_hook(a1: *const u64, language_index_ptr: *const i32) -> *const u64 {
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

    unsafe { crate::GET_GAME_LANGUAGE_ORIGINAL(a1, language_index_ptr) }
}

