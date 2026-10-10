//! Counts the heap traffic of the process it is injected into, without ever allocating itself
//!
//! Inject it with `DYLD_INSERT_LIBRARIES` on macOS or `LD_PRELOAD` on Linux (glibc) and name a file in
//! `CFONTS_PERF_INTERPOSE_OUT`, at exit the process writes one line of counters there:
//!
//! `allocations=.. bytes=.. peak=.. live=.. mallocs=.. callocs=.. reallocs=.. aligned=.. frees=..`
//!
//! - `allocations` counts every block handed out by malloc, calloc, posix_memalign and aligned_alloc,
//!   plus every realloc whose block moved or whose usable size grew, a realloc that fits in place is no new block
//! - `bytes` adds up the sizes those allocations asked for, the new size for a counted realloc
//! - `live` and `peak` follow the usable size the allocator reports for each block, `malloc_size` on macOS and
//!   `malloc_usable_size` on Linux, `peak` is the high water mark of `live`
//! - `mallocs`, `callocs`, `reallocs` and `aligned` count the calls, `frees` counts the calls to free with a block,
//!   a free of a null pointer counts nothing, and neither does the old block a moving realloc gives back
//!
//! macOS swaps the functions through the `__DATA,__interpose` section, each replacement calls the original,
//! Linux exports the functions over glibc's and calls glibc's own `__libc_*` entry points
//!
//! The writer is registered with `atexit` by a constructor, so a process that aborts or dies by a signal writes nothing,
//! and the line is a snapshot at exit, a thread still exiting then may free after it, both cfonts binaries run one thread

use std::{
	ffi::{CStr, c_int, c_void},
	sync::atomic::{AtomicI64, AtomicU64, Ordering::Relaxed},
};

use libc::{O_CREAT, O_TRUNC, O_WRONLY, atexit, c_uint, close, getenv, open, write};

#[cfg(target_os = "linux")]
use libc::{EINVAL, ENOMEM, malloc_usable_size};
#[cfg(target_os = "macos")]
use libc::{aligned_alloc, calloc, free, malloc, malloc_size, posix_memalign, realloc};

static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static BYTES: AtomicU64 = AtomicU64::new(0);
static LIVE: AtomicI64 = AtomicI64::new(0);
static PEAK: AtomicI64 = AtomicI64::new(0);
static MALLOCS: AtomicU64 = AtomicU64::new(0);
static CALLOCS: AtomicU64 = AtomicU64::new(0);
static REALLOCS: AtomicU64 = AtomicU64::new(0);
static ALIGNED: AtomicU64 = AtomicU64::new(0);
static FREES: AtomicU64 = AtomicU64::new(0);

/// The environment variable naming the file the counters are written to
const OUT_VARIABLE: &CStr = c"CFONTS_PERF_INTERPOSE_OUT";

// THE ALLOCATOR UNDERNEATH

// glibc's own entry points, the exported replacements below stand in front of them
#[cfg(target_os = "linux")]
unsafe extern "C" {
	#[link_name = "__libc_malloc"]
	fn system_malloc(size: usize) -> *mut c_void;
	#[link_name = "__libc_calloc"]
	fn system_calloc(count: usize, size: usize) -> *mut c_void;
	#[link_name = "__libc_realloc"]
	fn system_realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
	#[link_name = "__libc_free"]
	fn system_free(ptr: *mut c_void);
	#[link_name = "__libc_memalign"]
	fn system_memalign(align: usize, size: usize) -> *mut c_void;
}

#[cfg(target_os = "macos")]
unsafe fn system_malloc(size: usize) -> *mut c_void {
	unsafe { malloc(size) }
}

#[cfg(target_os = "macos")]
unsafe fn system_calloc(count: usize, size: usize) -> *mut c_void {
	unsafe { calloc(count, size) }
}

#[cfg(target_os = "macos")]
unsafe fn system_realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
	unsafe { realloc(ptr, size) }
}

#[cfg(target_os = "macos")]
unsafe fn system_free(ptr: *mut c_void) {
	unsafe { free(ptr) }
}

#[cfg(target_os = "macos")]
unsafe fn system_posix_memalign(out: *mut *mut c_void, align: usize, size: usize) -> c_int {
	unsafe { posix_memalign(out, align, size) }
}

/// glibc has no `__libc_posix_memalign`, its memalign takes the same request once the alignment is checked
#[cfg(target_os = "linux")]
unsafe fn system_posix_memalign(out: *mut *mut c_void, align: usize, size: usize) -> c_int {
	if !align.is_power_of_two() || !align.is_multiple_of(size_of::<usize>()) {
		return EINVAL;
	}

	let ptr = unsafe { system_memalign(align, size) };
	if ptr.is_null() {
		return ENOMEM;
	}

	unsafe { *out = ptr };
	0
}

#[cfg(target_os = "macos")]
unsafe fn system_aligned_alloc(align: usize, size: usize) -> *mut c_void {
	unsafe { aligned_alloc(align, size) }
}

#[cfg(target_os = "linux")]
unsafe fn system_aligned_alloc(align: usize, size: usize) -> *mut c_void {
	unsafe { system_memalign(align, size) }
}

/// The bytes the allocator really set aside for one block
#[cfg(target_os = "macos")]
fn usable_size(ptr: *mut c_void) -> i64 {
	unsafe { malloc_size(ptr) as i64 }
}

/// The bytes the allocator really set aside for one block
#[cfg(target_os = "linux")]
fn usable_size(ptr: *mut c_void) -> i64 {
	unsafe { malloc_usable_size(ptr) as i64 }
}

// COUNTING

/// Moves the live bytes and keeps the high water mark
fn change_live(delta: i64) {
	let live = LIVE.fetch_add(delta, Relaxed) + delta;
	PEAK.fetch_max(live, Relaxed);
}

/// Counts one new block, a null pointer is a failed request and counts nothing
fn count_block(requested: usize, ptr: *mut c_void) {
	if ptr.is_null() {
		return;
	}

	ALLOCATIONS.fetch_add(1, Relaxed);
	BYTES.fetch_add(requested as u64, Relaxed);
	change_live(usable_size(ptr));
}

unsafe extern "C" fn counted_malloc(size: usize) -> *mut c_void {
	MALLOCS.fetch_add(1, Relaxed);
	let ptr = unsafe { system_malloc(size) };
	count_block(size, ptr);
	ptr
}

unsafe extern "C" fn counted_calloc(count: usize, size: usize) -> *mut c_void {
	CALLOCS.fetch_add(1, Relaxed);
	let ptr = unsafe { system_calloc(count, size) };
	count_block(count.saturating_mul(size), ptr);
	ptr
}

unsafe extern "C" fn counted_realloc(old: *mut c_void, size: usize) -> *mut c_void {
	REALLOCS.fetch_add(1, Relaxed);
	let old_usable = if old.is_null() { 0 } else { usable_size(old) };
	let ptr = unsafe { system_realloc(old, size) };

	if ptr.is_null() {
		// glibc frees the old block for a zero size, any other null leaves it untouched
		if size == 0 {
			change_live(-old_usable);
		}
		return ptr;
	}

	let new_usable = usable_size(ptr);
	if ptr != old || new_usable > old_usable {
		ALLOCATIONS.fetch_add(1, Relaxed);
		BYTES.fetch_add(size as u64, Relaxed);
	}
	change_live(new_usable - old_usable);
	ptr
}

unsafe extern "C" fn counted_posix_memalign(out: *mut *mut c_void, align: usize, size: usize) -> c_int {
	ALIGNED.fetch_add(1, Relaxed);
	let result = unsafe { system_posix_memalign(out, align, size) };
	if result == 0 {
		count_block(size, unsafe { *out });
	}
	result
}

unsafe extern "C" fn counted_aligned_alloc(align: usize, size: usize) -> *mut c_void {
	ALIGNED.fetch_add(1, Relaxed);
	let ptr = unsafe { system_aligned_alloc(align, size) };
	count_block(size, ptr);
	ptr
}

unsafe extern "C" fn counted_free(ptr: *mut c_void) {
	if !ptr.is_null() {
		FREES.fetch_add(1, Relaxed);
		change_live(-usable_size(ptr));
	}
	unsafe { system_free(ptr) }
}

// INSTALLING THE REPLACEMENTS

/// One replacement and the function it stands in for, the pair layout dyld reads from `__DATA,__interpose`
#[cfg(target_os = "macos")]
#[repr(C)]
struct Interpose {
	replacement: *const c_void,
	original: *const c_void,
}

// the pairs are written once at link time and only ever read by dyld
#[cfg(target_os = "macos")]
unsafe impl Sync for Interpose {}

#[cfg(target_os = "macos")]
#[used]
#[unsafe(link_section = "__DATA,__interpose")]
static INTERPOSE: [Interpose; 6] = [
	Interpose { replacement: counted_malloc as *const c_void, original: malloc as *const c_void },
	Interpose { replacement: counted_calloc as *const c_void, original: calloc as *const c_void },
	Interpose { replacement: counted_realloc as *const c_void, original: realloc as *const c_void },
	Interpose { replacement: counted_posix_memalign as *const c_void, original: posix_memalign as *const c_void },
	Interpose { replacement: counted_aligned_alloc as *const c_void, original: aligned_alloc as *const c_void },
	Interpose { replacement: counted_free as *const c_void, original: free as *const c_void },
];

/// Stands in for glibc's malloc
///
/// # Safety
///
/// The contract of the glibc function it replaces
#[cfg(target_os = "linux")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc(size: usize) -> *mut c_void {
	unsafe { counted_malloc(size) }
}

/// Stands in for glibc's calloc
///
/// # Safety
///
/// The contract of the glibc function it replaces
#[cfg(target_os = "linux")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn calloc(count: usize, size: usize) -> *mut c_void {
	unsafe { counted_calloc(count, size) }
}

/// Stands in for glibc's realloc
///
/// # Safety
///
/// The contract of the glibc function it replaces
#[cfg(target_os = "linux")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
	unsafe { counted_realloc(ptr, size) }
}

/// Stands in for glibc's posix_memalign
///
/// # Safety
///
/// The contract of the glibc function it replaces
#[cfg(target_os = "linux")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn posix_memalign(out: *mut *mut c_void, align: usize, size: usize) -> c_int {
	unsafe { counted_posix_memalign(out, align, size) }
}

/// Stands in for glibc's aligned_alloc
///
/// # Safety
///
/// The contract of the glibc function it replaces
#[cfg(target_os = "linux")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aligned_alloc(align: usize, size: usize) -> *mut c_void {
	unsafe { counted_aligned_alloc(align, size) }
}

/// Stands in for glibc's free
///
/// # Safety
///
/// The contract of the glibc function it replaces
#[cfg(target_os = "linux")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free(ptr: *mut c_void) {
	unsafe { counted_free(ptr) }
}

// WRITING THE COUNTERS AT EXIT

/// A fixed buffer the counters are formatted into, the exit path must not allocate either
struct Line {
	bytes: [u8; 512],
	len: usize,
}

impl Line {
	/// Appends raw bytes, whatever does not fit is dropped
	fn push(&mut self, text: &[u8]) {
		for &byte in text {
			if self.len < self.bytes.len() {
				self.bytes[self.len] = byte;
				self.len += 1;
			}
		}
	}

	/// Appends one `name=value` pair, space separated from the one before
	fn push_counter(&mut self, name: &[u8], value: i64) {
		if self.len > 0 {
			self.push(b" ");
		}
		self.push(name);
		self.push(b"=");
		if value < 0 {
			self.push(b"-");
		}

		let mut digits = [0u8; 20];
		let mut rest = value.unsigned_abs();
		let mut start = digits.len();
		loop {
			start -= 1;
			digits[start] = b'0' + (rest % 10) as u8;
			rest /= 10;
			if rest == 0 {
				break;
			}
		}
		self.push(&digits[start..]);
	}
}

/// Writes every counter as one line to the file `CFONTS_PERF_INTERPOSE_OUT` names, nothing when it is unset
extern "C" fn write_counters() {
	let path = unsafe { getenv(OUT_VARIABLE.as_ptr()) };
	if path.is_null() {
		return;
	}

	let counters: [(&[u8], i64); 9] = [
		(b"allocations", ALLOCATIONS.load(Relaxed) as i64),
		(b"bytes", BYTES.load(Relaxed) as i64),
		(b"peak", PEAK.load(Relaxed)),
		(b"live", LIVE.load(Relaxed)),
		(b"mallocs", MALLOCS.load(Relaxed) as i64),
		(b"callocs", CALLOCS.load(Relaxed) as i64),
		(b"reallocs", REALLOCS.load(Relaxed) as i64),
		(b"aligned", ALIGNED.load(Relaxed) as i64),
		(b"frees", FREES.load(Relaxed) as i64),
	];
	let mut line = Line { bytes: [0; 512], len: 0 };
	for (name, value) in counters {
		line.push_counter(name, value);
	}
	line.push(b"\n");

	let file = unsafe { open(path, O_WRONLY | O_CREAT | O_TRUNC, 0o644 as c_uint) };
	if file < 0 {
		return;
	}
	unsafe {
		write(file, line.bytes.as_ptr().cast(), line.len);
		close(file);
	}
}

/// Registers the counter writer, the loader calls this as soon as the library is in
extern "C" fn register_writer() {
	unsafe { atexit(write_counters) };
}

#[used]
#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__mod_init_func"))]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".init_array"))]
static REGISTER_WRITER: extern "C" fn() = register_writer;
