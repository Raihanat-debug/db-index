# Benchmark – Database Indexing

## Objective

The objective is to implement a simple database indexing system that stores integer records and answers lookup queries efficiently. Two different indexing data structures are compared:

1. B-Tree (`BTreeSet`)
2. HashSet

Both implementations support the same operations:

- Insert a record
- Search for a record

The comparison evaluates their theoretical performance, memory usage, and practical characteristics.

---

## Algorithm 1 – B-Tree (`BTreeSet`)

Rust's `BTreeSet` is implemented as a balanced B-tree. Elements are stored in sorted order, and the tree automatically remains balanced after insertions.

### Complexity

| Operation | Complexity |
|-----------|------------|
| Insert | O(log n) |
| Search | O(log n) |
| Memory | O(n) |

where **n** is the number of stored records.

### Advantages

- Automatically balanced.
- Maintains elements in sorted order.
- Supports ordered iteration.
- Efficient for range queries.
- Predictable worst-case performance.

### Disadvantages

- Slightly slower than hash tables for exact lookups.
- Requires tree traversal during search.

---

## Algorithm 2 – HashSet

A HashSet stores records using a hash function. Each value is mapped to a bucket, allowing very fast average lookup.

### Complexity

| Operation | Complexity |
|-----------|------------|
| Insert | O(1) average |
| Search | O(1) average |
| Memory | O(n) |

### Advantages

- Very fast average lookup.
- Very fast insertion.
- Simple implementation.
- Excellent for exact membership queries.

### Disadvantages

- Elements are not stored in order.
- Does not support efficient range queries.
- Worst-case performance may degrade if many hash collisions occur.

---

## Comparison

| Feature | BTreeSet | HashSet |
|---------|----------|----------|
| Data Structure | Balanced B-Tree | Hash Table |
| Insert | O(log n) | O(1) average |
| Search | O(log n) | O(1) average |
| Ordered Elements | Yes | No |
| Range Queries | Yes | No |
| Worst-case Search | O(log n) | O(n) |
| Memory Usage | Moderate | Moderate |

---

## Running the benchmark

You can run the benchmark locally with:

```bash
cargo run --bin benchmark
```

This will measure insert and lookup performance for both data structures on 100,000 operations.

## Conclusion

Both implementations correctly solve the Database Indexing problem.

The B-Tree implementation maintains records in sorted order while guaranteeing logarithmic insertion and lookup times. This makes it well suited for database systems that require ordered traversal or range queries.

The HashSet implementation provides constant average-time insertion and lookup, making it an excellent choice for applications that require only exact record searches.

For ordered indexes and database-style queries, the B-Tree is generally the preferred structure. For fast exact lookups where ordering is unnecessary, the HashSet offers better average performance.