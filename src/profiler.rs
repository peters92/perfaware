pub mod hp_profiler {
    use core::panic;
    use std::arch::x86_64::_rdtsc;
    use std::fmt::{self, Write};
    use std::time::{Duration, Instant};

    pub fn read_tsc() -> u64 {
        unsafe {
            _rdtsc()
        }
    }

    struct ProfilerNode {
        start: Instant,
        start_tsc: u64,
        time_spent: Duration,
        tsc_spent: u64,
        label: String,
        byte_count: usize,
    }
    impl ProfilerNode {
        fn build(start: Instant, label: &str) -> Self {
            ProfilerNode {
                start,
                start_tsc: read_tsc(),
                time_spent: Duration::from_micros(0),
                tsc_spent: read_tsc(), // this is later used for the increment
                label: label.to_owned(),
                byte_count: 0,
            }
        }

        fn increment(&mut self) {
            self.time_spent += self.start.elapsed();
            self.tsc_spent += read_tsc() - self.tsc_spent;
        }

        fn increment_byte_count(&mut self, byte_count: usize) {
            self.byte_count += byte_count;
        }
    }

    pub struct Profiler {
        nodes: Vec<ProfilerNode>,
    }

    impl Profiler {
        pub fn build() -> Self {
            let nodes: Vec<ProfilerNode> = Vec::with_capacity(8); // Initial capacity 8
            Profiler { nodes }
        }

        pub fn init_node(&mut self) {
            let node = ProfilerNode::build(Instant::now(), "");

            self.nodes.push(node)
        }

        pub fn log(&mut self, label: &str, byte_count: usize) {
            match self.nodes.last_mut() {
                Some(node) => {
                    if node.label == "".to_string() {
                        node.label = label.to_string()
                    }
                    node.increment();
                    node.increment_byte_count(byte_count);
                }
                None => panic!("No nodes found. You forgot to initialize the profiler."),
            }

            // Init new node in profiler
            self.init_node();
        }
    }

    impl fmt::Display for Profiler {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let total_time = self.nodes[0].start.elapsed();
            let total_cycles: u64 = read_tsc() - self.nodes[0].start_tsc;
            // Create string for storing printable results
            let mut result = String::new();
            write!(
                &mut result,
                "Profiler results:
                \rTotal time:   {:.2?} ({:.2}%)\
                \rTotal cycles:   {:.2?} ({:.2}%)",
                total_time, 100.,
                total_cycles, 100.
            )?;

            // For each ProfileNode gather information and calculate stats
            // then add to our result string
            for node in self.nodes.iter() {
                if node.label == "" {
                    continue;
                }
                let time_percent =
                    100. * (node.time_spent.as_nanos() as f64 / total_time.as_nanos() as f64);
                let megabyte_count = node.byte_count as f64 / (1024. * 1024.);
                let gigabyte_count = megabyte_count / 1024.;
                let time_spent_frac_seconds = node.time_spent.as_nanos() as f64 / 1000_000_000.;

                let throughput = gigabyte_count / time_spent_frac_seconds;
                write!(
                    &mut result,
                    "\nNode:         {}
                    \rTime spent:   {:.2?} ({:.2?}%)
                    \rByte count:   {:.2?}MB
                    \rThroughput:   {:.4?}GB/s",
                    node.label, node.time_spent, time_percent, megabyte_count, throughput
                )?
            }

            write!(f, "{}", result)
        }
    }
}

pub mod profiler {
    use core::panic;
    use std::fmt::{self, Write};
    use std::time::{Duration, Instant};

    struct ProfilerNode {
        start: Instant,
        time_spent: Duration,
        label: String,
        byte_count: usize,
    }
    impl ProfilerNode {
        fn build(start: Instant, label: &str) -> Self {
            ProfilerNode {
                start,
                time_spent: Duration::from_micros(0),
                label: label.to_owned(),
                byte_count: 0,
            }
        }

        fn increment(&mut self) {
            self.time_spent += self.start.elapsed();
        }

        fn increment_byte_count(&mut self, byte_count: usize) {
            self.byte_count += byte_count;
        }
    }

    pub struct Profiler {
        nodes: Vec<ProfilerNode>,
    }

    impl Profiler {
        pub fn build() -> Self {
            let nodes: Vec<ProfilerNode> = Vec::with_capacity(8); // Initial capacity 8
            Profiler { nodes }
        }

        pub fn init_node(&mut self) {
            let node = ProfilerNode::build(Instant::now(), "");

            self.nodes.push(node)
        }

        pub fn log(&mut self, label: &str, byte_count: usize) {
            match self.nodes.last_mut() {
                Some(node) => {
                    if node.label == "".to_string() {
                        node.label = label.to_string()
                    }
                    node.increment();
                    node.increment_byte_count(byte_count);
                }
                None => panic!("No nodes found. You forgot to initialize the profiler."),
            }

            // Init new node in profiler
            self.init_node();
        }
    }

    impl fmt::Display for Profiler {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let total_time = self.nodes[0].start.elapsed();
            // Create string for storing printable results
            let mut result = String::new();
            write!(
                &mut result,
                "Profiler results:
                \rTotal time:   {:.2?} ({:.2}%)",
                total_time, 100.
            )
            .unwrap();

            // For each ProfileNode gather information and calculate stats
            // then add to our result string
            for node in self.nodes.iter() {
                if node.label == "" {
                    continue;
                }
                let time_percent =
                    100. * (node.time_spent.as_nanos() as f64 / total_time.as_nanos() as f64);
                let megabyte_count = node.byte_count as f64 / (1024. * 1024.);
                let gigabyte_count = megabyte_count / 1024.;
                let time_spent_frac_seconds = node.time_spent.as_nanos() as f64 / 1000_000_000.;

                let throughput = gigabyte_count / time_spent_frac_seconds;
                write!(
                    &mut result,
                    "\nNode:         {}
                    \rTime spent:   {:.2?} ({:.2?}%)
                    \rByte count:   {:.2?}MB
                    \rThroughput:   {:.4?}GB/s",
                    node.label, node.time_spent, time_percent, megabyte_count, throughput
                )
                .unwrap()
            }

            write!(f, "{}", result)
        }
    }
}
