//! CLI での操作フローを定義するモジュール。
//!
//! ユーザーからの入力受け取り、[`Todo`] の生成、一覧表示までの
//! 一連の流れを担当します。
use crate::model::todo::Todo;

/// CLI アプリケーションの操作フローを実行する関数。
///
/// ユーザーに Todo の入力を繰り返し促し、`n` が入力されたら入力を終了して
/// 登録された Todo の一覧を表示します。
pub fn run() {
    let mut todos: Vec<Todo> = Vec::new();
    loop {
        let todo = input_todo();
        todos.push(todo);
        println!("タイトル:{}", todos.last().unwrap().title);
        println!("本文:{}", todos.last().unwrap().content);
        println!("続けますか？(y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "n" {
            break;
        }
    }
    println!("Todo一覧");
    for todo in todos {
        println!("タイトル:{}", todo.title);
        println!("本文:{}", todo.content);
    }
}
/// ユーザーが入力した内容を返す関数
fn input_todo() -> Todo {
    println!("タイトルを追加する。");
    let mut title = String::new();
    std::io::stdin().read_line(&mut title).unwrap();
    println!("本文を追加する。");
    let mut content = String::new();
    std::io::stdin().read_line(&mut content).unwrap();
    // 最後の式を return せずに返すのが Rust の慣用的な書き方。
    Todo {
        title: title.trim().to_string(),
        content: content.trim().to_string(),
    }
}
