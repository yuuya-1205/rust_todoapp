//! `model::todo::Todo` の公開APIに対する「結合テスト（integration test）」。
//!
//! ポイント：
//! - このファイルは `todoapp` 本体とは **別クレート** としてコンパイルされます。
//! - そのため、参照できるのは `pub` で公開されたAPIだけです。
//! - 既存の [`todo_integration_test`] と同様に、外部利用者の視点で
//!   [`Todo`] 構造体の振る舞いを検証します。

// ライブラリクレート `todoapp` の公開API（Todo 構造体）を読み込む。
use todoapp::model::todo::Todo;

/// 複数件の Todo を Vec にまとめ、件数と各要素の内容を保持できることを確認する。
#[test]
fn 複数のtodoをvecに集めて内容を検証できる() {
    let todos = vec![
        Todo {
            title: String::from("買い物"),
            content: String::from("牛乳を買う"),
        },
        Todo {
            title: String::from("掃除"),
            content: String::from("部屋を片付ける"),
        },
    ];

    assert_eq!(todos.len(), 2);
    assert_eq!(todos[0].title, "買い物");
    assert_eq!(todos[1].content, "部屋を片付ける");
}

/// タイトル・本文が空文字の Todo も生成でき、空文字を保持できることを確認する。
#[test]
fn 空文字のタイトルと本文を持つtodoを生成できる() {
    let todo = Todo {
        title: String::new(),
        content: String::new(),
    };

    assert_eq!(todo.title, "");
    assert_eq!(todo.content, "");
}

/// derive した PartialEq により、値が異なる Todo 同士は等しくないと判定されることを確認する。
#[test]
fn 値が異なるtodoは等しくないと判定される() {
    let a = Todo {
        title: String::from("タイトル"),
        content: String::from("本文"),
    };
    let b = Todo {
        title: String::from("タイトル"),
        content: String::from("別の本文"),
    };

    assert_ne!(a, b);
}
