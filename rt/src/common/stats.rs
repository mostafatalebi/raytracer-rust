use std::time::{Duration, Instant};

#[derive(Default)]
pub struct Stats {
    start_time: Option<Instant>,
    end_time: Option<Instant>,
    duration: Duration,
    rays_count_emitted: usize,
    num_of_threads: usize,
}


impl Stats {
    pub fn new(duration: Duration, rays_count_emitted: usize, num_of_threads: usize) -> Self {
        Self {
            start_time: None,
            end_time: None,
            duration, rays_count_emitted, num_of_threads,
        }
    }

    pub fn record_start_time(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn record_end_time(&mut self) {
        self.end_time = Some(Instant::now());
        self.duration = self.end_time.unwrap() - self.start_time.unwrap()
    }


    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    pub fn set_rays_count_emitted(&mut self, rays_count_emitted: usize) {
        self.rays_count_emitted = rays_count_emitted;
    }

    pub fn set_num_of_threads(&mut self, num_of_threads: usize) {
        self.num_of_threads = num_of_threads;
    }

    pub fn print_stats(&self) {
        let mut output = String::new();
        output.push_str("\n\nrender finished\n");
        output.push_str(format!("duration: {:?}\n", self.duration).as_str());
        output.push_str(format!("rays_count_emitted: {}\n", self.rays_count_emitted).as_str());
        output.push_str(format!("num of threads: {}\n", self.num_of_threads).as_str());
        print!("{}", output);
    }
}