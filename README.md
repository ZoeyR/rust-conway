#rust-conway [![Build Status](https://travis-ci.org/dgriffen/rust-conway.svg?branch=master)](https://travis-ci.org/dgriffen/rust-conway)

Conway's game of life implemented in rust

##How to build rust-conway

1. Install [Rust](https://github.com/rust-lang/rust) and [Cargo](https://github.com/rust-lang/cargo) either by building from source, or to install nightlies open a terminal and type:

  ```
  curl -s https://static.rust-lang.org/rustup.sh | sudo sh
  ```
2. Type the following into the terminal:

  ```
  git clone https://github.com/dgriffen/rust-conway
  cd rust-conway
  cargo build
  ```

##Goals
* Have a proper gui
* Allow dynamic switching of engines
* Provide variable generation speed
* Have modifiable rules
