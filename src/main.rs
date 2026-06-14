fn main() {
    println!("題名を入力してください");
    let mut title = String::new();
    std::io::stdin().read_line(&mut title).unwrap();

    println!("本文を入力してください");
    let mut content = String::new();
    std::io::stdin().read_line(&mut content).unwrap();

    println!("題名:{}", title);
    println!("本文:{}", content);
}
