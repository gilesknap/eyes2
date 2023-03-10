# Intro

This is my learning Rust exercise.

It's a rewrite of a 'evolution simulator' that I wrote in C++ with Microsoft
Foundation Classes 23 years ago (2000).

Original source is here: https://github.com/gilesknap/eyes

# How

I mostly read 'the book' of rust https://doc.rust-lang.org/book/title-page.html

But also really like this gentle intro:
https://stevedonovan.github.io/rust-gentle-intro/readme.html

# Patterns

I want to include examples of as many idiomatic patterns as possible
to get a feel for them.

Things that we have so far with **TODO**s for missing patterns:

- Chap 3
    - Variables and mutable variables
    - if , loop, while, for
- Chap 4
    - I now have a reasonable understanding of ownership and borrowing see
      for example AddCreature(Creature) being handed over via UpdateQueue
    - **TODO** not yet used slices
- Chap 5
    - structures and implementations
- Chap 6
    - enums
    - match
    - concise flow control with if let
- Chap 7
    - **TODO** probably need better packaging (currently have lib and bin in one crate)
- Chap 8
    - Vec
    - HashMap
    - **TODO** string UTF8 - could use some nice characters for rendering the world
- Chap 9
    - Use of Result and match
    - Use of expect unwrap() ok()
    - use of ? to propagate errors upward
- Chap 10
    - use of generics and traits (see Entity generic and trait)
    - perhaps to treat grass and creatures as generic entities in the world


# Questions
- I would prefer main and lib only at the root then sub folders is that achievable / desirable ?
- read through TODOs
