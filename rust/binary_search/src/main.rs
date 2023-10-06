use std::time::Instant;

// fn main() {
//     println!("{:?}", search(Vec::from([-1, 0, 3, 5, 9, 12, 13]), 3));
// }

// fn search(nums: Vec<i32>, target: i32) -> i32 {
//     if nums.len() == 0 {
//         println!("Number not found in list");
//         std::process::exit(1);
//     }
//     let pivot = ((nums.len() as f32 - 1f32) / 2f32).ceil() as usize;
//     if nums[pivot as usize] == target { return pivot as i32; }
//     if target < nums[pivot] {
//         return search(Vec::from(&nums[..pivot]), target);
//     }
//     return pivot as i32 + search(Vec::from(&nums[pivot..]), target);
// }

struct Solution {}
impl Solution {
    pub fn bin_search(nums: Vec<i32>, target: i32) -> i32 {                
        let pivot = ((nums.len() as f32 - 1f32) / 2f32).ceil() as usize;
        if nums[pivot as usize] == target { return pivot as i32; }
        if nums.len() == 1 { return -1 }
        if target < nums[pivot] { return Self::bin_search(Vec::from(&nums[..pivot]), target); }
        return match Self::bin_search(Vec::from(&nums[pivot..]), target) {
            -1 => -1,
            ret => pivot as i32 + ret
        }
    }
}
fn main() {
    let target = 3;
    let time = Instant::now();
    let result = Solution::bin_search(Vec::from([-2, 3, 5, 8, 10]), target);
    let elapsed = time.elapsed();
    println!("{} in {:?}",
        match result {
            -1 => "No results found".to_string(),
            index => format!("Target ({}) found at index {}", target, index)
        },
        elapsed
    );
}