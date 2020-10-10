/* Attrbiute is meta info added to module, crate, or item
 * - conditional compilation
 * - crate name, version, type (binary/library)
 * - disable warnings
 * - enable compiler features
 * - link to foreign library
 * - mark function as unit test
 * - mark function as benchmark
 *
 * Applied to whole crate: #![crate_attribute]
 * Applied to single module/item: #[crate_attribute]
 *
 * Syntax:
 * #[attribute = "value"]
 * #[attribute(key = "value")]
 * #[attribute(value)]
 * #[attribute(value1, value2, value3, value4)]
 * 
 * Conditionals:
 * #[cfg(...)] attribute
 * cfg!(...) macro
 * #[cfg(custom_condition)] and cfg!(some_condition) if --cfg passed to compiler "rustc --cfg some_condition"
 */

 //if "lib", then no longer need to pass --crate-type to rustc
//#![crate_type = "bin"]
//#![crate_name = "rust_by_example"]

#[allow(dead_code)]
fn dead_code() {}

fn configuration() {
     #[cfg(target_os = "linux")]
     fn are_you_on_linux() {

     }
     
     #[cfg(not(target_os = "linux"))]
     fn are_you_not_on_linux() {
          
     }

     if cfg!(target_os = "linux") {
          are_you_not_on_linux();
     } else {
          // are_you_on_linux();
     }
}

pub fn main() {
     configuration();
}
