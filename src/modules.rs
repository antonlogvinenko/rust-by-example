//modules are created by: file.rs, mod_name/mod.rs, mod keyword
//anything in mod_name is not visible except mod.rs
//mod keyword: no args, pub(in path/self/super/crate)

//'mod' keyword inserts file contents in place

//pub(X) makes object available for module defined by X and for that module's children
//availability: available module + object available in that module

//'use' renames module path, with optional 'as'

//public structs with private fields can't be constructed using field names

//When you're inside mod tree then:
//1. You see all objects of your ancesters
//2.1 You can see objects in descendants if all objects in path are pub
//2.2 pub(X) may add additional restriction for ancestors

mod my_mod {

     fn _private_function() {}

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

          fn _private_function() {}

          pub fn public_function() {}

          pub(self) fn _pub_self_function() {}

          pub(super) fn pub_super_self_function() {}

          pub(in crate::modules::my_mod) fn pub_in_crate_my_mod_function() {}

          pub(crate) fn pub_crate_function() {}
     }

     mod private_nested {
          fn _call_from_module() {
               _private_function();
               _pub_self_function();
          }

          fn _private_function() {}

          pub fn public_function() {}

          pub(self) fn _pub_self_function() {}

          pub(super) fn pub_super_self_function() {}

          pub(in crate::modules::my_mod) fn pub_in_crate_my_mod_function() {}

          pub(crate) fn pub_crate_function() {}
     }
}

fn structures() {
     mod my_mod {
          pub struct OpenBox<T> {
               pub contents: T,
          }
          pub struct ClosedBox<T> {
               contents: T,
          }

          impl<T> ClosedBox<T> {
               pub fn new(contents: T) -> ClosedBox<T> {
                    ClosedBox { contents: contents }
               }

               pub fn unclassify_contents(&self) -> &T {
                    &self.contents
               }
          }
     }

     let open_box = my_mod::OpenBox {
          contents: String::from("public"),
     };
     assert_eq!(open_box.contents, "public");
     assert_eq!(open_box.contents, "public");
     assert_eq!(open_box.contents, "public");

     // let failed_closed_box = my_mod::ClosedBox {contents: "classified"};
     let closed_box = my_mod::ClosedBox::new("classified");
     // assert_eq!(closed_box.contents, "classified");
     assert_eq!(closed_box.unclassify_contents(), &"classified");
}

fn mod_access() {
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

fn use_declaration() {
     mod use_mod {
          pub fn public_function() -> &'static str {
               &"cake"
          }
     }

     assert_eq!(use_mod::public_function(), "cake");
     {
          use use_mod::public_function;
          assert_eq!(public_function(), "cake")
     }
     {
          use use_mod::public_function as pf;
          assert_eq!(pf(), "cake");
     }
}

fn super_and_self() {
     mod top {

          pub fn function() -> &'static str {
               "super's function"
          }

          mod cool {
               pub fn function() -> &'static str {
                    "cool's function"
               }
          }

          pub mod my {
               fn function() -> &'static str {
                    "my function"
               }

               pub fn calling() {
                    assert_eq!(function(), "my function");
                    assert_eq!(super::function(), "super's function");
                    assert_eq!(super::cool::function(), "cool's function");
               }
          }
     }

     top::my::calling();
}

pub fn main() {
     mod_access();
     super_and_self();
     use_declaration();
     structures();
}
