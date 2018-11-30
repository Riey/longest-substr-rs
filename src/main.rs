fn find_substr(input: &[u8]) -> &[u8] {
    println!("{}", ::std::str::from_utf8(input).unwrap());

    if input.len() < 1 {
        return input;
    }

    let mut infos = vec![None; 26];
    let mut cur = 0;

    while input.len() > cur {
        let index = (input[cur] - b'a') as usize;
        if let Some(prev) = infos[index] {
            let left = find_substr(&input[..cur]);
            let right = find_substr(&input[prev+1..]);

            if left.len() > right.len() {
                    return left;
            } else {
                    return right;
            }
        }

        infos[index] = Some(cur);

        cur += 1;
    }

    input
}

fn main() {
    println!("{}", ::std::str::from_utf8(find_substr(b"helloworld")).unwrap());
}
