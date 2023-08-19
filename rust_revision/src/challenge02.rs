pub(crate) fn challenge() {
    println!("Temperature Converter");
    println!("Temperature:{}", converter(23.4))
}

fn converter(x: f64) -> f64 {
    x * 9.0 / 5.0 + 32.0
}