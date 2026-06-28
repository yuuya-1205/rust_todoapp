//! CLI での操作フローを定義するモジュール。
//!
//! ユーザーからの入力受け取り、[`Todo`] の生成、一覧表示までの
//! 一連の流れを担当します。
//!
//! 入出力（stdin / stdout）に依存する部分と、純粋なロジック
//! （入力値の整形・継続判定・表示用の整形）を分けることで、
//! ロジック部分を単体テストできるようにしています。
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
        // 直前に追加した Todo を表示する。
        println!("{}", format_todo(todos.last().unwrap()));
        println!("続けますか？(y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if !should_continue(&input) {
            break;
        }
    }
    println!("Todo一覧");
    for todo in &todos {
        println!("{}", format_todo(todo));
    }
}

/// ユーザーが入力した内容を読み取り、[`Todo`] を生成して返す関数。
///
/// stdin からの読み取りを担当し、整形と生成は [`build_todo`] に委ねます。
fn input_todo() -> Todo {
    println!("タイトルを追加する。");
    let mut title = String::new();
    std::io::stdin().read_line(&mut title).unwrap();
    println!("本文を追加する。");
    let mut content = String::new();
    std::io::stdin().read_line(&mut content).unwrap();
    build_todo(&title, &content)
}

/// タイトルと本文の文字列から [`Todo`] を生成する関数。
///
/// 前後の空白や改行を取り除いてから [`Todo`] を組み立てます。
fn build_todo(title: &str, content: &str) -> Todo {
    Todo {
        title: title.trim().to_string(),
        content: content.trim().to_string(),
    }
}

/// 入力を継続するかどうかを判定する関数。
///
/// 前後の空白を取り除いた入力が `"n"` のときだけ終了（`false`）を返し、
/// それ以外は継続（`true`）を返します。
fn should_continue(input: &str) -> bool {
    input.trim() != "n"
}

/// 1件の [`Todo`] を表示用の文字列に整形する関数。
///
/// 「タイトル」と「本文」をそれぞれ 1 行ずつに整形して返します。
fn format_todo(todo: &Todo) -> String {
    format!("タイトル:{}\n本文:{}", todo.title, todo.content)
}

#[cfg(test)]
mod tests {
    use super::{build_todo, format_todo, should_continue};
    use crate::model::todo::Todo;

    /// build_todo は前後の空白・改行を取り除いて Todo を生成する。
    #[test]
    fn build_todoは空白と改行を取り除く() {
        let todo = build_todo("  買い物 \n", "牛乳を買う\n");

        assert_eq!(
            todo,
            Todo {
                title: String::from("買い物"),
                content: String::from("牛乳を買う"),
            }
        );
    }

    /// build_todo は空文字の入力でも空の Todo を生成する。
    #[test]
    fn build_todoは空入力でも生成できる() {
        let todo = build_todo("\n", "   ");

        assert_eq!(todo.title, "");
        assert_eq!(todo.content, "");
    }

    /// should_continue は "n" のときだけ false を返す。
    #[test]
    fn should_continueはnのときだけ終了する() {
        assert!(!should_continue("n"));
        assert!(!should_continue(" n \n"));
        assert!(should_continue("y"));
        assert!(should_continue("yes"));
        assert!(should_continue("\n"));
        assert!(should_continue("no"));
    }

    /// format_todo はタイトルと本文を2行に整形する。
    #[test]
    fn format_todoはタイトルと本文を整形する() {
        let todo = Todo {
            title: String::from("買い物"),
            content: String::from("牛乳を買う"),
        };

        assert_eq!(format_todo(&todo), "タイトル:買い物\n本文:牛乳を買う");
    }
}
