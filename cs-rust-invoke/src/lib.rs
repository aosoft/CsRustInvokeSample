use libc;

struct RustSample {
    number: i32,
    str: String,
}

impl RustSample {
    fn new() -> RustSample {
        RustSample { number: 0, str: String::new() }
    }

    unsafe fn new_raw() -> *mut RustSample {
        let p = Box::into_raw(Box::new(RustSample::new()));
        println!("RustSample.new_raw {:x}", std::mem::transmute::<_, usize>(p));
        p
    }

    fn destroy(&mut self) {
        unsafe {
            Box::from_raw(self as *mut RustSample);
        }
    }

    fn get_current_value(&self) -> i32 {
        self.number
    }

    fn add(&mut self, num: i32) {
        self.number += num;
    }

    fn sub(&mut self, num: i32) {
        self.number -= num;
    }

    fn append_string(&mut self, s: *const libc::c_char) -> u32 {
        unsafe {
            match std::ffi::CStr::from_ptr(s).to_str() {
                Ok(s2) => self.str = format!("{}{}", self.str, s2),
                _ => {},
            }
        }
        self.str.chars().count() as u32
    }

    fn print_chars(&self) {
        println!("[{:x}] {}", unsafe { std::mem::transmute::<_, usize>(self as *const Self) }, self.str);
    }
}

impl Drop for RustSample {
    fn drop(&mut self) {
        unsafe {
            println!("RustSample.drop {:x}", std::mem::transmute::<_, usize>(self as *const Self));
        }
    }
}

#[no_mangle]
pub unsafe extern fn create_rust_sample_instance(buffer: *mut *const (), buffer_size: u32) -> u32 {
    let table = &[
        RustSample::destroy as fn (&mut RustSample) as *const(),
        RustSample::get_current_value as fn (&RustSample) -> i32 as *const(),
        RustSample::add as fn (&mut RustSample, num: i32) as *const(),
        RustSample::sub as fn (&mut RustSample, num: i32) as *const(),
        RustSample::append_string as fn (&mut RustSample, s: *const libc::c_char) -> u32 as *const(),
        RustSample::print_chars as fn (&RustSample) as *const(),
    ];

    let required_size = (std::mem::size_of::<*const ()>() + std::mem::size_of_val(table)) as u32;

    if buffer == std::ptr::null_mut::<*const ()>() {
        return required_size;
    }

    if buffer_size < required_size {
        return 0;
    }

    let instance = RustSample::new_raw() as *const ();
    if instance == std::ptr::null::<()>() {
        return 0;
    }

    let mut p = buffer;
    *p = instance;
    p = p.add(1);
    for x in table {
        *p = *x;
        p = p.add(1);
    }

    required_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        unsafe {
            let size = create_rust_sample_instance(std::ptr::null_mut(), 0);
            let buffer = libc::malloc(size as usize) as *mut *const ();
            let size = create_rust_sample_instance(buffer, size);
            assert!(size > 0);

            let p = *buffer as *mut RustSample;
            let fn_destroy= std::mem::transmute::<_, fn (&mut RustSample)>(*(buffer.add(1)));
            let fn_get_current_value = std::mem::transmute::<_, fn (&RustSample) -> i32>(*(buffer.add(2)));
            let fn_add = std::mem::transmute::<_, fn (&mut RustSample, i32)>(*(buffer.add(3)));
            let fn_sub = std::mem::transmute::<_, fn (&mut RustSample, i32)>(*(buffer.add(4)));
            let fn_append_string = std::mem::transmute::<_, fn (&mut RustSample, *const libc::c_char) -> u32 >(*(buffer.add(5)));
            let fn_print_chars = std::mem::transmute::<_, fn (&RustSample)>(*(buffer.add(6)));

            fn_add(&mut *p, 10);
            assert_eq!(fn_get_current_value(&*p), 10);
            fn_sub(&mut *p, 5);
            assert_eq!(fn_get_current_value(&*p), 5);
            fn_append_string(&mut *p, std::ffi::CString::new("test").unwrap().as_ptr());
            fn_print_chars(&*p);
            fn_append_string(&mut *p, std::ffi::CString::new("test").unwrap().as_ptr());
            fn_print_chars(&*p);

            fn_destroy(&mut *p);

            libc::free(buffer as *mut libc::c_void);
        }
    }
}