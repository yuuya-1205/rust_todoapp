//! Todoのデータ構造を定義するモジュール //ここに「このモジュールは何か？」を書く
//!
//! このモジュールはTodoアプリで扱う1件のタスクを表す
//! [`Todo`] 構造体を提供します。

/// 1件のToDoタスクを表す構造体
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
/// assert_eq!(todo.title, "タイトル");
/// ```
pub struct Todo {
    /// タスクのタイトル
    pub title: String,
    /// タスクの本文
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::Todo;

    /// Todo を生成したとき、タイトルと本文が保持されることを確認する。
    #[test]
    fn todoを生成するとタイトルと本文を保持する() {
        let todo = Todo {
            title: String::from("買い物"),
            content: String::from("牛乳を買う"),
        };

        assert_eq!(todo.title, "買い物");
        assert_eq!(todo.content, "牛乳を買う");
    }
}
