use std::process;
use std::fs;

pub fn get_code_address(_binary_name: &str) -> u64 {
    let pid = process::id();
    let proc_maps_filename = format!("/proc/{}/maps", pid);

    let contents = fs::read_to_string(proc_maps_filename).expect("Could not open the file!");

    let code_address_hex = contents.split("-").next().unwrap();

    u64::from_str_radix(code_address_hex, 16).unwrap()
}
