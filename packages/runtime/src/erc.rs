use std::sync::Arc;

/// Leak the Arc to external code as a pointer
pub fn leak<T: Send + Sync + 'static>(t: Arc<T>) -> *const T {
    Arc::into_raw(t)
}

/// Free an Arc (decrement the ref count) previously leaked to external code as a pointer
///
/// The pointer MUST be one that was [`leak`]-ed
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn free<T: Send + Sync + 'static>(ptr: *const T) {
    if !ptr.is_null() {
        let _ = unsafe { Arc::from_raw(ptr) };
    }
}

/// Increment the ref count of an Arc previously leaked to external code as a pointer
///
/// The pointer MUST be one that was [`leak`]-ed
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn add_ref<T: Send + Sync + 'static>(ptr: *const T) -> *const T {
    if ptr.is_null() {
        return std::ptr::null();
    }
    let x = unsafe { Arc::from_raw(ptr) };
    // increase the ref count
    let x2 = Arc::clone(&x);
    // leak the original pointer back
    let p1 = Arc::into_raw(x);
    // leak the new pointer (should be the same one)
    let p2 = Arc::into_raw(x2);
    assert!(ptr == p1, "add_ref: input pointer mismatched");
    assert!(p1 == p2, "add_ref: output pointer mismatched");
    // return the new pointer
    p2
}
