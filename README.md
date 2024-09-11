# Rust-rev
A simple rust rev-shell.

## Requirements
- Rust installed (see [rust installation guide](https://www.rust-lang.org/tools/install))

## Installation

### Clone the repository
Clone this repo to your local machine:
```
git clone https://github.com/Bugyei-Thomas/rust-rev
cd rust-rev
```
## Build the project
Build this in release mode for optimized performance
```
carg build --release
cd target/release
```
## Setup listner with nc (or preferred listner)
```
nc -nlvp 4444
```
## Run the program
```
./reverse_shell
```
