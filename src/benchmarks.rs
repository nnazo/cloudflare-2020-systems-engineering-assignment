use std::time::Instant;

/// This struct keeps track of our request benchmarking.
#[derive(Default, Debug)]
pub struct Benchmarks {
    pub durations: Vec<u128>,
    pub slowest: Option<u128>,
    pub fastest: Option<u128>,
    pub succeeded: u32,
    pub error_codes: Vec<u32>,
    pub smallest: Option<usize>,
    pub largest: usize,
}

impl Benchmarks {
    /// Updates our benchmark values as necessary given the start, end Instants and
    /// the response.
    pub fn update(&mut self, start: Instant, end: Instant, resp: String) -> Option<()> {
        let duration = end
            .checked_duration_since(start)
            .expect("End instant occurred before start instant");
        let ms = duration.as_millis();
        self.durations.push(ms);
        self.durations.sort();

        // Update the slowest request if necessary
        match &mut self.slowest {
            Some(slowest) => {
                *slowest = (*slowest).max(ms);
            },
            None => {
                self.slowest.replace(ms);
            }
        }

        // Update the fastest request if necessary
        match &mut self.fastest {
            Some(fastest) => {
                *fastest = (*fastest).min(ms);
            },
            None => {
                self.fastest.replace(ms);
            }
        }

        // Get the response body. If there was not one, return
        let body = resp.split("\r\n\r\n").last()?;
        
        // Update the largest response body if necessary
        self.largest = self.largest.max(body.len());

        // Update the smallest response body if necessary
        match &mut self.smallest {
            Some(smallest) => {
                 *smallest = (*smallest).min(body.len());
            },
            None => {
                self.smallest.replace(body.len());
            }
        }

        // Get the 2nd value in the iterator, which is the response code;
        let code = resp.split_ascii_whitespace().into_iter().nth(1)?;
        if let Ok(code) = code.parse::<u32>() {
            if code >= 400 {
                self.error_codes.push(code);
            } else {
                self.succeeded += 1;
            }
        }

        None
    }

    /// Computes the mean request duration
    pub fn mean(&self) -> f64 {
        let sum: u128 = self.durations.iter().sum();
        sum as f64 / self.durations.len() as f64
    }

    /// Computes the median request duration
    pub fn median(&self) -> u128 {
        match self.durations.get(self.durations.len() / 2) {
            Some(duration) => *duration,
            None => 0,
        }
    }

    /// Computes the success percentage of the requests
    pub fn success_percentage(&self) -> f64 {
        let num = self.durations.len();
        ((num - self.error_codes.len()) as f64 / num as f64) * 100 as f64
    }
}
