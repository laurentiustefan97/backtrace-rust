pub enum CpuRegister {
    // Stack Pointer
    SP,
    // Program Counter
    PC,
    // Return Address (pseudo register)
    RA,
}

#[inline(always)]
pub fn read_register(register: CpuRegister) -> usize {
    match register {
        CpuRegister::SP => get_sp_value(),
        CpuRegister::PC => get_pc_value(),
        // Should not be called this way
        CpuRegister::RA => get_pc_value(),
    }
}

#[inline(always)]
pub fn access_memory(address_ptr: *const usize) -> usize {
    let ret: usize;

    unsafe { ret = *address_ptr; }

    ret
}

// x86-64
// Given a DWARF register index, returns the associated CPU register
#[inline(always)]
#[cfg(any(target_arch = "x86_64"))]
pub fn reg_idx_dwarf_to_cpu(reg_index: u16) -> Option<CpuRegister> {
    if reg_index == 7 {
        Some(CpuRegister::SP)
    } else if reg_index == 16 {
        Some(CpuRegister::RA)
    } else {
        None
    }
}

#[inline(always)]
#[cfg(any(target_arch = "x86_64"))]
pub fn reg_idx_cpu_to_dwarf(cpu_reg: CpuRegister) -> Result<u16, ()> {
    match cpu_reg {
        CpuRegister::SP => Ok(7),
        // PC has no associated DWARF register
        CpuRegister::PC => Err(()),
        CpuRegister::RA => Ok(16),
    }
}

#[inline(always)]
#[cfg(any(target_arch = "x86_64"))]
fn get_sp_value() -> usize {
    let mut rsp: usize;

    unsafe {
        asm!("" : "={rsp}"(rsp) : : : "intel")
    }

    rsp
}

#[inline(always)]
#[cfg(any(target_arch = "x86_64"))]
fn get_pc_value() -> usize {
    let mut rip: usize;

    unsafe {
        asm!("call 1f\n1: pop rax" : "={rax}"(rip) : : : "intel")
    }

    rip
}

// x86
#[inline(always)]
#[cfg(any(target_arch = "x86"))]
pub fn reg_idx_dwarf_to_cpu(reg_index: u16) -> Option<CpuRegister> {
    if reg_index == 4 {
        Some(CpuRegister::SP)
    } else if reg_index == 8 {
        Some(CpuRegister::RA)
    } else {
        None
    }
}

#[inline(always)]
#[cfg(any(target_arch = "x86"))]
pub fn reg_idx_cpu_to_dwarf(cpu_reg: CpuRegister) -> Result<u16, ()> {
    match cpu_reg {
        CpuRegister::SP => Ok(4),
        // PC has no associated DWARF register
        CpuRegister::PC => Err(()),
        CpuRegister::RA => Ok(8),
    }
}

#[inline(always)]
#[cfg(any(target_arch = "x86"))]
fn get_sp_value() -> usize {
    let mut esp: usize;

    unsafe {
        asm!("" : "={esp}"(esp) : : : "intel")
    }

    esp
}

#[inline(always)]
#[cfg(any(target_arch = "x86"))]
fn get_pc_value() -> usize {
    let mut eip: usize;

    unsafe {
        asm!("call 1f\n1: pop eax" : "={eax}"(eip) : : : "intel")
    }

    eip
}
