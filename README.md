Compile-Time Ray Tracer in Rust
================================

![compile time render](out512x512.png)

Introduction
------------
This in a Rust reimplementation of https://github.com/tcbrindle/raytracer.hpp. 
It tries to produce a ray traced image *entirely* at compile-time.
This means the final render is stored directly in the binary 
without performing any computations at run-time.

This implementation extensively exploits `const fn` nightly features to see 
what is the current state of compile-time evaluation in Rust.

Current limitations of `const fn`
---------------------------------
I tried to make this implementation close to the original, though, it was not always feasible.
C++ version uses templates to make `RayTracer` class generic over a scene 
to allow for rendering two types of scenes: statically and dynamically created. The similar applies to a canvas.
Unfortunately, it is not possible to use generic bounds to constraint type 
to have a `const` implementation of the trait, so I hardcoded type of scene with some code duplication 
for run-time and compile-time versions.

Another thing is that right now Rust doesn't support `const` function pointers and closures
which are used in the original implementation within object surface property definitions.
I have replaced them with trait.

Also, it isn't possible to use `for` loops to iterate over slices in `const` context as it requires 
const implementations of `IntoIterator` and `Iterator` traits for slices.
A simple workaround is to use `while` loop with manual indexing.

Performance
-----------
It took almost 8 minutes to render above image at compile-time on my machine (Ryzen 7 7840U).
Peak compiler's memory usage reached 6.6GB. For comparison the run-time release version executes in 0.4s on the same machine.
It makes compile-time version 70000x slower. I was using `rustc 1.87.0-nightly (f04bbc60f 2025-02-20)`.
