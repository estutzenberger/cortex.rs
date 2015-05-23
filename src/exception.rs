use core::option::Option::{None, Some, self};

extern {
    fn __STACK_START__();
    /// Non maskable interrupt
    pub fn nmi();
    /// All class of fault
    pub fn hard_fault();
    /// System service call via SWI instruction
    pub fn svc();
    /// Pendable request for system service
    pub fn pend_sv();
    /// System tick timer
    pub fn sys_tick();
}

/// Exception "vector"
#[link_section=".exception_vector"]
pub static VECTOR: [Option<unsafe extern fn()>; 16] = [
    // OFFSET    HANDLER
    /* 0x0000 */ Some(__STACK_START__),
    /* 0x0004 */ Some(::reset),
    /* 0x0008 */ Some(nmi),
    /* 0x000C */ Some(hard_fault),
    /* 0x0010 */ None,
    /* 0x0014 */ None,
    /* 0x0018 */ None,
    /* 0x001C */ None,
    /* 0x0020 */ None,
    /* 0x0024 */ None,
    /* 0x0028 */ None,
    /* 0x002C */ Some(svc),
    /* 0x0030 */ None,
    /* 0x0034 */ None,
    /* 0x0038 */ Some(pend_sv),
    /* 0x003C */ Some(sys_tick),
];
