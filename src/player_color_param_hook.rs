use crate::load_order::get_load_order;

use nucc_player_color_param::PlayerColorParam;
use nucc_player_color_param_asbr::{from_binary_data};
use nucc_player_color_param_json::{from_json};

use std::ffi::{c_char, CString};

#[repr(C)]
pub struct NuccMemVector {
    pub unk00: *mut u64,
    pub unk08: *mut u64,
    pub unk10: *mut u64,
    pub start: *const c_char,
    pub position: *const c_char,
    pub end: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NuccMemPlayerColorParam {
    character_id_hash: u32,
    costume_index: i32,
    padding: u64,
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
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

pub extern "C" fn parse_playercolorparam_hook(cache_wrapper: *mut u64) -> *const u64 {
    const XFBIN_PATH: *const c_char = c"data/param/battle/PlayerColorParam.bin.xfbin".as_ptr();
    const CHUNK_NAME: *const c_char = c"PlayerColorParam".as_ptr();
    let mut result: *const u64 = unsafe { crate::LOAD_NUCCBINARY_ORIGINAL(XFBIN_PATH, CHUNK_NAME) };
    let mut reader = unsafe { UnsafeReader::new(result as *const u8) };
    let mut data: PlayerColorParam = from_binary_data(&mut reader).unwrap();

    let merging_directory = std::path::Path::new("param/battle/PlayerColorParam");
    let directory_path = std::path::PathBuf::from(crate::MERGING_ROOT_PATH).join(merging_directory);
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
            game_entry.character_id_hash = crate::NUCC_HASH.unwrap()(character_id_c_str.as_ptr());
            game_entry.costume_index = key.costume_index as i32;
            result = crate::RGBA_INT_TO_FLOAT.unwrap()(&game_entry.red, value.to_u32()) as *const u64;
            buffer_ptr = (*cache).position as *mut NuccMemPlayerColorParam;
            if (*cache).end == (*cache).position {
                result = crate::ALLOCATE_PLAYERCOLORPARAM_DATA.unwrap()(
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

