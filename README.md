# db-index

A small Rust project that compares two common database indexing approaches:

- BTreeSet-based indexing using a balanced B-tree
- HashSet-based indexing using a hash table

The project includes:

- two runnable binaries for the index implementations
- integration tests for both implementations
- a simple benchmark binary to compare performance

## Project structure

- `src/bin/b-tree.rs` – B-tree based index implementation
- `src/bin/hashset.rs` – HashSet based index implementation
- `src/bin/benchmark.rs` – simple benchmark runner
- `tests/b-tree_test.rs` – tests for the B-tree implementation
- `tests/hashset_test.rs` – tests for the HashSet implementation
- `benchmark.md` – explanation of the benchmark and comparison

## Run the project

### Run the B-tree implementation

```bash
cargo run --bin b-tree
```

### Run the HashSet implementation

```bash
cargo run --bin hashset
```

### Run the tests

```bash
cargo test
```

### Run the benchmark

```bash
cargo run --bin benchmark
```

## Notes

This project is intended as a simple educational example of how different indexing structures behave for insert and lookup operations.
