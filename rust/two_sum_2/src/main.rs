use std::cmp::Ordering::*;

fn main() {
    println!("{:?}", two_sum([-1, 0].into(), -1));
}

fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l = 0;
    let mut r = numbers.len() - 1;

    while l <= r {
        match (numbers[l] + numbers[r]).cmp(&target) {
            Less => l += 1,
            Greater => r -= 1,
            Equal => return vec![(l+1) as i32, (r+1) as i32],
        }
    }

    vec![-1, -1]
}
