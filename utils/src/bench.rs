use human_repr::{HumanCount, HumanDuration};
use serde::Serialize;
use serde_with::{serde_as, DurationNanoSeconds};
use std::{
    fmt::Display,
    fs::File,
    io::Write,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
use tabled::{settings::Style, Table, Tabled};

pub type BenchResult = (Duration, usize, usize);
type BenchMetrics = (Duration, usize, usize, usize);

pub fn benchmark<T: Display + Clone, F>(func: F, inputs: &[T], file: &str)
where
    F: Fn(T) -> BenchResult,
{
    let mut results = Vec::new();
    for input in inputs {
        let ((duration, size, cycles), peak_memory) = measure_peak_memory(|| func(input.clone()));
        results.push((duration, size, cycles, peak_memory));
    }

    write_csv(file, inputs, &results);
}

fn get_current_memory_usage() -> Result<usize, std::io::Error> {
    let content = std::fs::read_to_string("/proc/self/status")?;
    for line in content.lines() {
        if line.starts_with("VmRSS:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(kb) = parts[1].parse::<usize>() {
                    return Ok(kb * 1024); // kb to bytes
                }
            }
        }
    }
    Ok(0)
}

pub fn measure_peak_memory<R, F: FnOnce() -> R>(func: F) -> (R, usize) {
    let peak = Arc::new(AtomicUsize::new(0));
    let stop = Arc::new(AtomicBool::new(false));

    let peak_clone = Arc::clone(&peak);
    let stop_clone = Arc::clone(&stop);
    let monitor = thread::spawn(move || {
        while !stop_clone.load(Ordering::Relaxed) {
            if let Ok(mem) = get_current_memory_usage() {
                peak_clone.fetch_max(mem, Ordering::Relaxed);
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    let result = func();

    stop.store(true, Ordering::Relaxed);
    monitor.join().unwrap();

    (result, peak.load(Ordering::Relaxed))
}

pub fn write_csv<T: Display>(file: &str, inputs: &[T], results: &[BenchMetrics]) {
    let mut file = File::create(file).unwrap();
    file.write_all(
        format!("n,cycles,prover time (ms),proof size (bytes),peak memory (MB)\n").as_bytes(),
    )
    .unwrap();
    inputs
        .iter()
        .zip(results)
        .for_each(|(input, (duration, size, cycles, peak_memory))| {
            file.write_all(
                format!(
                    "{},{},{},{},{}\n",
                    input,
                    cycles,
                    duration.as_millis(),
                    size,
                    peak_memory
                )
                .as_bytes(),
            )
            .unwrap();
        });
}

#[serde_as]
#[derive(Serialize, Tabled)]
pub struct Metrics {
    pub size: usize,
    #[serde_as(as = "DurationNanoSeconds")]
    #[tabled(display_with = "display_duration")]
    pub exec_duration: Duration,
    #[serde_as(as = "DurationNanoSeconds")]
    #[tabled(display_with = "display_duration")]
    pub proof_duration: Duration,
    #[serde_as(as = "DurationNanoSeconds")]
    #[tabled(display_with = "display_duration")]
    pub verify_duration: Duration,
    #[tabled(display_with = "display_cycles")]
    pub cycles: u64,
    #[tabled(display_with = "display_bytes")]
    pub proof_bytes: usize,
    #[tabled(display_with = "display_bytes")]
    pub peak_memory: usize,
}

fn display_bytes(bytes: &usize) -> String {
    bytes.human_count_bytes().to_string()
}

fn display_cycles(cycles: &u64) -> String {
    cycles.human_count_bare().to_string()
}

fn display_duration(duration: &Duration) -> String {
    duration.human_duration().to_string()
}

impl Metrics {
    pub fn new(size: usize) -> Self {
        Metrics {
            size,
            exec_duration: Duration::default(),
            proof_duration: Duration::default(),
            verify_duration: Duration::default(),
            cycles: 0,
            proof_bytes: 0,
            peak_memory: 0,
        }
    }
}

pub fn benchmark_v2<T: Display + Clone, F>(func: F, inputs: &[T], file: &str)
where
    F: Fn(T) -> Metrics,
{
    let mut results = Vec::new();
    for input in inputs {
        let (mut metrics, peak_memory) = measure_peak_memory(|| func(input.clone()));
        metrics.peak_memory = peak_memory;
        results.push(metrics);
    }

    write_csv_v2(file, &results);
}

pub fn write_csv_v2(out_path: &str, results: &[Metrics]) {
    let mut out = csv::WriterBuilder::new().from_path(out_path).unwrap();

    let mut all_metrics = Vec::new();

    for metric in results {
        out.serialize(&metric).expect("Could not serialize");
        out.flush().expect("Could not flush");
        all_metrics.push(metric);
    }

    out.flush().expect("Could not flush");

    let mut table = Table::new(&all_metrics);
    table.with(Style::modern());
    println!("{table}");
}
