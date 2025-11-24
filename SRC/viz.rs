use std::cmp::max;
pub fn draw_histogram(data: &[f64], bins: usize) {
    if data.is_empty() {
        println!("No data to visualize.");
        return;
    }
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max_val - min;
    if range <= 0.0 {
        println!("All values are equal.");
        return;
    }
    let bin_width = range / bins as f64;
    let mut counts = vec![0; bins];
    for &x in data {
        let idx = ((x - min) / bin_width).floor() as usize;
        let idx = if idx >= bins { bins - 1 } else { idx };
        counts[idx] += 1;
    }
    let max_count = *counts.iter().max().unwrap_or(&1);
    println!("\nHistogram:");
    for (i, &count) in counts.iter().enumerate() {
        let bar = "#".repeat(max(1, (count * 40) / max_count));
        let lower = min + i as f64 * bin_width;
        let upper = lower + bin_width;
        println!("{:>6.2} - {:>6.2} | {}", lower, upper, bar);
    }
}

