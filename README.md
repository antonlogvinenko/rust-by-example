## Ownership & borrowing
#### Ownership & Copy vs Move
* known size => stack, unknown stack => heap
* tradeoffs: stack is fast but restricted, heap is slower but more versatile
* ownership is introduced for managing heap data (freeing memory, but only freeing once)
* ownership extended for all types
  * stack data: just copying (types that implement `Copy`)
  * stack (refs to heap) + heap data: static checks of ownership moves (types that implement `Drop`)
* Mechanics
  * Single owner of any value with possible moving
  * When the owner goes out of scope, the value is dropped
	* Thrown away with stack at some point
	* Or `drop()` method is called
* Prevents
  * Memory leaks (not all): There's an owner with drop when it's out of scope
  * Double free errors: single owner
* Notes and observations:
  * Mutability can be changed when taking ownership (why not)
  * `Copy` means fast copying
  * `Copy` derived (and allowed) by compiler if and only if all components are `Copy`
  * `Copy` and `Drop` types can't be both implemented for the same type
  * Moving out of `&` or `&mut` (in pattern matching) requires `Copy`



#### Borrowing
* Prevents
  * Data races: can't compile code with both read and write access from difference references
  * Dangling pointers: data guaranteed to exist while reference exist
* References
  * Creating references
	* Immutable: `let x = &y` or `let ref x = y`
	* Mutable: `let x = &mut y` or `let ref mut x = y`
  * Pattern matching
	* `&` and `&mut` move the value out of the reference (require `Copy`)
	* `ref` and `ref mut` create a reference to the value
* Modes:
  * No references: owner can read and write
  * Only one `&mut` reference; owner can't read and write and borrow
  * Many `&` references; owner can only read, but not write and borrow
* Intersection
  * Rules are applied to parts of data
  * Restrictions sum up for the whole structure
  * Example:
	* In `(a, b)` `a` can be mutable borrowed while `b` is immutably borrowed
	* `a` can't be read and written through the owner
	* `b` can't be written but can be read through the owner
	* Whole tuple can't be read and written through the owner
	
* ??? dereferencing
* ??? automatic dereferencing https://doc.rust-lang.org/book/ch05-03-method-syntax.html
* ??? scope vs lifetime
* ??? chain of mut
* ??? rebinding
* ??? associated functions



## Constants
* `const` keyword and `mut` not allowed
* type annotation required
* can be declared in any scope, including the global scope
* can only be set to a constant expression



## Modules
* Package -> crate -> module
* Package: Cargo.toml + several crates
* Crate: binary/lib with crate root (root module)
  * `src/lib.rs` is the single library crate named after package
  * `src/main.rs` is the default binary crate named after package
  * multiple binary crates can be stored at `src/bin`
* Modules
  * Start at `crate` root
  * Are defined with `mod`
  * Paths inside module tree:
	* Absolute (start with `crate`)
    * Relative (start with `self` or `super`)
  * Visibility rules
	* Module sees everything in ancestors
	* Modules sees on pub path in descendants
		* Everything is private by default
		* `pub` to make public
		* `pub(X)` may add additional restrictions
  * Referring to visible objects
	* With `::` and path
	* Brining in scope with `use` keyword
		* module or its component (can be renamed with `as`)
		* listing several modules/items with `::name{self, name1, name2, ...}`
		* glob operator `*`
		* make new symbol public by using `pub use`
* Separate module files
  * Use `mod` to include files to build mod tree
  * `mod foo` is `foo.rs` or `foo/mod.rs`

* ??? Release profiles
* ??? Workspaces (cross deps, external deps versions)

## Strings
* Implementation, several levels, `नमस्ते` as example
  * bytes level: `[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]`
  * UTF-8 scalar level (Rust's `char` type): `['न', 'म', 'स', '्', 'त', 'े']` (4th and 6th are diacritics not letters)
  * grapheme clusters: `["न", "म", "स्", "ते"]`
* No support for indexing
  * Returning bytes is not something that is expected from indexing
  * Returning actual letters is not always possible in `O(1)`
* Support for slices
  * Range is specified in bytes
  * Slicing allowed only at `char` (UTF-8 scalars) boundaries, otherwise code panics


## Errors
* `?` Operator and function type
  * Implement `Try` (`Result`, `Option` etc)
  * Try::from_error
* Type mismatch
  * Container type mismatch: convert (e.g., `Result` to `Option`)
  * Error type mismatch
	* Map everything to a single error (`map_err`)
	* Wrap with Box or custom wrapper (implement `From`)



## Generics, traits, lifetimes
* Generics
  * functions, structs, enums, methods
* Traits
 * define, implement, default methods
 * as param/return values (`impl` and bound syntax, also `+`, also `where`) !!!
 * !!! Conditional method implementation: fixed type, trait bound
* Lifetimes
 * why, !!! functions, !!! structs, !!! methods, !!! lifetime elision, static lifetime
* Trait objects
  * `T: Trait` vs `T: dyn Trait` (or `T: Trait`)
  * object safety
* `impl` for traits redefines T and 'a

* https://stackoverflow.com/questions/57754901/what-is-a-fat-pointer
* https://stackoverflow.com/questions/67767207/why-are-trait-methods-with-generic-type-parameters-object-unsafe



## Iterators
* Iterator adapters (modify iterator) and consuming adaptors (move iterator in)
* `iter`, `iter_mut`, `into_iter` for marking ownership mode for original values



## Smart pointers
#### Box
* unknown size (unknown type or recursive type)
* unspecified type inside (that only implements specific traits)
* transfer data while insured that data will be moved not copied
#### Deref
* deref returns a reference
* *x -> *(x.deref())
* autodereference with * for methods, i.e. rewrites: x => *x => *(x.deref())
#### Drop
* logic
* `mom::std::drop`
#### RC, reference counting, multiple ownership, i.e. ownership+Box
* `clone()` to increase counter
* `Drop` trait to decrease counter (and cleanup owned resource when 0)
* `Weak`: `downgrade`/`upgrade`, `weak_count`/`strong_count`
#### RefCell, interior mutability, i.e. &+&mut+Box+runtimechecks
* borrow + Ref, borrow_mut + RefMut



## Concurrency
#### threads
* spawn, sleep, join, move
#### channels
* send, recv, try_recv
* ownership transfer
* receiving via iterator
* cloning transmitter
#### shared state concurrency 
* `Mutex`, lock, `LockResult<MutexGuard, Err>`
* Drop&Deref implementation
* Arc
#### Send & Sync
* T: Send = T can be sent to another thread
* T: Sync = T is safe to be referenced from another thread (&T implements Send)
  * Types that consist of types that are Sync, are Sync themselves



## Pattern matching
* refutable vs irrefutable
* syntax
  * literals, variables
  * multiple patterns, ranges
  * destructuring structs, enums, nested
  * ignoring values: _, nested _,  unused-variable, ..
  * extra conditionals
  * @ bindings



## Advanced features

#### Unsafe Rust
* why: conservative static checks & low level standard libraries
* unsafe block: isolation
* 5 features
  * Dereferencing a raw pointer
	* ignore borrowing rules
	* possibly invalid memory
	* can be null
	* no automatic cleanup
  * Calling unsafe function or method
	* what is unsafe function
	* creating safe abstraction
	* extern functions
  * accessing/modifying static variable
  * implementing unsafe trait
  * accessing union fields

#### Advanced traits
* associated type: allow only one implementation of a trait for a type
* default generic parameters and operator overloading
* disambiguation
  * passing self to specific interface
  * casting variable `<Type as Trait>::method_name()`
* using supertraits
* `newtype` pattern to overcome orphan rule

#### Aadvanced types
* `newtype`
  * Avoid confusion with units
  * Expose different API
* Type synonyms: `type` to reduce repetition
* Type that never returns: `!` can be coerced to any other type
* DST types
  * must know size
  * `Sized` trait (auto implemented if all components are Sized)
  * `T` is treated as `T: Sized`; use `?Sized` for both known and not known sizes
  * DST have extra bit of data to specify length
  * otherwise put behind a pointer of some kind
* Advanced functions and closures
  * function pointers
	* fn() -> type: for both functions and closures
	* functions: all of Fn, FnMut, FnOnce, closures: some of them
  * returning closures
	* return `Box<dyn Fn(i32) -> i32>`
* Macros
  * declarative
  * procedural
	* derive & attribute-like
	* function-like


Generics available in: Types, traits, functions, impl methods, impl traits
All of them introduce generic type parameters with restrictions (where or in-place)

<details>
<summary>code sample</summary>

```rust
struct GenVal<T>(T);
impl<T> GenVal<T> {
	fn value(&self) -> &T {
		&self.0
	}
}
```
</details>
