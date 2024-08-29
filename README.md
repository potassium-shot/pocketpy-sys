# Pocketpy Rust FFI Bindings

[Pocketpy](https://github.com/pocketpy/pocketpy) is a portable python 3.x interpreter written in C11.
These are low-level Rust bindings to Pocketpy. I may make higher level bindings in the future.

The one issue I've been having is that most linkers can't link pocketpy's static library if it is compiled with IPO (interprocedural optimizations), which will obviously result in a drop of performance.
I've seen other people have the same kind of issues while linking C code to Rust code, but I'm not aware of any solution. If you think of something, feel free to make an issue/pr.

