# BabySAT Solver in Rust 


## Install (Ubuntu)
```
make install 
```
Install Rust along necessary system dependencies.

## Build
```
make build
```
Build optimized release binary that will be located in `/target/release/babysat`

## Test
```
make test
```
Test all `.cnf` files by running the command from the project root

## Debug
```
cargo run -- ARGUMENTS
```
Debug mode will include statistics
