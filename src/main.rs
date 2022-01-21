extern "C" {
    fn vDSP_maxmgv(__A: *const f32, __IA: i64, __C: *mut f32, __N: u64);
    fn vDSP_meamgv(__A: *const f32, __IA: i64, __C: *mut f32, __N: u64);
    fn vDSP_rmsqv(__A: *const f32, __IA: i64, __C: *mut f32, __N: u64);
}

fn main() {
    example_max_magnitude();
    example_mean_magnitude();
    example_root_mean_square();
}

///
/// See: https://developer.apple.com/documentation/accelerate/1449986-vdsp_maxmgv?language=objc
///
fn example_max_magnitude() {
    let a = [-1.5, 2.25, 3.6, 0.2, -0.1, -4.3];
    let ia: i64 = 1;
    let mut c = f32::NAN;
    let n: u64 = a.len() as u64;
    unsafe {
        vDSP_maxmgv(a.as_ptr(), ia, &mut c, n);
    }
    println!("max magnitude: {c}");
}

///
/// See: https://developer.apple.com/documentation/accelerate/1449731-vdsp_meamgv?language=objc
///
fn example_mean_magnitude() {
    let a = [-1.5, 2.25, 3.6, 0.2, -0.1, -4.3];
    let ia: i64 = 1;
    let mut c = f32::NAN;
    let n: u64 = a.len() as u64;
    unsafe {
        vDSP_meamgv(a.as_ptr(), ia, &mut c, n);
    }
    println!("mean magnitude: {c}");
}

///
/// See: https://developer.apple.com/documentation/accelerate/1450655-vdsp_rmsqv?language=objc
///
fn example_root_mean_square() {
    let a = [-1.5, 2.25, 3.6, 0.2, -0.1, -4.3];
    let ia: i64 = 1;
    let mut c = f32::NAN;
    let n: u64 = a.len() as u64;
    unsafe {
        vDSP_rmsqv(a.as_ptr(), ia, &mut c, n);
    }
    println!("root mean square: {c}");
}
