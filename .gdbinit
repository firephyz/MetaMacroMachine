set substitute-path "/rustc/de1bc0008be096cf7ed67b93402250d3b3e480d0" "/home/kyle/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust"

skip -gfi /home/kyle/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/*
# skip -gfi /rustc/de1bc0008be096cf7ed67b93402250d3b3e480d0/library/*

break syms::main

