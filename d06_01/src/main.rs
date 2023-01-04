use peekmore::PeekMore;

fn main() {
    let content = include_str!("input.txt");
    let chars = content.chars();
    let mut iter = chars.into_iter().peekmore();
    for counter in 0..(content.len() - 4) {
        let a = iter.next().unwrap().to_string();
        let b = iter.peek_nth(0).unwrap().to_string();
        let c = iter.peek_nth(1).unwrap().to_string();
        let d = iter.peek_nth(2).unwrap().to_string();
        if a != b && a != c && a != d && b != c && b != d && c != d {
            println!("{}", counter+4);
            break;
        }
    }
}
