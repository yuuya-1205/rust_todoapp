//! `tests/` ディレクトリに置く「結合テスト（integration test）」の学習用サンプル。
//!
//! ポイント：
//! - このファイルは `todoapp` 本体とは **別クレート** としてコンパイルされます。
//! - そのため、参照できるのは `pub` で公開されたAPIだけです。
//!   （`cli.rs` の `build_todo` などの非公開関数は、ここからは見えません。）
//! - 公開されている [`Todo`] 構造体を、外部利用者の視点で検証します。
//!
//! ユニットテスト（各 `.rs` ファイル内の `#[cfg(test)] mod tests`）との違い：
//! - ユニットテスト   … 同じクレート内なので非公開関数もテストできる。内部ロジック向け。
//! - 結合テスト(これ) … 別クレートから公開APIだけをテストする。外から見た振る舞い向け。

// ライブラリクレート `todoapp` の公開API（Todo 構造体）を読み込む。
use todoapp::model::todo::Todo;

/// 公開APIである Todo を外部クレートから生成し、フィールドを保持できることを確認する。
#[test]
fn 外部クレートからtodoを生成して値を保持できる() {
    let todo = Todo {
        title: String::from("買い物"),
        content: String::from("牛乳を買う"),
    };

    assert_eq!(todo.title, "買い物");
    assert_eq!(todo.content, "牛乳を買う");
}

/// derive した PartialEq により、同じ値の Todo 同士が等しいと判定されることを確認する。
#[test]
fn 同じ値のtodoは等しいと判定される() {
    let a = Todo {
        title: String::from("タイトル"),
        content: String::from("本文"),
    };
    let b = Todo {
        title: String::from("タイトル"),
        content: String::from("本文"),
    };

    assert_eq!(a, b);
}
