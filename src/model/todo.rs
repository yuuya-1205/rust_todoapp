//! Todoのデータ構造を定義するモジュール //ここに「このモジュールは何か？」を書く
//!
//! このモジュールはTodoアプリで扱う1件のタスクを表す
//! [`Todo`] 構造体を提供します。

/// 1件のToDoタスクを現す構造体
///
/// タイトルと本文を持ち、ユーザーが追加したタスクに対応します。
///
/// # Examples
///
/// ```
/// let todo = Todo {
///     title: String::from("タイトル"),
///     content: String::from("本文"),
/// };
/// assert_eq!(todo.title, "Rustを学ぶ");
/// ```
pub struct Todo {
    /// タスクのタイトル
    pub title: String,
    /// タスクの本文
    pub content: String,
}
