use libc;

struct RustSample {
    number: i32,
    str: String,
}

impl RustSample {
    fn destroy(&mut self) {
        unsafe {
            libc::free(self as *mut RustSample as *mut libc::c_void);
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

    fn append_string_slice(&mut self, s: &str) {
        self.str = format!("{}{}", self.str, s);
    }

    fn print_chars(&self) {
        println!("{}", self.str);
    }
}

impl RustSample {
    unsafe fn new() -> *mut RustSample {
        let p = libc::malloc(std::mem::size_of::<RustSample>());
        let p2 = p as *mut RustSample;
        if p2 != std::ptr::null_mut::<RustSample>() {
            libc::memset(p, 0, std::mem::size_of::<RustSample>());
            *p2 = RustSample { number: 0, str: String::new() };
        }
        p2
    }
}

#[no_mangle]
pub unsafe fn create_rust_sample_instance(buffer: *mut *const (), buffer_size: u32) -> u32 {
    let table = &[
        RustSample::destroy as fn (&mut RustSample) as *const(),
        RustSample::get_current_value as fn (&RustSample) -> i32 as *const(),
        RustSample::add as fn (&mut RustSample, num: i32) as *const(),
        RustSample::sub as fn (&mut RustSample, num: i32) as *const(),
        RustSample::append_string_slice as fn (&mut RustSample, s: &str) as *const(),
        RustSample::print_chars as fn (&RustSample) as *const(),
    ];

    let required_size = (std::mem::size_of::<*const ()>() + std::mem::size_of_val(table)) as u32;

    if buffer == std::ptr::null_mut::<*const ()>() {
        return required_size;
    }

    if buffer_size < required_size {
        return 0;
    }

    let instance = RustSample::new() as *const ();
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
            let fn_add = std::mem::transmute::<_, fn (&mut RustSample, num: i32)>(*(buffer.add(3)));
            let fn_sub = std::mem::transmute::<_, fn (&mut RustSample, num: i32)>(*(buffer.add(4)));
            let fn_append_string_slice = std::mem::transmute::<_, fn (&mut RustSample, s: &str)>(*(buffer.add(5)));
            let fn_print_chars = std::mem::transmute::<_, fn (&RustSample)>(*(buffer.add(6)));

            fn_add(&mut *p, 10);
            assert_eq!(fn_get_current_value(&*p), 10);
            fn_sub(&mut *p, 5);
            assert_eq!(fn_get_current_value(&*p), 5);
            fn_append_string_slice(&mut *p, "test");
            fn_print_chars(&*p);
            fn_append_string_slice(&mut *p, "test");
            fn_print_chars(&*p);

            fn_destroy(&mut *p);

            libc::free(buffer as *mut libc::c_void);
        }
    }
}