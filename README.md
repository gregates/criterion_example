Demo for measuring overhead in [criterion](https://github.com/bheisler/criterion.rs) benchmarks,
using different patterns to separate input generation from the function you want to measure.

```
small slice                 time:   [12.181 ns 12.192 ns 12.201 ns]
big slice                   time:   [4.0691 ms 4.0853 ms 4.1001 ms]
big slice returned          time:   [377.09 ns 381.50 ns 386.09 ns]
no closure                  time:   [4.2026 ms 4.2064 ms 4.2104 ms]
no closure & return input   time:   [368.68 ns 373.52 ns 378.48 ns]
black box                   time:   [42.327 ns 45.663 ns 49.377 ns]
black box no closure        time:   [37.270 ns 39.100 ns 41.183 ns]
```

measurements taken on EC2 r8g.2xlarge

See also [user guide section on timing
loops](https://bheisler.github.io/criterion.rs/book/user_guide/timing_loops.html).

