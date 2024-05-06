# Crates Rofio

## Overview

Crates Rofio is a utility for searching and managing Rust crates interactively using
`rofi`. It enables users to search crates on crates.io, cache them locally, and directly
access crate documentation or their pages on crates.io.

## Features

- **Search Crates**: Search for Rust crates interactively using `rofi`.
- **Cache Results**: Automatically cache search results for faster subsequent accesses.
- **Direct Access**: Directly open documentation or crates.io pages of the crates.

## Dependencies

- Rust
- Tokio (for async runtime)
- reqwest (for HTTP requests)
- serde (for JSON handling)
- webbrowser (for opening URLs in the browser)

## Setup and Installation

1. Ensure Rust is installed on your system.
2. Clone the repository:

```bash
git clone <repository-url>
```

3. Build the project:

```bash
cargo build --release
```

4. Run the application

```bash
cargo run
```

## Usage

Launch the tool, and `rofi` will prompt you to select a crate from the cache or search
online. Choose an option by typing or using arrow keys, then press Enter. If "Search
Online" is selected, type your query and proceed to view or interact with the search
results.

## License

Crates Rofio by voidfemme is marked with CC0 1.0 Universal 💖

## Philosophy

Crates Rofio is released into the public domain as a reflection of my belief in the principle
of "From each according to their ability, to each according to their needs." I believe that
software should be a collective effort, where individuals contribute their skills and knowledge
for the benefit of the community as a whole.

By dedicating this plugin to the public domain, I aim to ensure that it remains freely
available to anyone who needs it, without barriers or restrictions. I encourage users to
utilize, modify, and distribute Crates Rofio according to their needs, and I welcome
contributions from those who have the ability to improve and enhance the library.

My goal is to foster a spirit of collaboration, mutual aid, and shared ownership in the
software development community. I believe that by working together and pooling our
resources, we can create tools and technologies that serve the common good and promote a more
equitable and inclusive society.
