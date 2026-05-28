// Optimized solution using game theory reduction
pub fn winner_optimized(heaps: &[i64]) -> &'static str {

    let all_even = heaps.iter().all(|x| x % 2 == 0);

    if all_even {
        "second"
    } else {
        "first"
    }
}