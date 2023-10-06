use std::cmp::Ordering;
use std::iter::FromIterator;

const BAD_VERSION: i32 = 1702766719;

fn main() {
    println!("{}", fbv(2126753390));
}

fn is_bad_version(v: i32) -> bool {
    v >= BAD_VERSION
}

fn fbv(n: i32) -> i32 {
    let mut l = 1;
    let mut r = n;

    loop { // loop is ok since there is a guaranteed result
        let mid = dbg!(r - ((r - l) / 2));
        if is_bad_version(mid) {
            if !is_bad_version(mid - 1) {
                return mid;
            }
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }
}