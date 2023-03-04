<div align="center">

# **R**u**s**t **re**namer
Rsre it's tool to rename file/directory<br>
<sub>
Zero dependencies 👀
</sub>

<a href="https://www.gnu.org/licenses/">
  <img src="https://img.shields.io/badge/license-MIT/APACHE-orange.svg" alt="License">
</a>
<a href="https://rust-lang.org/">
  <img src="https://img.shields.io/badge/Made%20with-Rust-orange.svg" alt="Rust">
</a>
<br>
<a href="https://github.com/TheAwiteb/rsre/actions/workflows/ci.yml">
  <img src="https://github.com/TheAwiteb/rsre/actions/workflows/ci.yml/badge.svg" alt="Continuous Integration">
</a>
<br>
<a href="https://github.com/TheAwiteb/rsre/actions/workflows/release.yml">
  <img src="https://github.com/TheAwiteb/rsre/actions/workflows/release.yml/badge.svg" alt="Release">
</a>

</div>

## Requirements
 * [Rust](https://www.rust-lang.org/) Nightly

## Install
### With Cargo
```bash
# Install nightly rust
rustup toolchain install nightly
# Install Rsre with nightly rust
cargo +nightly install rsre
rsre --version
```
### From source
```bash
# Install nightly rust
rustup toolchain install nightly
# Clone the repo
git clone https://github.com/theawiteb/rsre.git
# Change directory to it
cd rsre
# Build it with cargo
cargo +nightly build --release
# Move the binary to `/usr/bin` (Unix like system) (need permission to move in `/usr/bin`)
# You can change binary directory to `~/.cargo/bin` if its exists and it's in `$PATH`
sudo mv ./target/release/rsre /usr/bin/rsre
# Print the version
rsre --version
```

## Why
because i don't want to write full path of the new name 😶

## Using
```
USAGE:
    rsre FILE/DIRECTORY NEW_FULL_NAME

OPTIONS:
    -h, --help     Print help information
    -V, --version  Print version information
```

## Example
### Long path
```
# with mv
mv ../../foo/bar/bat/foo.txt ../../foo/bar/bat/bar.txt
# with rsre
rsre ../../foo/bar/bat/foo.txt bar.txt
```
### Single file
```
rsre bar.rs foo.rs
```


## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
