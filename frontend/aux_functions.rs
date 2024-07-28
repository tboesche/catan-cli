pub fn linspace(start: f64, end: f64, num: usize) -> Vec<f64> {
    if num == 0 {
        return Vec::new();
    }
    if num == 1 {
        return vec![start];
    }

    let step = (end - start) / (num - 1) as f64;
    (0..num).map(|i| start + i as f64 * step).collect()
}