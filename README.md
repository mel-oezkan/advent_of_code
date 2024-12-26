# advent_of_code

The following repo contains the solutions for the Advent of code 2024. This repo was created to only solve the basic problems of the current advent of code using a new programming language namely Rust.

Solutions are not optimal and are only supposed to make myself more familiar with Rust data structures and code flow.

## Solution Strategy

### Day 08

-   For any signal find all the possible positions
    -   O(n): Iterate over the matrix and get unique symbol types (#k) and position
-   Using the signal positions iteratively generate the new antinodes
    -   O(#k\*#n): crete the positions
