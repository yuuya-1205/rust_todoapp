// このcrate::presentation::cli.rsの説明がわからない。
use crate::model::todo::Todo;

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
    return Todo {
        title: title.trim().to_string(),
        content: content.trim().to_string(),
    };
}
