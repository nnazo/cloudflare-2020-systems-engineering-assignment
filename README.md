# Systems Engineering Assignment 2020

## Build
You must have Rust and Cargo installed.
1. Install Rust and Cargo if necessary
2. Have OpenSSL installed, and refer to https://docs.rs/openssl/0.10.30/openssl/#manual for setting environment variables if your installation is not automatically detected.
3. Run `cargo build --release`
4. Execute `./target/release/cloudflare-2020-systems-engineering-assignment` with the desired options.

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

Reduced output for `./target/release/cloudflare-2020-systems-engineering-assignment --url https://github.com/nnazo --profile 100`
```
# of requests made: 100
fastest time: 195 ms
slowest time: 1178 ms
mean time: 267.17 ms
median time: 209 ms
success percentage: 39%
unsuccessful response codes: [429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429, 429]
smallest response: 1947 bytes
largest response: 174617 bytes
```

Reduced output for `./target/release/cloudflare-2020-systems-engineering-assignment --url https://duckduckgo.com --profile 100`
```
# of requests made: 100
fastest time: 168 ms
slowest time: 215 ms
mean time: 189.08 ms
median time: 193 ms
success percentage: 100%
unsuccessful response codes: []
smallest response: 6346 bytes
largest response: 7138 bytes
```

Full output for https://google.com
```
➜  cloudflare-2020-systems-engineering-assignment git:(main) ✗ ./target/release/cloudflare-2020-systems-engineering-assignment --url https://google.com --profile 100    
response:
HTTP/1.1 301 Moved Permanently
Location: https://www.google.com/
Content-Type: text/html; charset=UTF-8
Date: Sun, 11 Oct 2020 18:21:15 GMT
Expires: Tue, 10 Nov 2020 18:21:15 GMT
Cache-Control: public, max-age=2592000
Server: gws
Content-Length: 220
X-XSS-Protection: 0
X-Frame-Options: SAMEORIGIN
Alt-Svc: h3-Q050=":443"; ma=2592000,h3-29=":443"; ma=2592000,h3-27=":443"; ma=2592000,h3-T051=":443"; ma=2592000,h3-T050=":443"; ma=2592000,h3-Q046=":443"; ma=2592000,h3-Q043=":443"; ma=2592000,quic=":443"; ma=2592000; v="46,43"
Connection: close

<HTML><HEAD><meta http-equiv="content-type" content="text/html;charset=utf-8">
<TITLE>301 Moved</TITLE></HEAD><BODY>
<H1>301 Moved</H1>
The document has moved
<A HREF="https://www.google.com/">here</A>.
</BODY></HTML>


# of requests made: 100
fastest time: 86 ms
slowest time: 122 ms
mean time: 98.34 ms
median time: 96 ms
success percentage: 0%
unsuccessful response codes: [301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301, 301]
smallest response: 779 bytes
largest response: 779 bytes
```

Out of the requests that returned a semi-large HTML webpage, the worker site seemed the fastest for the given response size. The github request took much longer although the successful responses were much larger. Rust-lang had a similar benchmark time but the response was much smaler than the worker site since it was always gave a 301 response code.