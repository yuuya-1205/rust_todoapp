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
/// // doctest は別クレートとして実行されるため、公開API を import する。
/// use todoapp::model::todo::Todo;
///
/// let todo = Todo {
///     title: String::from("タイトル"),
///     content: String::from("本文"),
/// };
/// assert_eq!(todo.title, "タイトル");
/// ```
#[derive(Debug, PartialEq)]
pub struct Todo {
    /// タスクのタイトル
    pub title: String,
    /// タスクの本文
    pub content: String,
}

impl Todo {
    /// タイトルと本文を受け取り、新しい [`Todo`] を生成して返すコンストラクタ。
    ///
    /// `title` と `content` は [`String`] へ変換できる値（`&str` や `String` など）を
    /// 受け取れます。
    ///
    /// # Examples
    ///
    /// ```
    /// // doctest は別クレートとして実行されるため、公開API を import する。
    /// use todoapp::model::todo::Todo;
    ///
    /// let todo = Todo::new("タイトル", "本文");
    /// assert_eq!(todo.title, "タイトル");
    /// assert_eq!(todo.content, "本文");
    /// ```
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Todo {
        Todo {
            title: title.into(),
            content: content.into(),
        }
    }
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

    /// Todo::new で生成したとき、タイトルと本文が保持されることを確認する。
    #[test]
    fn newで生成するとタイトルと本文を保持する() {
        let todo = Todo::new("買い物", "牛乳を買う");

        assert_eq!(todo.title, "買い物");
        assert_eq!(todo.content, "牛乳を買う");
    }

    /// Todo::new に String を渡しても正しく生成できることを確認する。
    #[test]
    fn newはstringを受け取れる() {
        let todo = Todo::new(String::from("掃除"), String::from("床を拭く"));

        assert_eq!(todo, Todo::new("掃除", "床を拭く"));
    }
}
