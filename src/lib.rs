#![feature(asm)]

mod register;
mod address;

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

    type CpuRegister = register::CpuRegister;

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
            let mut function_index = -1;
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

            // The addr2line object
            let addr2line_ctx = addr2line::Context::from_dwarf(dwarf).expect("Could not get addr2line context from dwarf object!");

            // Get the instruction pointer value
            let mut ip: u64 = register::read_register(CpuRegister::PC);
            // Convert the instruction pointer value to a static address
            ip -= self.code_address;

            // Get the stack pointer value
            let mut sp: u64 = register::read_register(CpuRegister::SP);

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
                // Don't print the current function name
                if function_index != -1 {
                    // Find functions at current code address (-1 in order to detect inline function as well)
                    let frames = addr2line_ctx.find_frames(ip - 1);
                    match frames {
                        Ok(mut frames_iter) => {
                            let mut count = 0;

                            // Iterate over the functions found
                            while let Ok(Some(frame)) = frames_iter.next() {
                                let function = frame.function.unwrap();
                                let location = frame.location.unwrap();

                                let function_name = function.demangle().unwrap();
                                let file = location.file.unwrap();
                                let line = location.line.unwrap();

                                if count == 0 {
                                    println!("{:>4}: {}", function_index, function_name);
                                } else {
                                    println!("{:>6}{}", "", function_name);
                                }
                                println!("{:>12} at {}:{}", "", file, line);

                                count += 1;
                            }

                            if count == 0 {
                                println!("{:>4}: Name unknown", function_index);
                            }
                        }

                        Err(_) => println!("{:>4}: Name unknown", function_index)
                    }
                }
                function_index += 1;

                // Get the unwind info for the current instruction pointer value
                let unwind_result = eh_frame.unwind_info_for_address(&bases, &mut ctx, ip,
                                                                     gimli::UnwindSection::cie_from_offset);
                
                // We finished generating the backtrace
                if let Err(_) = unwind_result {
                    break;
                }

                let unwind_info = unwind_result.unwrap();

                match unwind_info.cfa() {
                    gimli::CfaRule::RegisterAndOffset { register, offset } => {
                        let gimli::Register(reg_idx) = register;

                        if let Some(CpuRegister::SP) = register::reg_idx_dwarf_to_cpu(*reg_idx) {
                            // Now sp is the CFA
                            sp = ((sp as i64) + *offset) as u64;
                        }
                    }

                    gimli::CfaRule::Expression(_) => {
                        // TODO
                        println!("TO BE IMPLEMENTED");
                    }
                }

                // Only return address register is of interest
                let ra_idx = register::reg_idx_cpu_to_dwarf(CpuRegister::RA).unwrap();
                let ip_rule = unwind_info.register(gimli::Register(ra_idx));

                // Only offset rule supported now
                if let gimli::RegisterRule::Offset(offset) = ip_rule {
                    // Access the memory value where the return address is stored
                    // and translate it into a static address
                    let saved_return_address = (sp as i64 + offset) as *const u64;
                    ip = register::access_memory(saved_return_address) - self.code_address;
                }
            }
        }
    }
}
