## モジュール構成

このプロジェクトでは、ディレクトリー単位のモジュール入口として `mod.rs` を使用します。

`mod.rs` は、そのディレクトリー配下のモジュールを Rust に認識させるための入口ファイルです。

例えば、次のような構成にします。

```txt
src/
  main.rs
  model/
    mod.rs
    todo.rs
  presentation/
    mod.rs
    cli.rs
```

### `main.rs`

`main.rs` はアプリケーションの起動地点です。

このプロジェクトでは、`main.rs` に詳細な処理を書きすぎず、各レイヤーの入口を読み込んで実行します。

```rust
mod model;
mod presentation;

fn main() {
    presentation::cli::run();
}
```

`mod model;` は `src/model/mod.rs` を読み込みます。
`mod presentation;` は `src/presentation/mod.rs` を読み込みます。

### `model/mod.rs`

`model/mod.rs` は `model` ディレクトリーの入口ファイルです。

```rust
pub mod todo;
```

これは、`model` モジュールの中に `todo` モジュールがあることを表します。

この宣言により、他のファイルから次のように参照できます。

```rust
use crate::model::todo::Todo;
```

### `model/todo.rs`

`model/todo.rs` は Todo のデータ構造を定義するファイルです。

```rust
pub struct Todo {
    pub title: String,
    pub content: String,
}
```

`Todo` は、Todo アプリで扱う 1 件のタスクを表します。

### `presentation/mod.rs`

`presentation/mod.rs` は `presentation` ディレクトリーの入口ファイルです。

```rust
pub mod cli;
```

これは、`presentation` モジュールの中に `cli` モジュールがあることを表します。

この宣言により、`main.rs` から次のように呼び出せます。

```rust
presentation::cli::run();
```

### `presentation/cli.rs`

`presentation/cli.rs` は CLI での操作フローを定義するファイルです。

CLI は Command Line Interface の略で、ターミナル上でユーザーとやり取りするための画面・操作部分を表します。

このファイルには、以下のような処理を書きます。

- ユーザーに入力を促す。
- 入力された値を受け取る。
- Todo を一覧表示する。
- アプリケーションの操作フローを管理する。

### `mod` と `pub mod` の違い

`mod` は、モジュールを読み込むための宣言です。

```rust
mod model;
```

これは、`model` モジュールをこの crate に読み込むという意味です。

一方で、`pub mod` は、モジュールを読み込んだうえで外部にも公開します。

```rust
pub mod todo;
```

これは、`todo` モジュールを `model` の外からも使えるようにするという意味です。

### この構成にする理由

この構成にすると、`main.rs` を薄く保てます。

`main.rs` はアプリケーションの起動だけを担当し、Todo のデータ構造や CLI の操作フローは別ファイルに分けます。

これにより、次のような利点があります。

- ファイルごとの責務が分かりやすくなる。
- コードが増えても見通しが悪くなりにくい。
- `model` や `presentation` など、レイヤー単位で整理できる。
- 将来的に `favorite.rs` などの新しいモジュールを追加しやすい。

### 新しいモジュールを追加する例

例えば、`model` に `favorite.rs` を追加する場合は、次のような構成にします。

```txt
src/
  model/
    mod.rs
    todo.rs
    favorite.rs
```

そのうえで、`model/mod.rs` に次を追加します。

```rust
pub mod favorite;
```

これで、他のファイルから次のように使えるようになります。

```rust
use crate::model::favorite::Favorite;
```

### ルール

このプロジェクトでは、ディレクトリーをモジュールとして扱う場合、入口ファイルとして `mod.rs` を置きます。

```txt
src/<module_name>/mod.rs
```

`mod.rs` には、そのディレクトリー配下で公開するモジュールを `pub mod` で宣言します。
