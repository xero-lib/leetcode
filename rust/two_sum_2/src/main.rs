fn main() {
    println!("{:?}", two_sum([2, 3, 4].into(), 6));
}

fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    // iterate over each element until current + smallest >= target
    let mut idx = 0_usize;
    while numbers[idx] - numbers[0] < target {
        // binary search for largest number without exceeding target
        let Ok(result_idx) = numbers.binary_search(&(target - numbers[idx])) else { 
            idx += 1;
            continue;
        };
        
        return vec![(idx + 1) as i32, (result_idx + 1) as i32];
    }

    vec![]
}