#![feature(asm)]

mod register;
mod address;

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

pub mod backtrace {
    use super::register;
    use super::address;

    use object::Object;
    use object::read::ObjectSection;
    use memmap::Mmap;
    use gimli::UnwindSection;
    use std::{fs, borrow};
    use std::result::Result;
    use std::env;

    pub struct BacktraceGenerator {
        binary_name: String,
        pub code_address: u64,
    }

    impl BacktraceGenerator {
        pub fn new() -> BacktraceGenerator {
            // Get the executable name at runtime
            let exec_name = env::current_exe().unwrap();
            let exec_name = exec_name.to_str().unwrap();

            let code_address = address::get_code_address(exec_name);

            BacktraceGenerator {
                binary_name: String::from(exec_name),
                code_address,
            }
        }

        // The entry point of the backtrace process
        pub fn unwind_stack(&self) {
            let mut unwind_index = -1;
            // Get dwarf parser
            let file = fs::File::open(&self.binary_name).unwrap();
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
            let dwarf_cow = gimli::Dwarf::load(&load_section, &load_section_sup).unwrap();

            // Borrow a `Cow<[u8]>` to create an `EndianSlice`.
            let borrow_section: &dyn for<'a> Fn(
                &'a borrow::Cow<[u8]>,
            ) -> gimli::EndianSlice<'a, gimli::RunTimeEndian> =
                &|section| gimli::EndianSlice::new(&*section, endian);

            // The dwarf parser
            let dwarf = dwarf_cow.borrow(&borrow_section);

            // Get the instruction pointer value
            let mut ip: u64 = register::read_register(register::GeneralPurposeRegister::PC);
            // Convert the instruction pointer value to a static address
            ip -= self.code_address;

            // Get the stack pointer value
            let mut sp: u64 = register::read_register(register::GeneralPurposeRegister::SP);

            // Eh frame
            let text_section = object.section_by_name(".text").unwrap();
            let object_eh_frame = object.section_by_name(".eh_frame").unwrap();

            let eh_frame_raw = object_eh_frame.data();
            let eh_frame = gimli::EhFrame::new(&eh_frame_raw, gimli::NativeEndian);
            let mut ctx = gimli::UninitializedUnwindContext::new();
            let bases = gimli::BaseAddresses::default()
                        .set_text(text_section.address())
                        .set_eh_frame(object_eh_frame.address());

            loop {
                // Getting the function name
                let function_name = self.get_function_name(&dwarf, ip)
                                    .expect("No function was found at that address!");
                
                if unwind_index != -1 {
                    println!("{}: {}", unwind_index, function_name);
                }
                unwind_index += 1;

                // Get the unwind info for the current instruction pointer value
                let unwind_result = eh_frame.unwind_info_for_address(&bases, &mut ctx, ip,
                                                                     gimli::UnwindSection::cie_from_offset);
                
                // We finished generating the backtrace
                if let Err(_) = unwind_result {
                    break;
                }

                let unwind_info = unwind_result.unwrap();

                // println!("{:?}", unwind_info);

                match unwind_info.cfa() {
                    gimli::CfaRule::RegisterAndOffset { register, offset } => {
                        if let gimli::Register(7) = register {
                            // Now sp is the CFA
                            sp = ((sp as i64) + *offset) as u64;
                        }
                    }

                    gimli::CfaRule::Expression(expression) => {
                        // TODO
                        println!("TO BE IMPLEMENTED");
                    }
                }

                // Only return address register is of interest
                let ip_rule = unwind_info.register(gimli::Register(16));

                // Only offset rule supported now
                if let gimli::RegisterRule::Offset(offset) = ip_rule {
                    ip = register::access_memory((sp as i64 + offset) as u64) - self.code_address;
                }
            }
        }

        fn get_function_name(&self,
                             dwarf: &gimli::Dwarf<gimli::EndianSlice<'_, gimli::RunTimeEndian>>,
                             address: u64) -> Result<String, gimli::Error> {
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
                        let mut low_pc_addr = 0;
                        let mut high_pc_offset = 0;

                        dlog!("Found a function tag");

                        let name_attr = entry.attr_value(gimli::DW_AT_name)?;

                        let low_pc_attr = entry.attr_value(gimli::DW_AT_low_pc)?;
                        if let Some(gimli::AttributeValue::Addr(addr)) = low_pc_attr {
                            dlog!("The low pc is 0x{:x}", addr);
                            low_pc_addr = addr;
                        }

                        let high_pc_attr = entry.attr_value(gimli::DW_AT_high_pc)?;
                        if let Some(gimli::AttributeValue::Udata(offset)) = high_pc_attr {
                            dlog!("The high pc has the offset 0x{:x}", offset);
                            high_pc_offset = offset;
                        }

                        // Search the given address in the current function PC interval
                        if address >= low_pc_addr && address < low_pc_addr + high_pc_offset {
                            let mut function_name: &str = "";
                            dlog!("Found the function with the address {}", address);

                            // Parse the name attribute

                            // The DW_AT_name parsed for Rust binaries is AttributeValue::DebugStrRef
                            if let Some(gimli::AttributeValue::DebugStrRef(offset)) = name_attr {
                                if let Ok(s) = dwarf.debug_str.get_str(offset) {
                                    function_name = s.to_string()?;
                                }
                            }

                            // The DW_AT_name parsed for C binaries is AttributeValue::String
                            if let Some(gimli::AttributeValue::String(slice)) = name_attr {
                                function_name = slice.to_string()?;
                            }

                            if function_name != "" {
                                dlog!("The function name is {}", function_name);

                                return Ok(String::from(function_name));
                            }
                        }

                        dlog!("");
                    }
                }
            }

            Ok(String::from("Name unknown"))
        }
    }
}
