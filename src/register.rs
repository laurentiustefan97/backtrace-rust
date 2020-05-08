pub enum GeneralPurposeRegister {
    // Stack Pointer
    SP,
    // Program Counter
    PC,
}

#[inline(always)]
pub fn read_register(register: GeneralPurposeRegister) -> u64 {
    match register {
        GeneralPurposeRegister::SP => get_sp_value(),
        GeneralPurposeRegister::PC => get_pc_value(),
    }
}

#[inline(always)]
pub fn access_memory(address_ptr: *const u64) -> u64 {
    let ret: u64;

    unsafe { ret = *address_ptr; }

    ret
}

// x86-64
#[inline(always)]
#[cfg(any(target_arch = "x86_64"))]
fn get_sp_value() -> u64 {
    let mut rsp: u64;

    unsafe {
        asm!("" : "={rsp}"(rsp) : : : "intel")
    }

    rsp
}

#[inline(always)]
#[cfg(any(target_arch = "x86_64"))]
fn get_pc_value() -> u64 {
    let mut rip: u64;

    unsafe {
        asm!("lea rax, [rip]" : "={rax}"(rip) : : : "intel")
    }

    rip
}

// x86
#[inline(always)]
#[cfg(any(target_arch = "x86"))]
fn get_sp_value() -> u64 {
    let mut esp: u64;

    unsafe {
        asm!("" : "={esp}"(esp) : : : "intel")
    }

    esp
}

#[inline(always)]
#[cfg(any(target_arch = "x86"))]
fn get_pc_value() -> u64 {
    let mut eip: u64;

    unsafe {
        asm!("lea eax, [eip]" : "={eax}"(eip) : : : "intel")
    }

    eip
}
