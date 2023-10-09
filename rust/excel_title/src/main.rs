fn main() {
    println!("{}", convert_to_title(27));
}

// sub 1, mod 26, int div 26, repeat, add 1
fn convert_to_title(column_number: i32) -> String {
    let mut rem = column_number - 1;
    let mut out = Vec::<char>::new();

    while rem > 26 {
        out.push(dbg!(((rem % 26) as u8 + (b'A' - 1)) as char));
        rem /= 26;
    }
    
    out.push((rem as u8 + (b'A' - 1)) as char);
    
    if out[0] == 'Z' {
		out[0] = 'A';
		out.insert(0, 'A');
	} else {
		out[0] = (out[0] as u8 + 1) as char;
	}

    out.reverse();

    out.iter().collect::<String>()
}


