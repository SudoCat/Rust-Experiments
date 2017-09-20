use std::collections::HashMap;

fn main() {
    // Vec
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    
    let third: &i32 = &v[2];
    let fourth: Option<&i32> = v.get(2);

    // Strings
    let mut s = String::new();

    let data = "initial contents";

    let s2 = data.to_string();

    // the method also works on a literal directly:
    let s3 = "initial contents".to_string();

    let s4 = String::from("initial contents");

    let mut s5 = String::from("foo");
    s5.push_str("bar");

    let mut s6 = String::from("foo");
    let s7 = String::from("bar");
    s6.push_str(&s7);

    let mut s8 = String::from("lo");
    s8.push('l');

    // hashmaps
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
