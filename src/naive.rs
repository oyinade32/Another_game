use std::collections::HashMap;


pub fn winner_naive(heaps: &[i64]) -> &'static str {
    let mut memo = HashMap::new();

    if can_win(heaps.to_vec(), &mut memo) {
        "first"
    } else {
        "second"
    }
}

fn can_win(state: Vec<i64>, memo: &mut HashMap<Vec<i64>, bool>) -> bool {
    
    let mut current: Vec<i64> = state.into_iter().filter(|&x| x > 0).collect();

    
    current.sort();


    if current.is_empty() {
        return false;
    }

    
    if let Some(&result) = memo.get(&current) {
        return result;
    }

    let n = current.len();

    
    for mask in 1..(1 << n) {
        let mut next = current.clone();

        for i in 0..n {
            if (mask & (1 << i)) != 0 {
                next[i] -= 1;
            }
        }

        if !can_win(next, memo) {
            memo.insert(current.clone(), true);
            return true;
        }
    }

    memo.insert(current.clone(), false);
    false
}