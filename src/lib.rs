pub mod naive;
pub mod optimized;

pub fn winner(heaps: &[i64]) -> &'static str {
    let all_even = heaps.iter().all(|x| x % 2 == 0);

    if all_even {
        "second"
    } else {
        "first"
    }
}