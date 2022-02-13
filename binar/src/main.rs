
fn main() {
    let mut text = String::new();
        text.push_str("0 morning\n");
        text.push_str("1 Noon\n");
        text.push_str("2 evnin\n");
        text.push_str("3 night\n");
        text.push_str("4 Monday\n");
        text.push_str("5 Tuesday\n");
        text.push_str("#\n");
        text.push_str("0 1 lunch is coming soon\n");
        text.push_str("0 4 this day\n");
        text.push_str("1 2 go home\n");
        text.push_str("2 3 go to sleep\n");
        text.push_str("3 5 next day\n");
        text.push_str("4 5 next day\n");

    println!("{}", text);
}
