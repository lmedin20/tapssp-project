#[derive(Default, Debug, Clone)]
pub struct Running {
    pub count: u64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
}
impl Running {
    pub fn update(&mut self, x: f64) {
        if self.count == 0 {
            self.min = x;
            self.max = x;
        } else {
            if x < self.min { self.min = x; }
            if x > self.max { self.max = x; }
        }
        self.sum += x;
        self.count += 1;
    }
    pub fn mean(&self) -> f64 {
        if self.count == 0 { 0.0 } else { self.sum / self.count as f64 }
    }
}
pub fn full_stats(values: &[f64]) {
    if values.is_empty() {
        println!("No numeric data.");
        return;
    }
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let count = v.len();
    let sum: f64 = v.iter().sum();
    let mean = sum / count as f64;
    let median = v[count / 2];
    let p25 = v[(count as f64 * 0.25).floor() as usize];
    let p75 = v[(count as f64 * 0.75).floor() as usize];
    let variance = v.iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>() / count as f64;
    let stddev = variance.sqrt();
    println!("\n--- Full Statistics ---");
    println!("count: {}", count);
    println!("sum: {}", sum);
    println!("min: {}", v[0]);
    println!("max: {}", v[count - 1]);
    println!("mean: {}", mean);
    println!("median: {}", median);
    println!("Standard Deviation: {}", stddev);
    println!("Lower Quartile: {}", p25);
    println!("Upper Quartlie: {}", p75);
}



