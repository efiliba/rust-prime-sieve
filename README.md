# rust-prime-sieve
Rust version of prime sieve algorithm from [Dave's Garage](https://www.youtube.com/channel/UCNzszbnvQeFzObW0ghk0Ckw) YouTube channel

Sample code from https://www.youtube.com/watch?v=D3h62rgewZM implemented in Rust.

## Code optimisation stages

### Results from the YouTube video that first stage (1: Base ) is based on
- Python   25
- C#     2651
- C++    4645
  
#### Run on MacBook 2018 - 2.2 GHz 6-Core Intel Core i7

| Stage    | Passes (debug) | Passes (optimised) | Comments                                            |
|   ---    |     :---:      |       :---:        |                         ---                         |
| 1: Base  |       62       |       4900         | Uses a full array 1 million and 1 for easy indexing |
