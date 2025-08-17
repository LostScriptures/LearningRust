# Learning Rust

## Agenda

This agenda is used as an orientation.

- [ ] Setup
  - [X] General (Updating if already installed)
  - [X] Setting up a workspace
  - [ ] (Optional) Further Rust Tools
    - [ ] `cargo binstall`
    - [ ] `cargo generate astra`
      - [ ] Dependency: Nushell
      - [ ] Dependency: Just


- [X] Hellowo World!
- [X] The main method
- [X] Variables
  - [X] Mutability
  - [X] Global variables with static
  - [X] Type Annotations
- [ ] Datatypes
  - [ ] Integers 
    - [X] signed, unsigned
    - [X] Prefixes
      - [X] 0x = hex
      - [X] 0o = octal
      - [X] 0b = binary
      - [X] b'<x>'
    - [X] Overflow
      - [X] Wrapping add
      - [X] Saturating add
    - [X] isize
    - [X] usize
  - [X] floats 
  - [X] characters
  - [X] OwO What's this? So many Stwings???
    - [X] String (Heap)
    - [X] &str (Stack)
    - [X] other strings
  - [X] Tupels
  - [X] Arrays
  - [X] Vectors
  - [X] HashMaps
    - [X] insert
    - [X] remove
    - [X] iteration
    - [X] map.entery("x").and_modify(|v| *v += 1).or_insert(10);
- [X] println! and dbg!
- [X] Control Flow
  - [X] If 
  - [X] Else 
  - [X] Else If
- [X] Functions
  - [ ] Closures
- [ ] The borrow checker
  - [ ] Ownership
    - [ ] change of ownership
    - [ ] move
  - [ ] References
    - [ ] read only
    - [ ] mutable 
  - [ ] Memory
    - [ ] Stack for scalar values
    - [ ] Heap for complex data types
- [ ] Loops
  - [ ] `loop`
  - [ ] `for` 
    - [ ] ranges
  - [ ] `while`


- [ ] Results
  - [ ] OK & Error
  - [ ] unwrap
    - [ ] or
    - [ ] or else
  - [ ] expect
- [ ] Pattern matching
  - [ ] Why?
  - [ ] `match`
  - [ ] additional conditions
  - [ ] `@`
  - [ ] `if let`
  - [ ] `while let`
  - [ ] `matches!`
- [ ] Options
  - [ ] Some
  - [ ] None
- [ ] Enums
  - [ ] Enums as Wrappers
- [ ] Structs
  - [ ] declaration
  - [ ] impl
- [ ] Deriving

- [ ] Modules and visibility

- [ ] Generics
- [ ] Traits
- [ ] Polymorphisms

- [ ] Useful libraries
  - [ ] tracing
  - [ ] serde
  - [ ] reqwest (yes, it is really spelled like this)
  - [ ] for apis
    - [ ] axum
    - [ ] tonic

- [ ] Multithreading
  - [ ] Channels
- [ ] Async with Tokio
