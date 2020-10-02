//modules are created by: file.rs, mod_name/mod.rs, mod keyword
//anything in mod_name is not visible except mod.rs
//mod keyword: no args, pub(in path/self/super/crate)

//'mod' keyword inserts file contents in place

//pub(X) makes object available for module defined by X and for that module's children
//availability: available module + object available in that module

//'use' renames module path, with optional 'as'

//public structs with private fields can't be constructed using field names


mod my_mod {
     fn _private_function() {

     }

     pub fn public_function() {
          //Private in public_nested
          //public_nested::private_function();
          //Private in public_nested
          //private_nested::pub_self_function();
          //Public for crate::modules::my_mod and children
          public_nested::pub_super_self_function();
          //Public for crate::modules::my_mod and children
          public_nested::pub_in_crate_my_mod_function();
          //Public for crate and children
          public_nested::pub_crate_function();

          // private_nested::private_function();
          private_nested::public_function();
          // private_nested::pub_self_function();
          private_nested::pub_super_self_function();
          private_nested::pub_in_crate_my_mod_function();
          private_nested::pub_crate_function();
     }

     pub mod public_nested {
          fn _call_from_module() {
               _private_function();
               //Self module Can call function available for self module
               _pub_self_function();
               //Public for crate::modules::my_mod and children
               pub_super_self_function();
               //Public for crate::modules::my_mod and children
               pub_in_crate_my_mod_function();
               //Public for crate and children
               pub_crate_function();
          }

          fn _private_function() {
               
          }

          pub fn public_function() {

          }

          pub(self) fn _pub_self_function() {
               

          }
          
          pub(super) fn pub_super_self_function() {

          }

          pub(in crate::modules::my_mod) fn pub_in_crate_my_mod_function() {

          }

          pub(crate) fn pub_crate_function() {

          }
     }

     mod private_nested {
          fn _call_from_module() {
               _private_function();
               _pub_self_function();
          }

          fn _private_function() {

          }

          pub fn public_function() {
          }

          pub(self) fn _pub_self_function() {

          }
          
          pub(super) fn pub_super_self_function() {

          }

          pub(in crate::modules::my_mod) fn pub_in_crate_my_mod_function() {

          }

          pub(crate) fn pub_crate_function() {
               
          }
     }
}


pub fn main() {
     /*
      * my_mod::
      */
     //Can't call private function
     //my_mod::private_function();
     //Can call public function
     my_mod::public_function();

     /*
      * my_mod::public_nested::
      */
     //Can't call private function
     //my_mod::public_nested::private_function();
     //Can call public function
     my_mod::public_nested::public_function();
     //Can't call effectively private function - pub(self) - visible in specific module
     //my_mod::public_nested::pub_self_function();
     //Can't call effectively private function - pub(parent) - visible in specific module
     //my_mod::public_nested::pub_super_self_function();
     //Can't call effectively private function - pub(in crate::modules::my_mod) - visible in specific module
     //my_mod::public_nested::pub_in_crate_my_mod_function();
     //Can call function public for current crate - pub(crate)
     my_mod::public_nested::pub_crate_function();

     /*
     * my_mod::private_nested
     * Can't call anything inside private module
     */
     // my_mod::private_nested::private_function();
     // my_mod::private_nested::public_function();
     // my_mod::private_nested::pub_self_function();
     // my_mod::private_nested::pub_super_self_function();
     // my_mod::private_nested::pub_in_crate_my_mod_function();
     // my_mod::private_nested::pub_crate_function();
}