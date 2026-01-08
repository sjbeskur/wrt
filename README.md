# wrt — small DSL for coordinate transforms

This crate provides a small, ergonomic DSL around `nalgebra` to work with coordinate
frames and homogenous points. It includes typed wrappers and declarative macros to
reduce misuse and make common frame operations concise.

Features
- `Frame` and `Point` newtypes around `nalgebra::Matrix4<f64>` and `nalgebra::Vector4<f64>`.
- Declarative macros: `apply_transform!`, `compose!`, `to_frame!`, `rotate!`, `transform!`, and `hashmap!`.
- DSL-style operations such as `rotate!(A wrt B)` and angle-based rotations `rotate!(A, by 90.0, wrt B)`.

Quick examples

Usage in code (in `src/lib.rs` tests):

- Create frames/points:

```rust
use wrt::{Frame, Point};
use nalgebra::{Matrix4, Vector4};

let frame = Frame::from(Matrix4::identity());
let point = Point::from(Vector4::new(1.0, 2.0, 3.0, 1.0));
```

- Apply a transform to a point:

```rust
let p_world = crate::apply_transform!(frame, point.clone());
```

- Compose frames (typed):

```rust
let c = crate::compose!(frame_a.clone(), frame_b.clone());
```

- Convert a point from one frame to another:

```rust
let p_in_b = crate::to_frame!(point.clone(), from: T_a_world.clone(), to: T_b_world.clone());
```

- DSL rotate (conjugation):

```rust
let r = crate::rotate!(A wrt B);
```

- Angle-based rotate (degrees) — note macro parsing requires commas around the `by` and `wrt` clauses:

```rust
let r_angle = crate::rotate!(A, by 90.0, wrt B);
// or via transform! passthrough
let r2 = crate::transform!(rotate A, by 90.0, wrt B);
```

Why typed wrappers?

Using `Frame` and `Point` newtypes forces clearer intent and lets the macros expand
to method calls (`apply_to_point`, `compose`, `rotate_wrt`, ...) so misuse becomes
a compile-time type error instead of subtle runtime mistakes.

Notes and limitations
- Angle-based `rotate!` macro accepts numeric expressions but requires commas due to `macro_rules!` parsing rules: `rotate!(A, by 45.0, wrt B)`.
- If you want a more flexible, comma-free DSL (e.g. `rotate!(A by 45 wrt B)`), we can
  implement a `proc-macro` to accept arbitrary token patterns.

Running tests

```bash
cargo test
```

License

This project is personal and unlicensed — add a license if you want to publish it.

Enjoy — tell me if you'd like more DSL ops (`translate`, `scale`, `invert`, `look_at`),
or a proc-macro to loosen syntax constraints.
