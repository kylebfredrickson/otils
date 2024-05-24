# Otils

Otils implements oblivious and constant time primitives. These primitives are helpful for developing applications that are resistant to side-channel attacks in trusted execution environments (TEEs), e.g., Intel SGX. Otils depends on a minimal base of primitive C operations. The constant time-ness of utilities in this repository reduce to these operations being constant time.

**Note**: This code has not undergone code review. Promises of constant time operations have **not** been verified.

## Features

### ObliviousOps
This trait enables users to define custom types that can be obliviously operated on. Types must implement only three functions obliviously to gain access to this functionality.

- `oselect({0, 1}: i8, a: Self, b: Self) -> Self` On input 1 will return the value of `a`. On input 0 will return `b`.

- `oequal(a: Self, b: Self) -> {0, 1}: i8` Tests equality. If `a == b -> 1` and `a != b -> 0`.

- `ocompare(a: Self, b: Self) -> {-1, 0, 1}: i8` Compares inputs. `a > b -> 1`, `a == b -> 0`, and `a < b -> -1`.


### Oblivious Sort
Slices of types implementing `ObliviousOps` can be obliviously sorted. Currently, the following oblivious sort algorithms are implemented:

#### [Bitonic Sort](https://en.wikipedia.org/wiki/Bitonic_sorter)
Benchmark sorts $2^{20}$ 64-bit integers. Tested in SGX.

- `cf3009c`
    - Time: (8 threads) 111,088,422 ns/iter (+/- 976,209)



### Oblivious Compaction
Slices of types implementing `ObliviousOps` can be obliviously compacted. Currently, the following oblivious compaction algorithms are implemented:

#### [ORCompact](https://dl.acm.org/doi/abs/10.1145/3548606.3560603)
Benchmark compacts $2^{20}$ 64-bit integers. Tested in SGX.

- `cf3009c`
    - Time: (8 threads) 28,814,889 ns/iter (+/- 1,630,245)
