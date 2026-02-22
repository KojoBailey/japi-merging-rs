use nucc_player_color_param::PlayerColorParam;
use nucc_player_color_param_asbr::{from_binary_data};
use nucc_player_color_param_json::{from_json};

use std::ffi::{c_char, CString, OsStr};
use std::io::{BufRead, Write};

japi::register_mod! {
    title: "Merging",
    author: "Kojo Bailey",
    guid: "merging",
    version: "0.1.0",
    desc: "Allows for easy merging of otherwise incompatible mods."
}

#[repr(C)]
struct NuccMemVector {
    pub unk00: *mut u64,
    pub unk08: *mut u64,
    pub unk10: *mut u64,
    pub start: *const c_char,
    pub position: *const c_char,
    pub end: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct NuccMemPlayerColorParam {
    character_id_hash: u32,
    costume_index: i32,
    padding: u64,
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

static mut NUCC_HASH: Option<unsafe extern "C" fn(*const c_char) -> u32> = None;
static mut RGBA_INT_TO_FLOAT: Option<unsafe extern "C" fn(*const f32, u32) -> *const f32> = None;
static mut ALLOCATE_PLAYERCOLORPARAM_DATA: Option<unsafe extern "C" fn(
    *const NuccMemVector, *const NuccMemPlayerColorParam, *const NuccMemPlayerColorParam) -> *const u64> = None;

struct GameLanguage {
    steam_title: &'static str,
    code_3: &'static str,
}
static mut GAME_LANGUAGE: Option<GameLanguage> = None;

static mut GET_GAME_LANGUAGE_ORIGINAL: extern "C" fn(*const u64, *const i32) -> *const u64 = get_game_language_hook;

extern "C" fn get_game_language_hook(a1: *const u64, language_index_ptr: *const i32) -> *const u64 {
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

static mut LOAD_NUCCBINARY_ORIGINAL: extern "C" fn(*const c_char, *const c_char) -> *const u64 = load_nuccbinary_hook;

extern "C" fn load_nuccbinary_hook(xfbin_path: *const c_char, chunk_name_buffer: *const c_char) -> *const u64 {
    unsafe { LOAD_NUCCBINARY_ORIGINAL(xfbin_path, chunk_name_buffer) }
}

struct UnsafeReader {
    ptr: *const u8,
    offset: usize,
}

impl UnsafeReader {
    unsafe fn new(ptr: *const u8) -> Self {
        Self { ptr, offset: 0 }
    }
}

impl std::io::Read for UnsafeReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.ptr.add(self.offset),
                buf.as_mut_ptr(),
                buf.len()
            );
            self.offset += buf.len();
            Ok(buf.len())
        }
    }
}

impl std::io::Seek for UnsafeReader {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        let new_offset = match pos {
            std::io::SeekFrom::Start(n) => n as usize,
            std::io::SeekFrom::Current(n) => (self.offset as i64 + n) as usize,
            std::io::SeekFrom::End(_) => {
                return Err(std::io::Error::new(
                        std::io::ErrorKind::Unsupported,
                        "cannot seek from end with unknown size"
                ))
            }
        };
        self.offset = new_offset;
        Ok(new_offset as u64)
    }
}

static MERGING_ROOT_PATH: &str = "japi/merging/";

static mut PARSE_PLAYERCOLORPARAM_ORIGINAL: extern "C" fn(*mut u64) -> *const u64 = parse_playercolorparam_hook;

extern "C" fn parse_playercolorparam_hook(cache_wrapper: *mut u64) -> *const u64 {
    const XFBIN_PATH: *const c_char = c"data/param/battle/PlayerColorParam.bin.xfbin".as_ptr();
    const CHUNK_NAME: *const c_char = c"PlayerColorParam".as_ptr();
    let mut result: *const u64 = unsafe { LOAD_NUCCBINARY_ORIGINAL(XFBIN_PATH, CHUNK_NAME) };
    let mut reader = unsafe { UnsafeReader::new(result as *const u8) };
    let mut data: PlayerColorParam = from_binary_data(&mut reader).unwrap();

    let merging_directory = std::path::Path::new("param/battle/PlayerColorParam");
    let directory_path = std::path::PathBuf::from(MERGING_ROOT_PATH).join(merging_directory);
    for filename in get_load_order(merging_directory) {
        let filename = filename + ".json";
        let json_path = directory_path.join(&filename);
        let json_text = std::fs::read_to_string(&json_path).unwrap();
        let json_data = from_json(&json_text).unwrap();
        data.merge(&json_data);
    }

    unsafe {
        let mut buffer_ptr: *mut NuccMemPlayerColorParam;
        let cache = cache_wrapper.add(1) as *mut NuccMemVector;
        let mut game_entry: NuccMemPlayerColorParam = std::mem::zeroed();
        for (key, value) in data.entries.into_iter() {
            let character_id_c_str = CString::new(key.character_id).unwrap();
            game_entry.character_id_hash = NUCC_HASH.unwrap()(character_id_c_str.as_ptr());
            game_entry.costume_index = key.costume_index as i32;
            result = RGBA_INT_TO_FLOAT.unwrap()(&game_entry.red, value.to_u32()) as *const u64;
            buffer_ptr = (*cache).position as *mut NuccMemPlayerColorParam;
            if (*cache).end == (*cache).position {
                result = ALLOCATE_PLAYERCOLORPARAM_DATA.unwrap()(
                    cache,
                    buffer_ptr,
                    &game_entry
                );
            } else {
                *buffer_ptr = game_entry;
                (*cache).position = (*cache).position.add(32);
            }
        }
    }

    result
}

fn get_load_order(path: &std::path::Path) -> Vec<String> {
    let path = std::path::PathBuf::from(MERGING_ROOT_PATH).join(path);

    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }

    if !path.exists() {
        japi::log_fatal!("Failed to create directory at:\n\"{}\"", path.display());
        return Vec::new();
    }

    let priority_path = std::path::PathBuf::from(&path).join("_load_order.cfg");

    let mut load_order: Vec<String> = Vec::new();

    if priority_path.exists() {
        let priority_file = std::fs::File::open(&priority_path).unwrap();
        for line in std::io::BufReader::new(priority_file).lines() {
            if let Ok(filename) = line {
                let item_path = std::path::PathBuf::from(&path).join(format!("{}.json", filename));
                if item_path.exists() {
                    load_order.push(filename);
                }
            }
        }
    }

    for entry in std::fs::read_dir(&path).unwrap() {
        if let Ok(entry) = entry {
            let entry_path = entry.path();
            if entry_path.extension() == Some(OsStr::new("json")) {
                let filename = entry_path.file_stem().unwrap().to_string_lossy().to_string();
                if !load_order.contains(&filename) {
                    load_order.insert(0, filename);
                }
            }
        }
    }

    let priority_output_file = std::fs::File::create(&priority_path).unwrap();
    let mut writer = std::io::BufWriter::new(priority_output_file);
    for filename in &load_order {
        writeln!(writer, "{}", filename).unwrap();
    }

    load_order
}

#[unsafe(no_mangle)]
pub extern "C" fn ModInit() {
    unsafe {
        NUCC_HASH = Some(std::mem::transmute(japi::offset_to_module_address(0x6C92A0)));
        RGBA_INT_TO_FLOAT = Some(std::mem::transmute(japi::offset_to_module_address(0x6DC840)));
        ALLOCATE_PLAYERCOLORPARAM_DATA = Some(std::mem::transmute(japi::offset_to_module_address(0x47EB58)));
    }

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
