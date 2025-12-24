# Rust Trait Collision Limitation Report

This repository demonstrates a limitation in the Rust compiler's trait coherence checker regarding associated types from upstream crates.

## The Setup

The workspace consists of three crates:

1. **`trait-crate`**: Defines a trait `Associated` with an associated type `Item`, and a generic trait `Get<T>`.
2. **`first-crate`**: Implements `Associated` for a struct `First`, setting `type Item = u32`.
3. **`second-crate`**: Depends on both. It attempts to implement `Get<T>` for its own struct `Second` twice:
    * `impl Get<i32> for Second`
    * `impl Get<<first_crate::First as Associated>::Item> for Second`

## The Issue

Logically, these two implementations should not conflict.

* The first implementation is for `T = i32`.
* The second implementation is for `T = <First as Associated>::Item`.
* Since `first-crate` defines `Item = u32`, the second implementation resolves to `T = u32`.
* `i32` and `u32` are distinct types.

However, `cargo check` fails with **Error E0119**:

```rust
error[E0119]: conflicting implementations of trait `Get<i32>` for type `Second`
  --> second-crate/src/lib.rs:10:1
   |
 9 | impl Get<i32> for Second {}
   | ------------------------ first implementation here
10 | impl Get<<first_crate::First as Associated>::Item> for Second {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `Second`
```

## Explanation

The Rust compiler's coherence checker is conservative. When checking for conflicting implementations, it encounters a projection (an associated type) `<First as Associated>::Item`. Even though the upstream crate `first-crate` has defined this type as `u32`, the compiler in `second-crate` does not fully normalize this projection to `u32` for the purpose of proving disjointness from `i32` during the coherence check.

It treats the associated type as something that *could* potentially overlap with `i32`, leading to a compilation error. This prevents valid, non-overlapping implementations from being accepted.

## Illustration

```mermaid
graph TD
    subgraph "trait-crate"
        Def[trait Associated { type Item }<br/>trait Get&lt;T&gt;]
    end

    subgraph "first-crate"
        ImplFirst[impl Associated for First {<br/>    type Item = u32<br/>}]
    end

    subgraph "second-crate"
        ImplA[impl Get&lt;i32&gt; for Second]
        ImplB[impl Get&lt;First::Item&gt; for Second]
        
        ImplA -.-x|COMPILER ERROR:<br/>Conflicting Implementation| ImplB
    end

    Def --> ImplFirst
    Def --> ImplA
    Def --> ImplB
    ImplFirst -.->|Item is u32| ImplB
```
