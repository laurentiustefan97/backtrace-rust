use std::{process, fs, io::{BufRead, BufReader}};

// Gets the start address of the section in which section_addr belongs
pub fn get_start_section(section_addr: usize) -> Result<usize, ()> {
    let pid = process::id();
    let proc_maps_filename = format!("/proc/{}/maps", pid);

    let file = fs::File::open(proc_maps_filename).unwrap();
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let mut tokens = line.split(|c| c == '-' || c == ' ');

        let section_start_addr = usize::from_str_radix(tokens.next().unwrap(), 16).unwrap();
        let section_end_addr = usize::from_str_radix(tokens.next().unwrap(), 16).unwrap();

        if section_addr >= section_start_addr && section_addr < section_end_addr {
            return Ok(section_start_addr);
        }
    }

    Err(())
}
