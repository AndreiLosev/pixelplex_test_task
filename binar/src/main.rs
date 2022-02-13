
fn main() {
    let x = String::from("112 Доброе утроа");

    let mut key = String::new();
    let mut space_position = 0 as usize;

    for (i, char) in x.chars().enumerate() {
        if char == ' ' {
            space_position = i + 1;
            break;
        }

        key.push(char);
    }

    let line = &x[space_position..];

    dbg!(key, space_position, line);
}
