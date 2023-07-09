# RUST_ARCHITECT
A research project by Jahidul Arafat

```agsl
Rust 
>> Setup guidelines by JAHIDUL ARAFAT

# Install rust
> curl -L https://raw.githubusercontent.com/rust-lang/rustlings/main/install.sh | bash

# Check if rust installed
> rustup —version
> rust —version
>  rustup doc
> dustup update

# If Rust build hangs
> cargo clean
> rm -rf ~/.cargo/registry/index/* ~/.cargo/.package-cache
> cargo build --verbose
> cargo run # will compile and build all parts of the program
> cargo run —release # after done debugging, to release final version


# Create a new Cargo project
https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html

# Rustlings learning and exercise
> cd rustlings
> rustlings —version
> rustlings watch

```
