fn main() {
    let input = String::from("hello worldh");
    println!("{}", first_uniq_char(input));
}

fn first_uniq_char(s: String) -> i32 {
    // check for duplicates using copy
    // for (i, c) in s.chars().enumerate() {
    //     let mut copy: Vec<char> = s.clone().chars().collect();
    //     copy.remove(i);
    //     if !copy.contains(&c) {
    //         return i as i32;
    //     }
    // }

    // -1

    // check for duplicates using iteration
    'outer: for (i, c_1) in s.chars().enumerate() {
        for (j, c_2) in s.chars().enumerate() {
            if i == j  { continue; }
            if c_1 == c_2 { continue 'outer; }
        }
        return i as i32;
    }

    -1
}
