fn main() {
    let content = include_bytes!("input.txt")
                .windows(14)
                .position(|i| {
                    let s = std::str::from_utf8(i).unwrap();
                    unique(s).is_none()
                }).unwrap();
    println!("{}", content + 14);
}


fn unique(s: &str) -> Option<(usize, usize, char)> {
    s.chars().enumerate().find_map(|(i, c)| {
        s.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    })
}
