This Rust program generates all points on a Montgomery curve defined over a finite field $\mathbb{F}_p$. The curve is defined by the equation:

$B \cdot y^2 \equiv x^3 + A \cdot x^2 + x \mod P$

where $A$ and $B$ are constants, and $P$ is a prime number that defines the field.

## Overview

Montgomery curves are significant in elliptic curve cryptography due to their efficient point addition formulas. This implementation systematically checks each point in the finite field $\mathbb{F}_p$ to determine if it lies on the defined curve.

## Code Structure

The code consists of the following key components:

- **Constants**:
  - `A`: Curve parameter.
  - `B`: Curve parameter.
  - `P`: Prime defining the field $\mathbb{F}_p$.

- **Functions**:
  - `is_point_on_curve(x: i64, y: i64) -> bool`: Validates whether a given point $(x, y)$ satisfies the Montgomery curve equation.
  - `generate_montgomery_curve_points() -> Vec<(i64, i64)>`: Iterates through all possible $(x, y)$ pairs within the field and collects those that satisfy the curve equation.

## Usage

To run this program, ensure you have Rust installed on your machine. You can compile and execute the program using the following commands:

```bash
cargo build --release
cargo run
