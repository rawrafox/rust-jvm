# jvm #

A Rust library that allows you to embed and call the JVM.

## Usage

First, add the following to your `Cargo.toml`:

```toml
[dependencies]
jvm = "0.1"
```

Next, add this to your crate root:

```rust
extern crate jvm;
```

Then you can check how things are done in the https://github.com/rawrasaur/rust-jdbc repository.

## What is jvm? ##

The primary purpose of this crate is to allow you to use Java code in your Rust applications or libraries.

## Platforms ##

I am testing on my Mac, but I would love to setup some CI to get this party
started.

## Contributing ##

Patches are welcome, don't forget to add yourself to the Authors list.

## Authors ##

 - Aurora <@rawrasaur, aurora@aventine.se>
