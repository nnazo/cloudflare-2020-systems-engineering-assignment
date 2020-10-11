# Systems Engineering Assignment 2020

## Build
You must have Rust and Cargo installed.
1. Install Rust and Cargo if necessary
2. Run `cargo build --release`
3. Execute `./target/release/cloudflare-2020-systems-engineering-assignment` with the desired options.

## Benchmarks
### Links Benchmark
![](screenshots/links-benchmark.png)

### Another Webpage Benchmark (rust-lang)
![](screenshots/rust-lang-benchmark.png)

### Worker Benchmark

Command run: `./target/release/cloudflare-2020-systems-engineering-assignment --url https://cloudflare-2020-general-engineering-assignment.jacobcurtis2266.workers.dev --profile 100`

![](screenshots/worker-benchmark.png)

### Other Site Benchmark Outputs
Note: omitting returned webpage content due to their length.

Output for https://github.com/nnazo
```
# of requests made: 100
fastest time: 192 ms
slowest time: 1026 ms
mean time: 278.46 ms
median time: 230 ms
success percentage: 49%
error codes: [429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429]
smallest response: 0 bytes
largest response: 1474 bytes
```

Output for https://duckduckgo.com
```
# of requests made: 100
fastest time: 166 ms
slowest time: 212 ms
mean time: 188.17 ms
median time: 187 ms
success percentage: 100%
error codes: []
smallest response: 5763 bytes
largest response: 5763 bytes
```

Output for https://google.com
```
# of requests made: 100
fastest time: 82 ms
slowest time: 177 ms
mean time: 96.93 ms
median time: 93 ms
success percentage: 100%
error codes: []
smallest response: 220 bytes
largest response: 220 bytes
```

Out of the requests that returned a semi-large HTML webpage, the worker site seemed the fastest for the given response size. The github request took much longer and returned a response half the size, though requests did get rate-limited in that benchmark. Google had a similar benchmark time but the response was much smaler than the worker site.