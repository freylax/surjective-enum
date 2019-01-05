# surjective-enum
Rust macro to derive a surjective mapping from enum representation to enum.

[![Build Status](https://travis-ci.org/freylax/surjective-enum.svg?branch=master)](https://travis-ci.org/freylax/surjective-enum)


Derive a surjective ::core::convert::From<Unitary Enum Representation> conversion function
which maps all values which are not part of the enumeration to the last
enum discriminant.

The example
``` rust
	use surjective_enum::From;
    #[repr(u8)]
	#[derive(From)]
    pub enum Enum {
      Bar  = 0b00,
      Foo  = 0b01,
      Rest = 0b11
    }
```
will create a from(u8) -> Enum conversion function which maps
 0 -> Bar, 1 -> Foo and all other values to Rest.
