fn main() {
    let mut s: String = String::new();

    let data: &str = "initial contents";

    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    s.push('l');

    println!("{} {}", s, s2);

    let s1 = String::from("Hello ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    println!("{} {}", s2, s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    let s = dbg!(format!("{s1}-{s2}-{s3}"));

    let c = &s[0..1];

    println!("{c}");

    for c in "3Зд".chars() {
        println!("{c}");
    }

    for c in "3Зд".bytes() {
        println!("{c}");
    }
}
