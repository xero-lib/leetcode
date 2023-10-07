fn main() {
    println!("{:?}", intersect(vec![1, 1, 1, 1, 1, 2], vec![1, 1, 1, 2, 2, 2]));
}

fn intersect(nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    // could be improved
    let mut out: Vec<i32> = Vec::new();

    for i in nums1.iter() {
        for (idx, j) in nums2.iter().enumerate() {
            if *i == *j {
                out.push(nums2.remove(idx));
                break;
            }
        }
    }

    out
}
