<h1 align="center">
    bararaq
</h1> 

[<img alt="github" src="https://img.shields.io/badge/github-uraneko.bararaq-A5915F?style=for-the-badge&logo=github&labelColor=3a3a3a" height="25">](https://github.com/uraneko/bararaq) 
[<img alt="crates.io" src="https://img.shields.io/crates/v/bararaq.svg?style=for-the-badge&color=E40046&logo=rust&labelColor=3a3a3a" height="25">](https://crates.io/crates/bararaq) 
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-bararaq-495c9f?style=for-the-badge&logo=docsdotrs&labelColor=3a3a3a" height="25">](https://docs.rs/bararaq) 
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/uraneko/bararaq/rust.yml?branch=main&style=for-the-badge&labelColor=3a3a3a" height="25">](https://github.com/uraneko/bararaq/actions?query=branch%3Amain)
[<img alt="license" src="https://img.shields.io/github/license/uraneko/bararaq?style=for-the-badge&labelColor=3a3a3a&color=ECD53F" height="25">](https://github.com/uraneko/bararaq/blob/main/LICENSE)

<h3>
    A Terminal User Interface (TUI) Library
</h3>
 
bararaq is a TUI library written in rust. Implementation wise, this crate sits between crossterm and ratatui. Check features to know what this can be used for. Check examples to know how to use. Check direction to know where the crate is heading.

## Support 
I'm developing on Arch Linux x86_64 Hyprland. That is to say, there should be no issues on said systems. Linux x86_64 systems should, in general, work.

If the CI builds are passing then, the lib at least builds on Windows amd64 and Apple amd64/aarch64 systems (if you encounter a problem, open an issue describing what happened).

## Features
\- input: keyboard, mouse, window inputs 

✓ keyboard input: detect raw bytes keyboard input and decode it into keyboard input events

✓ mouse input: detect and decode mouse input events (can be turned off)

✗ window input: detect window resize, focus and close events.

~ gamepad input: support for gamepad input events, meant for ascii games.

\- themes: style components' text, backgrounds and borders.

✓ console utilities: support terminal modes; raw and cooked. Support for alternate screen.

✗ fonts support: allow user to pick their font families for different texts. Support for double width/height lines.

✗ scroll: support vertical scrolling.

~ custom component shapes: support non rectangular components that take vertices instead of width and height.

✗ overlay: support for rendering components with overlay.

~ audio: support fot terminal audio output.

~ journal: add logging capabilities for input events, rendering and components manipulation.

<br>

✗ not yet implemented 

~ not yet implemented, low priority.

\- work in progress

✓ implemented 

! implemented but buggy

### Installation

> [!IMPORTANT]
> this crate does not have a working version yet.

```bash
# As this is a library crate, simply use
cargo add bararaq 
```

## Examples
Refer to the examples <a href= "examples/README.md">README</a>.

> [!IMPORTANT] 
> This crate follows the [SemVer Spec](https://semver.org/) versioning scheme.
> Expect a mess until the crate hits version 1.0.0

<br>

