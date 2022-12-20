#![allow(dead_code)]
pub(crate) fn this<F>(f: F)
where
    F: Fn(),
{
    let start = std::time::Instant::now();
    f();
    let duration = start.elapsed();

    println!("TOOK: {:?}", duration);
}
