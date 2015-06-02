
extern {
	pub fn memset(dest: *mut u32, value: u32, size: u32);
}
#[no_mangle]
pub extern fn __aeabi_memset(_dest: *mut u32, _size: u32, _value: u32) {
    memset(_dest, _value, _size);
}
