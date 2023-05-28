# Pendulums and Differential Equations presentation June 2023
This repository holds the source code for my English presentation on Pendulums and Differential Equations done in June 2023

## Building the code
This section covers how to build the code in this repository. If you are only after the PDF for the presentation or you are on Windows, check out the [latest release](https://github.com/Pandicon/Pendulums-and-Differential-Equations-presentation-June-2023/releases/latest). I will be publishing Windows binaries for all the simulations as well as the PDF of the presentation there.
You can always build the projects yourself if you feel like it even if the relevant builds can be found in one of the releases.
### The presentation
To build the presentation, simply use your preferred way of building LaTeX documents while installing the required packages.
### The simulations
To build the simulations, I would recommend using Cargo from the [Rust programming language](https://www.rust-lang.org/) toolchain. You can then simply run `cargo build --release` or `cargo run --release` in the directory for the relevant simulation to build or run it respectively.