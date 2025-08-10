// A WebAssembly implementation of the Barnsley fern.

mod math {
    mod math_js {
        #[link(wasm_import_module = "Math")]
        unsafe extern "C" {
            pub fn random() -> f64;
        }
    }

    pub fn random() -> f64 {
        unsafe { math_js::random() }
    }
}

/// Return pointer to allocted memory of specified size.
#[unsafe(no_mangle)]
pub extern "C" fn create_array(size: usize) -> *mut f64 {
    let mut data = Vec::with_capacity(size);
    let ptr = data.as_mut_ptr();
    std::mem::forget(data);
    ptr
}

/// Return the minimum x-coordinate value.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn min_x() -> f64 {
    -2.1820
}

/// Return the maximum x-coordinate value.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn max_x() -> f64 {
    2.6558
}

/// Return the minimum y-coordinate value.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn min_y() -> f64 {
    0.0
}

/// Return the maximum y-coordinate value.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn max_y() -> f64 {
    9.9983
}

/// Get the next set of points for the Barnsley fern given a starting point.
///
/// # Safety
///
/// The memory pointers must previously have been allocated for the specified size.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn points(
    mut xpt_prev: f64,
    mut ypt_prev: f64,
    xpts_ptr: *mut f64,
    ypts_ptr: *mut f64,
    size: usize,
) {
    let xpts = unsafe { std::slice::from_raw_parts_mut(xpts_ptr, size) };
    let ypts = unsafe { std::slice::from_raw_parts_mut(ypts_ptr, size) };

    for (xpt, ypt) in std::iter::zip(xpts, ypts) {
        let random = math::random();
        if random < 0.01 {
            *xpt = 0.0;
            *ypt = 0.16 * ypt_prev;
        } else if random < 0.86 {
            *xpt = 0.85 * xpt_prev + 0.04 * ypt_prev;
            *ypt = -0.04 * xpt_prev + 0.85 * ypt_prev + 1.6;
        } else if random < 0.93 {
            *xpt = 0.2 * xpt_prev - 0.26 * ypt_prev;
            *ypt = 0.23 * xpt_prev + 0.22 * ypt_prev + 1.6;
        } else {
            *xpt = -0.15 * xpt_prev + 0.28 * ypt_prev;
            *ypt = 0.26 * xpt_prev + 0.24 * ypt_prev + 0.44;
        }
        (xpt_prev, ypt_prev) = (*xpt, *ypt);
    }
}
