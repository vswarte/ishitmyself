use std::mem;
use std::ops;
use std::sync;
use std::slice;
use std::collections;
use broadsword::runtime;
use broadsword::scanner;

use crate::export::Export;

pub type SingletonMap = collections::HashMap<String, usize>;
static SINGLETON_MAP: sync::OnceLock<SingletonMap> = sync::OnceLock::new();

#[derive(Debug)]
pub enum SingletonMapError {
    Pattern(broadsword::scanner::ParserError),
    Section(&'static str, SectionLookupError),
    MalformedName,
}

#[derive(Debug)]
pub enum LookupError {
    NotFound,
    SingletonMapCreation(SingletonMapError),
}

pub trait DLRFLocatable {
    const DLRF_NAME: &'static str;
}

/// Looks up instances of singleton'd classes by their name.
/// It builds a singleton map in the by matching an instruction pattern for
/// some exception creation.
/// Some singletons aren't necessarily always alive. Hence the 
/// Result<Option<T>, E>. An example of such is WorldChrMan of which an 
/// instance only exists if you're actually in the game world.
pub fn get_instance<T: DLRFLocatable>() -> Result<Option<&'static mut T>, LookupError> {
    let table = SINGLETON_MAP.get_or_init(
        || build_singleton_table()
            .map_err(LookupError::SingletonMapCreation)
            .expect("Could not create singleton map")
    );

    let ptr = table.get(T::DLRF_NAME)
        .map(usize::to_owned)
        .ok_or(LookupError::NotFound)?;

    unsafe {
        Ok((*(ptr as *const *mut T)).as_mut())
    }
}

const NULL_CHECK_PATTERN: &str = concat!(
    //  0 MOV REG, [MEM]
    "01001... 10001011 00...101 [........ ........ ........ ........]",
    //  7 TEST REG, REG
    "01001... 10000101 11......",
    // 10 JNZ +2e
    "01110101 ........",
    // 12 LEA RCX, [runtime_class_metadata]
    "01001... 10001101 00001101 [........ ........ ........ ........]",
    // 19 CALL get_singleton_name
    "11101000 [........ ........ ........ ........]",
);

/// Builds a table of all the singletons. It does so by looking for null checks
/// in the game by using an instance pattern. It then cycles over all 
/// candidates and vets the involved pointers. We expect a pointer to the 
/// instance's static, a pointer to the reflection metadata and a pointer to 
/// the get_singleton_name fn. Once all checks out we call get_singleton_name 
/// with the metadata to obtain the instance's type name.
fn build_singleton_table() -> Result<SingletonMap, SingletonMapError> {
    let (text_range, text_slice) = get_section(".text")
        .map_err(|e| SingletonMapError::Section(".text", e))?;

    let (data_range, _) = get_section(".data")
        .map_err(|e| SingletonMapError::Section(".data", e))?;

    let pattern = scanner::Pattern::from_bit_pattern(NULL_CHECK_PATTERN)
        .map_err(SingletonMapError::Pattern)?;

    let mut results: SingletonMap = Default::default();
    for candidate in scanner::simple::scan_all(text_slice, &pattern) {
        let static_offset = u32::from_le_bytes(
            candidate.captures[0].bytes.as_slice()
                .try_into().unwrap()
        );

        let metadata_offset = u32::from_le_bytes(
            candidate.captures[1].bytes.as_slice()
                .try_into().unwrap()
        );

        let fn_offset = u32::from_le_bytes(
            candidate.captures[2].bytes.as_slice()
                .try_into().unwrap()
        );

        let candidate_base = text_range.start + candidate.location;

        // Pointer to the instance of the singleton'd class
        let static_address = candidate_base + 7 + static_offset as usize;
        if !data_range.contains(&static_address) {
            continue;
        }

        // Pointer to the reflection metadata
        let metadata_addres = candidate_base + 19 + metadata_offset as usize;
        if !data_range.contains(&metadata_addres) {
            continue;
        }

        // Pointer to the name getter fn. char* get_singleton_name(metadata)
        let fn_address = candidate_base + 24 + fn_offset as usize;
        if !text_range.contains(&fn_address) {
            continue;
        }

        let get_singleton_name: extern "C" fn(usize) -> *const i8 = unsafe {
            mem::transmute(fn_address)
        };

        let cstr = unsafe {
            std::ffi::CStr::from_ptr(get_singleton_name(metadata_addres))
        };

        let name = cstr.to_str()
            .map_err(|_| SingletonMapError::MalformedName)?
            .to_string();

        results.insert(name, static_address);
    }

    Export::export(&results)
        .expect("Could not export CSV");

    Ok(results)
}

#[derive(Debug)]
pub enum SectionLookupError {
    NoGameBase,
    SectionNotFound,
}

fn get_section(
    section: &str,
) -> Result<(ops::Range<usize>, &[u8]), SectionLookupError> {
    let module = get_game_module()
        .ok_or(SectionLookupError::NoGameBase)?;

    let section_range = runtime::get_module_section_range(module, section)
        .map_err(|_| SectionLookupError::SectionNotFound)?;

    let section_slice = unsafe {
        slice::from_raw_parts(
            section_range.start as *const u8,
            section_range.end - section_range.start
        )
    };

    Ok((section_range, section_slice))
}

/// Attempts to figure out what people called the exe
fn get_game_module() -> Option<&'static str> {
    const MODULE_NAMES: [&str; 2] = [
        "eldenring.exe",
        "start_protected_game.exe",
    ];

    for name in MODULE_NAMES.iter() {
        if runtime::get_module_handle(name).is_ok() {
            return Some(name)
        }
    }
    None
}
