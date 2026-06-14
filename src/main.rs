mod model;
use model::todo::Todo;
fn main() {
    println!("題名を入力してください");
    let mut title = String::new();
    std::io::stdin().read_line(&mut title).unwrap();
    println!("本文を入力してください");
    let mut content = String::new();
    std::io::stdin().read_line(&mut content).unwrap();

    let todo = Todo {
        title: title.trim().to_string(),
        content: content.trim().to_string(),
    };
    println!("題名:{}", todo.title);
    println!("本文:{}", todo.content);
}

// print! は改行しない println! は改行する
// そのため、止まっているように見えるんだよね。

//今回のアプリはread_lineを用いているため、改行が入ってしまう。
// そのため、trim()を用いて改行を削除する。
