#![feature(asm)]

mod register;
mod address;

pub mod backtrace {
    use super::register;
    use super::address;

    use object::{Object, read::ObjectSection, read::ObjectSegment};
    use memmap::Mmap;
    use gimli::UnwindSection;
    use std::{fs, borrow, result::Result, env, fmt};

    type CpuRegister = register::CpuRegister;

    const STATIC: i8 = 0;
    const DYNAMIC: i8 = 1;

    const ELF_TYPE_OFFSET: usize = 0x10;
    const ET_EXEC: u8 = 0x02;

    // A backtrace symbol
    pub struct BacktraceSymbol {
        name: Option<String>,
        file: Option<String>,
        line: Option<u32>,
    }

    impl BacktraceSymbol {
        pub fn new(name: Option<String>, file: Option<String>, line: Option<u32>) -> BacktraceSymbol {
            BacktraceSymbol {
                name,
                file,
                line,
            }
        }
    }

    pub struct BacktraceFrame {
        symbols: Vec<BacktraceSymbol>,
    }

    impl BacktraceFrame {
        pub fn new(symbols: Vec<BacktraceSymbol>) -> BacktraceFrame {
            BacktraceFrame {
                symbols,
            }
        }
    }

    pub struct Backtrace {
        // Frames here are listed from top-to-bottom of the stack
        frames: Vec<BacktraceFrame>,
    }

    impl Backtrace {
        pub fn parse_frames(addr2line_ctx: &addr2line::Context<gimli::EndianSlice<gimli::RunTimeEndian>>,
                            symbol_map: &object::SymbolMap,
                            code_address: usize) -> Vec<BacktraceSymbol> {
            // The vector with the parsed backtrace frames
            let mut symbols_vec = Vec::new();

            // Find functions at current code address (-1 in order to detect inline function as well)
            let frames = addr2line_ctx.find_frames(code_address as u64);

            if let Ok(mut frames_iter) = frames {
                // Iterate over the functions found
                while let Ok(Some(frame)) = frames_iter.next() {
                    let function_name;
                    let function_file;
                    let function_line;

                    let function = frame.function;
                    if let Some(function) = function {
                        function_name = Some(String::from(function.demangle().unwrap()));
                    } else {
                        function_name = None;
                    }

                    let location = frame.location;
                    if let Some(location) = location {
                        let file = location.file;
                        if let Some(file) = file {
                            function_file = Some(String::from(file));
                        } else {
                            function_file = None;
                        }

                        function_line = location.line;
                    } else {
                        function_file = None;
                        function_line = None;
                    }

                    symbols_vec.push(BacktraceSymbol::new(function_name, function_file, function_line));
                }
            }

            // If we could not detect any symbol from the DWARF sections
            // we will use the ELF symbol table
            if symbols_vec.len() == 0 {
                // Find the symbol at the current address
                let elf_symbol = symbol_map.get(code_address as u64);

                if let Some(elf_symbol) = elf_symbol {
                    if let Some(name) = elf_symbol.name() {
                        let mut demangled_name = rustc_demangle::demangle(name).to_string();

                        // If the demangled name has a "::" in their structure (rust symbol mangling),
                        // get rid of it and what is afterwards
                        // (ex. test::main::h3778bc7a1687e186 => test::main)
                        if let Some(trim_pos) = demangled_name.rfind("::") {
                            demangled_name = String::from(&demangled_name[0..trim_pos]);
                        }

                        symbols_vec.push(BacktraceSymbol::new(Some(demangled_name), None, None));
                    }
                }
            }

            symbols_vec
        }

        pub fn new() -> Backtrace {
            // Get the executable name
            let exec_name = env::current_exe().unwrap();
            let exec_name = exec_name.to_str().unwrap();

            let mut function_index = -1;
            // Get dwarf parser
            let file = fs::File::open(&exec_name).unwrap();
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

            // Load all the sections
            let dwarf_cow = gimli::Dwarf::load(&load_section, &load_section_sup).unwrap();

            // Borrow a `Cow<[u8]>` to create an `EndianSlice`.
            let borrow_section: &dyn for<'a> Fn(
                &'a borrow::Cow<[u8]>,
            ) -> gimli::EndianSlice<'a, gimli::RunTimeEndian> =
                &|section| gimli::EndianSlice::new(&*section, endian);

            // The dwarf parser
            let dwarf = dwarf_cow.borrow(&borrow_section);

            // The addr2line object
            let addr2line_ctx = addr2line::Context::from_dwarf(dwarf)
                        .expect("Could not get addr2line context from dwarf object!");

            // The symbol table
            let symbol_map = object.symbol_map();

            // Get the instruction pointer value
            let mut ip: usize = register::read_register(CpuRegister::PC);

            // Get the start address of the .text section
            let code_address: usize;

            // The type of linking used in the current binary
            let linking;

            {
                // Get the first segment from the ELF
                let data = object.segments().next().unwrap().data();

                // And check the e_type field from the header
                // ET_EXEC => static linking was used, other variant => dynamic linking
                if data[ELF_TYPE_OFFSET] == ET_EXEC {
                    linking = STATIC;
                } else {
                    linking = DYNAMIC;
                }
            }

            if linking == DYNAMIC {
                // In case of dynamic linking, determine the start address of the .text section
                code_address = address::get_start_section(ip).unwrap();
            } else {
                // In case of static linking, consider the start address 0 (no relocation offset)
                code_address = 0;
            }

            // Convert the instruction pointer value to a static address
            ip -= code_address;

            // Get the stack pointer value
            let mut sp: usize = register::read_register(CpuRegister::SP);

            // Eh frame
            let text_section = object.section_by_name(".text").unwrap();
            let object_eh_frame = object.section_by_name(".eh_frame").unwrap();

            let eh_frame_raw = object_eh_frame.data();
            let eh_frame = gimli::EhFrame::new(&eh_frame_raw, gimli::NativeEndian);
            let mut ctx = gimli::UninitializedUnwindContext::new();
            let bases = gimli::BaseAddresses::default()
                        .set_text(text_section.address())
                        .set_eh_frame(object_eh_frame.address());

            // The frames of the current backtrace
            let mut frames_vec = Vec::new();

            // Unwind the stack
            loop {
                // Don't parse the first frame information (contains the current function)
                if function_index != -1 {
                    let symbols_vec = Backtrace::parse_frames(&addr2line_ctx, &symbol_map, ip);
                    frames_vec.push(BacktraceFrame::new(symbols_vec));
                }
                function_index += 1;

                // Get the unwind info for the current instruction pointer value
                let unwind_result = eh_frame.unwind_info_for_address(&bases, &mut ctx, ip as u64,
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
                            sp = ((sp as isize) + (*offset as isize)) as usize;
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
                    let saved_return_address = (sp as isize + offset as isize) as *const usize;
                    ip = register::access_memory(saved_return_address) - code_address - 1;
                }
            }

            Backtrace {
                frames: frames_vec,
            }
        }
    }

    impl fmt::Debug for Backtrace {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut frame_index = 0;

            for frame in &self.frames {
                let mut symbol_index = 0;

                for symbol in &frame.symbols {
                    if symbol_index == 0 {
                        writeln!(fmt, "{:>4}: {}", frame_index, symbol.name.as_ref().unwrap()).unwrap();
                    } else {
                        writeln!(fmt, "{:>6}{}", "", symbol.name.as_ref().unwrap()).unwrap();
                    }

                    let symbol_file = symbol.file.as_ref();

                    // We consider that if we have the symbol's file, we will have the symbol's line as well
                    // ???
                    if let Some(_) = symbol_file {
                        writeln!(fmt, "{:>12} at {}:{}", "", symbol.file.as_ref().unwrap(), symbol.line.as_ref().unwrap()).unwrap();
                    }

                    symbol_index += 1;
                }

                frame_index += 1;
            }

            write!(fmt, "")
        }
    }
}
