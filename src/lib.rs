use object::Object;
use memmap::Mmap;
use std::{fs, borrow};

use std::result::Result;

// The debug version
#[cfg(feature = "logging")]
macro_rules! dlog {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

// Non-debug version
#[cfg(not(feature = "logging"))]
macro_rules! dlog {
    ($( $args:expr ),*) => {}
}

pub fn print_function_information(binary_name: &str) -> Result<(), gimli::Error> {
    let function_name: &str = "";
    let file = fs::File::open(&binary_name).unwrap();
    let mmap = unsafe { Mmap::map(&file).unwrap() };
    let object = object::File::parse(&*mmap).unwrap();
    let endian = if object.is_little_endian() {
        gimli::RunTimeEndian::Little
    } else {
        gimli::RunTimeEndian::Big
    };

    let load_section = |id: gimli::SectionId| -> Result<borrow::Cow<[u8]>, gimli::Error> {
        Ok(object
            .section_data_by_name(id.name())
            .unwrap_or(borrow::Cow::Borrowed(&[][..])))
    };

    let load_section_sup = |_| Ok(borrow::Cow::Borrowed(&[][..]));

    // Load all the sections.
    let dwarf_cow = gimli::Dwarf::load(&load_section, &load_section_sup)?;

    // Borrow a `Cow<[u8]>` to create an `EndianSlice`.
    let borrow_section: &dyn for<'a> Fn(
        &'a borrow::Cow<[u8]>,
    ) -> gimli::EndianSlice<'a, gimli::RunTimeEndian> =
        &|section| gimli::EndianSlice::new(&*section, endian);

    // Create `EndianSlice`s for all of the sections.
    let dwarf = dwarf_cow.borrow(&borrow_section);

    // Iterate over all compilation units.
    let mut iter = dwarf.units();
    while let Some(header) = iter.next()? {
        // Parse the abbreviations and other information for this compilation unit.
        let unit = dwarf.unit(header)?;

        // Iterate over all of this compilation unit's entries.
        let mut entries = unit.entries();
        while let Some((_, entry)) = entries.next_dfs()? {
            // If we find an entry for a function, print it
            if entry.tag() == gimli::DW_TAG_subprogram {
                let mut function_name: &str = "";
                dlog!("Found a function tag");

                let name_attr = entry.attr_value(gimli::DW_AT_name)?;

                // The DW_AT_name parsed for Rust binaries is AttributeValue::DebugStrRef
                if let Some(gimli::AttributeValue::DebugStrRef(offset)) = name_attr {
                    if let Ok(s) = dwarf.debug_str.get_str(offset) {
                        function_name = s.to_string()?;
                    }
                }

                // The DW_AT_name parsed for C binaries is AttributeValue::String
                if let Some(gimli::AttributeValue::String(slice)) = name_attr {
                    function_name = slice.to_string()?
                }

                if function_name != "" {
                    dlog!("The function name is {}", function_name);
                }

                let low_pc_attr = entry.attr_value(gimli::DW_AT_low_pc)?;
                if let Some(gimli::AttributeValue::Addr(address)) = low_pc_attr {
                    dlog!("The low pc is 0x{:x}", address);
                }

                let high_pc_attr = entry.attr_value(gimli::DW_AT_high_pc)?;
                if let Some(gimli::AttributeValue::Udata(offset)) = high_pc_attr {
                    dlog!("The high pc has the offset 0x{:x}", offset);
                }

                dlog!("");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // TODO tests

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
