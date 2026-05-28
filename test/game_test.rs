use another_game::naive::winner_naive;
use another_game::optimized::winner_optimized;

#[test]
fn test_sample_cases() {
    assert_eq!(winner_optimized(&[1, 2, 3]), "first");
    assert_eq!(winner_optimized(&[2, 2, 2]), "second");
    assert_eq!(winner_optimized(&[5, 5, 4, 5]), "first");
}

#[test]
fn test_all_even_heaps() {
    assert_eq!(winner_optimized(&[2, 4, 6, 8]), "second");
}

#[test]
fn test_contains_odd_heap() {
    assert_eq!(winner_optimized(&[2, 4, 7, 8]), "first");
}

#[test]
fn test_single_heap() {
    assert_eq!(winner_optimized(&[1]), "first");
    assert_eq!(winner_optimized(&[2]), "second");
}

#[test]
fn compare_naive_and_optimized() {
    let test_cases = vec![
        vec![1, 2, 3],
        vec![2, 2],
        vec![5, 5, 4, 5],
        vec![10, 12, 14],
        vec![7, 8, 9],
    ];

    for heaps in test_cases {
        assert_eq!(winner_naive(&heaps), winner_optimized(&heaps));
    };
}