# Learning Rust

## Agenda

This agenda is used as an orientation.

- [ ] Setup
  - [X] General (Updating if already installed)
  - [ ] Setting up a workspace
  - [ ] (Optional) Further Rust Tools
    - [ ] `cargo binstall`
    - [ ] `cargo generate astra`
      - [ ] Dependency: Nushell
      - [ ] Dependency: Just


- [ ] Hellowo World!
- [ ] The main method
- [ ] Variables
  - [ ] Mutability
  - [ ] Global variables with static
  - [ ] Type Annotations
- [ ] Datatypes
  - [ ] Integers 
    - [ ] signed, unsigned
    - [ ] Prefixes
      - [ ] 0x
      - [ ] 0o
      - [ ] 0b
      - [ ] b'<x>'
    - [ ] Overflow
      - [ ] Wrapping add
      - [ ] Saturating add
    - [ ] isize
    - [ ] usize
  - [ ] floats 
  - [ ] characters
  - [ ] OwO What's this? So many Stwings???
    - [ ] String (Heap)
    - [ ] &str (Stack)
    - [ ] other strings
  - [ ] Tupels
  - [ ] Arrays
  - [ ] Vectors
  - [ ] HashMaps
    - [ ] insert
    - [ ] remove
    - [ ] iteration
    - [ ] map.entery("x").and_modify(|v| *v += 1).or_insert(10);
- [ ] println! and dbg!
- [ ] Control Flow
  - [ ] If 
  - [ ] Else 
  - [ ] Else If
- [ ] Functions
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
