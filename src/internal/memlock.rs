// The memlock code below is inspired by the secstr project
// https://github.com/myfreeweb/secstr
// Which was generously released into the public domain.
// The functions were modified to deal with structs rather than slices.

#[cfg(all(unix, not(target_arch = "wasm32")))]
extern crate libc;
#[cfg(all(apple, not(target_arch = "wasm32")))]
extern crate mach_o_sys;
#[cfg(all(windows, not(target_arch = "wasm32")))]
extern crate winapi;

#[cfg(all(unix, not(target_arch = "wasm32")))]
pub fn mlock<T: Sized>(cont: &T) {
  let ptr: *const T = cont;
  let size = std::mem::size_of::<T>();
  unsafe {
    libc::mlock(ptr as *mut libc::c_void, size);
    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    libc::madvise(ptr, size, libc::MADV_NOCORE);
    #[cfg(target_os = "linux")]
    libc::madvise(ptr as *mut libc::c_void, size, libc::MADV_DONTDUMP);
  }
}

#[cfg(all(windows, not(target_arch = "wasm32")))]
pub fn mlock<T: Sized>(cont: &T) {
  let addr: *const T = cont;
  let len = std::mem::size_of::<T>();
  unsafe {
  ::winapi::um::memoryapi::VirtualLock(
    addr as ::winapi::shared::minwindef::LPVOID,
    len as ::winapi::shared::basetsd::SIZE_T,
  );
  }
}

#[cfg(any(target_arch = "wasm32", all(not(unix), not(windows))))]
pub fn mlock<T: Sized>(cont: &T) {}

#[cfg(all(unix, not(target_arch = "wasm32")))]
pub fn munlock<T: Sized>(cont: &T) {
  let ptr: *const T = cont;
  let size = std::mem::size_of::<T>();
  unsafe {
    libc::munlock(ptr as *mut libc::c_void, size);
    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    libc::madvise(ptr, size, libc::MADV_CORE);
    #[cfg(target_os = "linux")]
    libc::madvise(ptr as *mut libc::c_void, size, libc::MADV_DODUMP);
  }
}

#[cfg(all(windows, not(target_arch = "wasm32")))]
pub fn munlock<T: Sized>(cont: &T) {
  let addr: *const T = cont;
  let len = std::mem::size_of::<T>();
  unsafe {
  ::winapi::um::memoryapi::VirtualUnlock(
    addr as ::winapi::shared::minwindef::LPVOID,
    len as ::winapi::shared::basetsd::SIZE_T,
  );
  }
}

#[cfg(any(target_arch = "wasm32", all(not(unix), not(windows))))]
pub fn munlock<T: Sized>(cont: &T) {}

fn size_of_slice<T: Sized>(slice: &[T]) -> usize {
  slice.len() * std::mem::size_of::<T>()
}

#[cfg(all(unix, not(target_arch = "wasm32")))]
pub fn mlock_slice<T: Sized>(cont: &[T]) {
  let size = size_of_slice(cont);
  unsafe {
    let ptr = cont.as_ptr() as *mut libc::c_void;
    libc::mlock(ptr, size);
    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    libc::madvise(ptr, size, libc::MADV_NOCORE);
    #[cfg(target_os = "linux")]
    libc::madvise(ptr, size, libc::MADV_DONTDUMP);
  }
}

#[cfg(all(windows, not(target_arch = "wasm32")))]
pub fn mlock_slice<T: Sized>(cont: &[T]) {
  unsafe {
    let addr = cont.as_ptr() as ::winapi::shared::minwindef::LPVOID;
    let len = size_of_slice(cont);
    ::winapi::um::memoryapi::VirtualLock(addr, len as ::winapi::shared::basetsd::SIZE_T);
  }
}

#[cfg(any(target_arch = "wasm32", all(not(unix), not(windows))))]
pub fn mlock_slice<T: Sized>(cont: &[T]) {}

#[cfg(all(unix, not(target_arch = "wasm32")))]
pub fn munlock_slice<T: Sized>(cont: &[T]) {
  let size = size_of_slice(cont);
  unsafe {
    let ptr = cont.as_ptr() as *mut libc::c_void;
    libc::munlock(ptr, size);
    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    libc::madvise(ptr, size, libc::MADV_CORE);
    #[cfg(target_os = "linux")]
    libc::madvise(ptr, size, libc::MADV_DODUMP);
  }
}

#[cfg(all(windows, not(target_arch = "wasm32")))]
pub fn munlock_slice<T: Sized>(cont: &[T]) {
  unsafe {
    let addr = cont.as_ptr() as ::winapi::shared::minwindef::LPVOID;
    let len = size_of_slice(cont);
    ::winapi::um::memoryapi::VirtualUnlock(
      addr as ::winapi::shared::minwindef::LPVOID,
      len as ::winapi::shared::basetsd::SIZE_T,
    );
  }
}

#[cfg(any(target_arch = "wasm32", all(not(unix), not(windows))))]
pub fn munlock_slice<T: Sized>(cont: &[T]) {}