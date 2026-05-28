Another Game — Rust Implementation

## Overview

This project is a Rust implementation of the CSES problem **Another Game**.
The game consists of multiple heaps of coins where two players take turns removing exactly one coin from any number of non-empty heaps. The player who removes the last coin wins the game.

The main objective of this deliverable is not only to solve the problem correctly, but also to compare different algorithmic approaches and analyze their performance using benchmarks and testing.

---

# Problem Statement

Given several heaps of coins:

* On each turn, a player may select one or more non-empty heaps.
* The player removes exactly one coin from each selected heap.
* The player who removes the final coin wins.

For every test case, determine whether the first or second player wins if both players play optimally.

---

# Project Structure

```text
another_game/
├── Cargo.toml
├── README.md
├── benches/
│   └── comparison.rs
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── naive.rs
│   └── optimized.rs
├── tests/
│   └── game_tests.rs
└── input.txt
```

---

# Algorithms Used

## 1. Naive Recursive Algorithm

The naive implementation uses recursive game-state exploration.

This approach:

* Generates all possible moves.
* Simulates future game states recursively.
* Uses memoization to reduce repeated calculations.

Although this method is mathematically correct, it becomes very slow as the number of heaps increases because the number of possible game states grows exponentially.

### Complexity

* Time Complexity: Exponential
* Space Complexity: High due to recursion and memoization

---

## 2. Optimized Parity Algorithm

After analyzing the game carefully, the problem can be simplified into a parity-based observation.

### Key Observation

* If all heaps contain even numbers of coins, the second player wins.
* If at least one heap contains an odd number of coins, the first player wins.

This removes the need for recursive simulation entirely.

### Complexity

* Time Complexity: O(n)
* Space Complexity: O(1)

---

# Why the Optimized Solution Works

Every move removes exactly one coin from selected heaps.
Removing one coin changes the parity of a heap:

* Even → Odd
* Odd → Even

The game eventually reduces to tracking parity changes rather than tracking exact heap sizes.

Through analysis of small game states, it can be observed that:

* Positions where all heaps are even are losing positions.
* Positions containing at least one odd heap are winning positions.

This observation allows the optimized solution to solve the problem efficiently in linear time.

---

# Benchmarking

Benchmarks were added to compare the recursive solution against the optimized solution.

The benchmark tests demonstrate how mathematical reduction dramatically improves performance compared to recursive state exploration.

## Running Benchmarks

```bash
cargo bench
```

---

# Testing

Unit tests were added to verify correctness for:

* Sample test cases
* Single heap cases
* Even heap configurations
* Odd heap configurations
* Comparison between naive and optimized solutions

## Running Tests

```bash
cargo test
```

---

# Running the Program

## Using Manual Input

```bash
cargo run
```

Example input:

```text
3
3
1 2 3
2
2 2
4
5 5 4 5
```

Expected output:

```text
first
second
first
```

---

## Using an Input File

Create an `input.txt` file and run:

```bash
cargo run < input.txt
```

---

# Technologies Used

* Rust
* Criterion Benchmarking Library
* Cargo Testing Framework

---

# Challenges Encountered

One of the main challenges during this project was understanding the actual mathematical behavior of the game.

At first glance, the problem appears to require heavy recursive simulation. However, after analyzing smaller cases carefully, a parity pattern became visible. This significantly simplified the solution and improved performance.

Another challenge was comparing two meaningful implementations while keeping the project academically structured and easy to defend.

---

# Conclusion

This project demonstrates the importance of algorithmic analysis and optimization.

The recursive implementation helped in understanding the game mechanics, while the optimized parity solution showed how mathematical observations can drastically reduce computational complexity.

The project also demonstrates:

* modular Rust project structure,
* benchmarking,
* testing,
* and clean separation of logic.

Overall, the optimized solution provides a fast and scalable approach suitable for competitive programming constraints such as those found in CSES.
