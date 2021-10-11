## Arithmetic operations
checked, wrapping, saturating, overflowing
byte literals b'X' b'xHH' b'\''

## Analogy
String - &str - Bytestring
Vec - slice - array

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
