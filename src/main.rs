use structopt::StructOpt;
use url::Url;
use openssl::ssl::{
    SslMethod, SslConnector,
};
use std::{
    io::prelude::*,
    time::{Duration, Instant},
    net::TcpStream,
};
use anyhow::{Result, anyhow};

mod benchmarks;
use benchmarks::Benchmarks;

/// Parses the given command line options. 
/// The --url option is required.
/// If --profile is not given, it defaults to 1.
#[derive(StructOpt, Debug)]
#[structopt(name = "cloudflare-2020-systems-engineering-assignment")]
struct Options {
    #[structopt(long)]
    url: Url,
    
    #[structopt(default_value = "1", long)]
    profile: usize,
}

fn main() -> Result<()> {
    // Validate provided command line arguments
    let options = Options::from_args();
    let url = options.url;
    let profile = options.profile;
    if profile == 0 {
        return Err(anyhow!("--profile value must be at least 1"));
    }
    let addrs = url.socket_addrs(|| None)?;
    let host = url.host_str().expect("No host provided");
    let addr = addrs.into_iter().next().expect("No socket address was present");

    // Begin making requests for the number of times specified
    let mut bench = Benchmarks::default();
    for (i, _n) in (0..profile).enumerate() {
        // Initialize our response buffer, and start measuring the request time 
        let mut buf = vec![];
        let start = Instant::now();

        // Connect with the regular TcpStream
        let stream = TcpStream::connect_timeout(&addr, Duration::from_secs(5))?;
        stream.set_read_timeout(Some(Duration::from_secs(5)))?;

        // Connect with OpenSSL
        let connector = SslConnector::builder(SslMethod::tls())?.build();
        let mut stream = connector.connect(host, stream)?;

        let req = format!("GET {} HTTP/1.1\r\nAccept: */*\r\nConnection: close\r\nHost: {}\r\n\r\n", url.path(), host);

        // Send the request and read the response.
        stream.write_all(&req.into_bytes())?;
        stream.read_to_end(&mut buf)?;

        // Get the end time
        let end = Instant::now();

        // Print the response if it is the first iteration
        let resp = String::from_utf8_lossy(&buf).into_owned();
        if i < 1 {
            println!("response:\n{}", resp);
        }

        // Do benchmarks
        bench.update(start, end, resp);
    }

    // Print benchmark results
    println!("");
    println!("# of requests made: {}", bench.durations.len());
    println!("fastest time: {} ms", bench.fastest.unwrap_or_default());
    println!("slowest time: {} ms", bench.slowest.unwrap_or_default());
    println!("mean time: {} ms", bench.mean());
    println!("median time: {} ms", bench.median());
    println!("success percentage: {}%", bench.success_percentage());
    println!("error codes: {:?}", bench.error_codes);
    println!("smallest response: {} bytes", bench.smallest.unwrap_or_default());
    println!("largest response: {} bytes", bench.largest);

    Ok(())
}
