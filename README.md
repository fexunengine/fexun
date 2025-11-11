<p align="center">
  <img src="assets/fexo_256x256.webp" alt="Fexo, FeXun mascot">
</p>

<div>&nbsp;</div>

FeXun is a visual novel engine written in Rust
that emphasizes performance, safety, and ease of use.

## Before You Continue!

Please note that FeXun in currently nothing more than a hobby project for [me](https://github.com/kurimanju-dev). I does not yet do anything that makes a game engine a game engine. If you were looking for a visual novel game engine, I suggest [Ren'Py](https://github.com/renpy/renpy).

## About This Repository

This repository contains "fexun-core", which practically makes up the entire engine, "fexun-runtime", which interfaces with "fexun-core" to generate the runtime, and various assets.

## Quick Overview

Dependencies:

- Python 3
- Rust nightly

Commands:

```
$ make // builds and links the runtime in debug mode

$ make release // builds and links the runtime with optimizations

$ make run // builds, links and runs the resulting executable

$ make release-run // builds, links and runs the resulting executable in release mode
```

## Architecture

`todo!()`