# Compiling Rust with musl

This serves as a complete, minimal example
of compiling Rust code, statically linked against [musl libc],
with support for OpenSSL.

Right now, it only works for a Linux execution platform,
but a Mac execution platform should be simple to support
by adding the relevant `repository_set` extension tags like in [this example].

See [`//example:hello-x86_64-linux-musl`](example/BUILD.bazel).

[musl libc]: https://musl.libc.org/
[this example]: https://github.com/bazelbuild/rules_rust/blob/0.59.1/examples/musl_cross_compiling/MODULE.bazel
