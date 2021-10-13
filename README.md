## Ownership & Copy vs Move
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



## Borrowing
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





## Constants vs static
* Constants
  * `const` keyword. `mut` not allowed
  * type annotation required
  * can be declared in any scope (local or global)
  * can only be set to a constant expression
* Static variables
  * Static location in memory
  * Can be mutated (unsafe)



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
		* `pub use`
			* make new symbol public by using 
			* reorganize library API
* Separate module files
  * Use `mod` to include files to build mod tree
  * `mod foo` is `foo.rs` or `foo/mod.rs`
* Workspaces (cross deps, external deps versions)
  * Several packages that depend on each other
	* `Cargo.toml` with `[workspace]` section specifying `members = ["package1", ...]`
	* `[dependencies]` with `package = { path = "../package" }` inside
  * External depepdencies declared in the workspace not packages
	* In Cargo.toml in `workspace`: `[dependencies]` with `package = "version"` 
	* In Cargo.toml in packages: just use the dependency
	* Same `Cargo.lock` and `output` directory



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
* Types of errors
  * Unrecoverable (use `panic!` macro)
  * Recoverable (use proper return types like `Result<T, E>`)
* `panic!`
  * unwindle (default) or abort policies (set in release profile)
  * `RUST_BACKTRACE` to see stacktrace if debug symbols are enabled
* Going from recoverable to unrecoverable
  * `.unwrap()`
  * `.expect("message")`
* `?` Operator and function type
  * Implement `Try` (`Result`, `Option` etc)
  * `unwrap` or `return Err(From::from(error))`
* Type mismatch
  * Container type mismatch: convert (e.g., `Result` to `Option`)
  * Error type mismatch
	* Map everything to a single error (`map_err`)
	* Wrap with Box or custom wrapper (implement `From`)



## Generics, traits, lifetimes
* Generics
  * Available for functions, structs, enums, methods, traits, and `impl`s
  * Type restrictions for generic parameters:
	* Bounds
		* In declaration `<T: Trait1 + Trait2 + 'lifetime>`
		* In where clause `where T: Trait1 + Trait2 + 'lifetime`
		* In function's args and return type: `impl Trait1 + Trait2`
	* Specific types
		* In generic traits, default type parameter, e.g. `<T=i32>`
		* In `impl`s, specific types in place of generics
* How many instances
  * associated type - hiding generic parameter
  * specific orphan rule for generic traits
* Special interesting cases:
  * Replace type parameter with specific type in `impl` block and implement method/trait for specific parametrization of the type
  * Set restrictions for a type parameter in `impl` block and implement method/trait for a subset of types described by restrictions
* Performance: monomorphization
  * e.g. find all specific uses of `Option<T>` and generate definitions and functions for each one



## Traits
* Traits
  * define and implement for types
  * default methods
  * associated methods
  * associated consts
  * supertraits
  * relationships between types
	* associated types: allow only one implementation of a trait for a type (+ iterators)
	* generic traits (+ operator overloading)
		* orphan rule upd: can modify a foreign trait for a foreign type if the parameter is local
* Trait objects
  * `Vec<dyn Trait>`
	* or legacy version `Vec<Trait>`
  * Dynamic dispatch (instead of static dispatch - monomorphization)
	* Compiler doesn't know specific type, which method on which type to call
	* Uses pointers inside trait to reference methods
* Object safety
	* A trait is object safe if following statements are true for all of its methods
		* Return type isn't `Self` (the exact type of `Self` is unknown, which means dynamic dispatch would be required for method call on the result, too)
		* There are no generic parameters (table of possible invocations for specific types would be too big or uncalculateable)
	* More info
		* https://stackoverflow.com/questions/57754901/what-is-a-fat-pointer
		* https://stackoverflow.com/questions/67767207/why-are-trait-methods-with-generic-type-parameters-object-unsafe
* Disambiguation
  * passing `self` to the specific interface (`Person::fly(&person)`, `Wizard::fly(&wizard)`)
	* works for methods
  * casting variable `<Type as Trait>::method_name()`
	* works for methods and associated functions
* Orphan instances and orphan rule
  * When implementing a train on a type, either trait or type (or both) must be defined in current scope
  * Using `newtype` pattern to overcome orphan rule



## Lifetimes
* Prevent
  * dangling pointers (together with references = borrow checker)
* Implementation for
  * functions
  * methods
  * structs
* Lfietime elision rules
  * For method and functions
  * If arguments include only one reference or there's a `self` reference, then all output references get this lifetime
* `'static` lifetime: lives for the entire duration of the program



## Tests
* Testing utils
  * `assert!`, `assert_eq!`, `assert_ne!`
  * adding a message with `assert!(expr, message, args)`
  * expect a panic `#[should_panic]` or `#[should_panic(expected = "message"]`
  * Using `Result<T, E>` to signal test pass or failure
* Controlling how tests are run
  * `--` to pass args to test binary & `cargo test -- --help` to see them
  * Parallelism: `cargo test -- --test-threads=1`
  * Showing output: `--show-output`
  * Running a subset of tests by name or part of the name: `cargo test name`
  * Ignore tests until requested
	* Ignore `#[test]`
	* Request `cargo test -- --ignored`
* Test orgranization
  * Unit tests
	* For both library and binary crates
	* Located inside modules
	* `test` namespace: `#[cfg(test)]`, `#[test]`
	* idiom to access code under test: `use super::*;`
  * Integration tests
	* Only for library crate
	* Located in `tests` dir next to `src`
	* Each test file in `tests` is a separate `crate`
	* Run specific integration test with `cargo test --test name`
		* Use `foo/mod.rs` convention for `foo` module inside `tests` dir to avoid running it as integration test
  * documentation & benchmark tests



## Closures
* What they are
  * A table stored in `self` referring/owning variables from the context
  * An interface
	* Only `&` refs in `self` table - Fn with `apply(&self)`
	* Has `&mut` refs in `self` table - FnMut with `apply(&mut self)`
	* Has ownership in `self` table - FnOnce with `apply(self)`
  * Subtyping: `Fn` < `FnMut` < `FnOnce` based on expected an actual access to `self`
	* Can only use `Fn` where `Fn` is expected (`&self` will be enough only for `Fn`)
	* Can use both `Fn` and `FnMut` where `FnMut` is expected (`&mut self` will be enough for `Fn` and `FnMut`)
	* Can use any of `Fn`, `FnMut`, and `FnOnce` where `FnOnce` is expected (owned `self` will be enought for all three)
* Force variable capture by move
  * Ownership of variable will be moved to the closure
  * Closure type will still be deretermined by captured variables usage, not `move` keyword (can be any of `Fn`, `FnMut`, `FnOnce`)
  * Useful for moving ownership to another thread
* Function pointers
  * fn() -> type: for both functions and closures
  * functions: all of Fn, FnMut, FnOnce, closures: some of them
* Returning closures
  * return `Box<dyn Fn(i32) -> i32>`



## Iterators
* Adaptors
  * Consuming adaptors (take ownership of the iterator and call the `next` method consuming the iterator)
  * Iterator adaptors (modify iterator) to produce another iterator
* `iter`, `iter_mut`, `into_iter` for marking ownership mode for original values



## Smart pointers
* Box for size of data
  * Size too large: leep it on heap and transfer data by moving not copying
  * Size unknown
  	* unspecified type inside (that only implements specific traits)
	* type whose size can't be known at compile type (recursive type)
* A smart pointer implements `Deref` and `Drop`
  * `Deref`
	* `deref` returns a reference
	* `*x` makes compiler decide with kind of `deref` to invoke and then does: *x -> *(x.deref())
		* From `&T` to `&U` when `T: Deref<Target=U>`
		* From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
		* From `&mut T` to `&U` when `T: Deref<Target=U>`
	* deref coersion on arguments for functions & methods (i.e. x => *x => *(x.deref()))
  * `Drop`
	* called when variables goes out of scope
	* To clean up a value early: `mem::std::drop`
* RC, reference counting, multiple ownership, i.e. ownership+Box
  * `clone()` to increase counter
  * `Drop` trait to decrease counter (and cleanup owned resource when 0)
  * Avoiding memory leaks by switching between `weak_count`/`strong_count`
	* `downgrade` on `RC<T>` to get `Weak<T>`
	* `upgrade` on `Weak<T>` to get `Option<RC<T>>`
* `RefCell`, interior mutability, i.e. &+&mut+Box+runtimechecks
  * Methods
	* `borrow` returns `Ref<T>`
	* `borrow_mut` returns `RefMut<T>`
  * Either one mutable or multiple immutable references
  * Borrowing rules are checked at runtime
  * `Rc<RefCell<T>>` pattern: multiple owners + interior mutability



## Concurrency
* Models
  * green-threads M:N - not part of Rust std lib (requires runtime)
  * using OS threads 1:1
* Threads
  * `spawn`, `sleep`, `join`
* Channels
  * `send`, `recv`, `try_recv`
  * ownership transfer via `send`
  * receiving via iterator
  * cloning transmitter (`mpsc`, multiple producer, single consumer)
* Shared state concurrency 
  * `Mutex::lock()` returns `LockResult<MutexGuard, Err>`
  * `MutexGuard` implements
	* `Deref` to access value
	* `Drop` to unlock
  * `Arc`, the atomic `RC`
* Send & Sync
  * `T: Send` means `T` can be sent to another thread
  * `T: Sync` means `T` is safe to be referenced from another thread (i.e. that `&T` implements `Send`)
  * Types made up of `Send` and `Sync` entierly are also `Send` and `Sync`



## Pattern matching
* Where: `match`, `if let`, `while let`, `for`, `let`, function parameters
* Refutable vs irrefutable patterns
* syntax
  * literals, variables
  * multiple patterns, ranges
  * destructuring structs, enums, tuples; nested
  * ignoring values:
	* ignore value: `_`, nested `_`
	* ignoring unused-variable: start it with _
	* rest of the value: cut off with `..`
  * extra conditionals
  * @ bindings



## Unsafe Rust
* Why: conservative static checks & low level standard libraries
* Unsafe block:
  * Enables 5 new features (doesn't turn off borrow checker, etc)
  * Provides isolation of unsafe code
* 5 features
  * Dereferencing a raw pointer (*const or *mut, can be created in safe code)
	* ignore borrowing rules for raw pointers
	* possibly invalid memory
	* can be null
	* no automatic cleanup
  * Calling unsafe function or method
	* Means fn has requirements that must be upheld that Rust can't guarantee
	* Creating safe abstraction
	* `Extern `functions
* Accessing/modifying static variable
  * `Unsafe trait`: defining and implementing
  * Accessing `union` fields (unions for interfacing C code)



## Advamced Types
* `newtype`
  * Avoid confusion with units
  * Expose different API
* Type synonyms: `type` to reduce repetition
* Type that never returns: `!` can be coerced to any other type
* DST types
  * Size of data must be known at compile time
  * By default it is assumed the type is known
	* `Sized` trait (auto implemented if all components are Sized)
	* `T` bound is always treated as `T: Sized`
  * If size is unknown
	* Use `?Sized` for specify that types with known and unknown size can be accepted
	* Put behind a pointer of some kind
  * DST have extra bit of data to specify length



## Macros
* Declarative with `macro-rules`
* procedural
  * `derive` attribute: derive a trate, i.e. implement a trait for structs and enums
  * attribute-like: implement new attributes (i.e. on `derive` level) for structs, enums, functions etc.
  * function-like



## To cover in more detail:
* ??? automatic dereferencing https://doc.rust-lang.org/book/ch05-03-method-syntax.html
* ??? scope vs lifetime
* ??? chain of mut



## Some Rust resources
* https://blog.adamchalmers.com/pin-unpin/
* https://tokio.rs/blog/2021-09-console-dev-diary-1
* https://deterministic.space/secret-life-of-cows.html
* https://blog.sunfishcode.online/rust-programs-entirely-in-rust/
* https://smallcultfollowing.com/babysteps//blog/2021/09/08/rustacean-principles/
* https://blog.cloudflare.com/workers-rust-sdk/
* https://blog.rust-lang.org/inside-rust/2021/09/06/Splitting-const-generics.html
* https://fettblog.eu/rust-error-handling/
* https://matklad.github.io/2021/09/04/fast-rust-builds.html
* https://paper.dropbox.com/doc/libz-blitz--BSGvASWRAXciw3AgxeUZ7678Ag-ymXpoWVNDwVDigdrJ5o49
* https://blog.yoshuawuyts.com/async-overloading/
* https://cryptography.rs
* https://alexis-lozano.com/hexagonal-architecture-in-rust-1/
* https://dev.to/somedood/optimizing-immutable-strings-in-rust-2ahj
* https://dev.to/davidedelpapa/rust-for-data-science-tutorial-1-4g5j
* https://medium.com/@glebpomykalov/lets-overtake-go-fasthttp-with-rust-hyper-b2d1004914f
* https://adventures.michaelfbryan.com/posts/daily/slice-patterns/
* https://quietmisdreavus.net/code/2018/01/10/not-a-layer-cake-analogy/
* http://www.cmyr.net/blog/rust-gui-infra.html
* https://blog.kdheepak.com/loading-a-rust-library-as-a-lua-module-in-neovim.html
* https://blog.polybdenum.com/2021/08/09/when-zero-cost-abstractions-aren-t-zero-cost.html
* https://cheats.rs
* https://kerkour.com/blog/rust-avoid-lifetimes/
