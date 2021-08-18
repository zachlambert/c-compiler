
# Objective

Implement my own language, using C as a base, but making various changes and additions.

I can make my own language, but make it a bit easier by starting from an existing language. I also don't have to implement compilation for the annoying parts of the language.

Requirements:
- Compilation should be fast! This is very important for productivity!

Long-term goals:
- Take the good bits from C and C++
- Add a few more features, either stuff I've come up with myself, or stuff I like from other languages
- Make a standard library
- Use the language to make a useful application
- Maybe re-write the compiler in this language at some point?

# Changes

Function definitions:
- Only a single definition - no need to forward declare, no headers.
- Make a function publicly visible with a "pub/publish/public" keyword
- Change definition to:
  func add(Vec a, Vec b) -> Vec
  (will help when adding multiple return values later, and if I add named return values)

Primitives:
- Rename short, int, etc to u8, ... u64, i8, ..., i64, f32, f64.
- No implicit casting
- Built in usize

Pointers:
- Still allow null pointers - these are useful!
- Allow explicit casting between pointer types - again, has its uses, but don't want
  it to be implicit.

Mutability:
- Default to const. Specify mutability with mut.

Arrays:
- No casting between arrays and pointers
- Arrays store their length, which you can access
- Change definition of arrays to i32[n] data (instead of i32 data[n]).

Enums:
- Use enum class by default

Built-in tagged unions and switches for them
- Basically copy rust's enums

Define function pointers more easily, and as their own type

No typedefs:
- Use "struct MyStruct {}" instead
- Add some other notation for things like "typedef f64 value_t", but add this later.

# Stuff to do later

The following changes aren't critical, but would make the language more useful.
Focus on getting the previous stuff done and finished before considering these changes.
I may also want to do another project in the meantime, and come back to this later.

These ideas are also not set in stone.

Namespaces
- Very important

Struct and Functor.
- Use structs as simple data structures.
- Use functors as objects that provide methods that do things, and store some state to help.
- The primary reason is to make it clear what the interface of the object is.
    - A struct is a compact way to pass around state.
    - A functor is a compact way to perform functions, which need some associated state.

Internal structs.
- Can have internal structs within structs (named or unnamed).
- Can have

Multiple return values.

(Optionally) named arguments (and return values? - useful for multiple return values).
- Basically, a function signature should tell you exactly what the function does most of the time.
- eg: func describeData(f64[] data) -> (f64 mean, f64 std_dev)

Configure compile time checks for naming conventions.
Don't enforce a particular style, but allow users to force their own consistent style.

Enums:
- Allow specifying values, so long as you specify the type ( enum MyEnum(i32) {...} or something) )
- Allow access to enum count (if using auto numbering)
- Explicit casting of values with my_enum.value(), instead of implicitly casting.

Templates:
- Kinda need templates. But don't want to make things slow to compile.
- There should be a way to implement templating which is much faster to compile.
