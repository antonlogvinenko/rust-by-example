## Arithmetic operations
checked, wrapping, saturating, overflowing
byte literals b'X' b'xHH' b'\''

## Analogy
String - &str - Bytestring
Vec<T> - slice &[T] - array &[T, N]

Unicode: String and &str
Filenames: std::path::PathBuf and &Path
Binary data: Vec<u8> and &[u8]
Env variables, command line arguments: OsString and &OsStr
C libs: std::ffi::CString and &CStr

### Characters
'\xHH' '\u{HHHH}'

### Vectors and arrays
vectors and arrays can be auto converted to slices, thus all slice methods are allowed on vectors and arrays
default initializer for vector and arrays

## `Sized` trait
* Sized
  * Types with known size
  * Marker traits
* Unsized - can be stored only via fat pointers (& or Box)
  * `str`, `[T]`
  * `dyn T`
  * a `struct`'s last field may be unsized

struct RcBox<T: ?Sized> { ref_count: usize, value: T }
let rcBox = RcBox { ref_count: 1, value: "adasd"};
let rcBoxRef: &RcBox<dyn Display> = &rcBox;

`?Sized` means that the given type is pointed to
`Sized` bound by default

