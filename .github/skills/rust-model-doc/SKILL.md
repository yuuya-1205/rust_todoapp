---
name: rust-model-doc
description: Enforce the project's Rust model file documentation convention (module-level `//!` comment, struct-level and field-level `///` doc comments in Japanese) when creating, editing, or reviewing model struct files such as src/model/*.rs. Use whenever you add or change a data-structure module (a `pub struct` that represents a domain entity) in this Rust todo app, or when the user asks to follow src/model/model.md. このスキルは src/model 配下の Rust 構造体ファイルを model.md の規約どおりに書く／レビューするためのものです。
user-invocable: true
---

# Rust Model Doc Convention

`src/model/model.md` で定義された Rust モデルファイルの記述規約を**必ず守る**ためのスキルです。
`src/model/*.rs` のような「ドメインのデータ構造（`pub struct`）を定義するモジュール」を作成・編集・レビューするときに使います。

> 正本（source of truth）は常にリポジトリ内の `src/model/model.md` です。読める場合はまずそれを読み、本スキルの内容と食い違うときは `model.md` を優先してください。

## いつ使うか

- `src/model/` に新しいモデルファイル（例: `todo.rs`, `user.rs`）を追加するとき
- 既存のモデル構造体やフィールドを追加・変更するとき
- モデルファイルが規約に沿っているかレビューするとき

## 守るべき規約（チェックリスト）

モデルファイルが完成・修正・レビューされたとき、**すべて**満たしていること:

1. **モジュールコメント（`//!`）がファイル先頭にある**
   - ファイルの一番上に `//!` でモジュール全体の役割を書く。
   - 構造体へのリンク `[`<構造体名>`]` を本文に含める。
2. **構造体コメント（`///`）がある**
   - `pub struct` の直前に `///` で「1件の何を表す構造体か」を1行で書く。
   - 続けて空行 + 補足説明（何の情報を持つか）を書く。
3. **各フィールドにフィールドコメント（`///`）がある**
   - すべての `pub` フィールドの直前に `///` でそのフィールドの意味を書く。
   - コメントのないフィールドを残さない。
4. **日本語で書く**
   - コメント本文はプロジェクトの慣習に合わせて日本語で書く。
5. **誤字・表記を整える**
   - 「現す」→「表す」、「ToDo/Todo」の表記ゆれなど、明らかな誤字は直す。

## テンプレート（完成形）

```rust
//! <構造体名>のデータ構造を定義するモジュール。
//!
//! このモジュールは<アプリ名>で扱う<役割>を表す
//! [`<構造体名>`] 構造体を提供します。

/// 1件の<役割>を表す構造体。
///
/// <構造体名>として扱うための情報を持ちます。
pub struct <構造体名> {
    /// <フィールド1>を表します。
    pub <field_1>: String,
    /// <フィールド2>を表します。
    pub <field_2>: String,
}
```

`//!`（インナーモジュールドキュメント）と `///`（アイテムドキュメント）の使い分け:

- `//!` … ファイル/モジュール**全体**が何かを説明する。先頭にのみ書く。
- `///` … 直後の構造体・フィールドなど**特定の要素**を説明する。

## 実例（このリポジトリの `src/model/todo.rs`）

```rust
//! Todoのデータ構造を定義するモジュール。
//!
//! このモジュールはTodoアプリで扱う1件のタスクを表す
//! [`Todo`] 構造体を提供します。

/// 1件のToDoタスクを表す構造体。
///
/// タイトルと本文を持ち、ユーザーが追加したタスクに対応します。
pub struct Todo {
    /// タスクのタイトル
    pub title: String,
    /// タスクの本文
    pub content: String,
}
```

## 作業手順

1. **読む**: 可能なら `src/model/model.md` を読み、最新の規約を確認する。
2. **適用 / 確認**: 対象ファイルが上のチェックリストをすべて満たすように作成・修正する。
3. **検証**: 変更後に `cargo build`（あれば `cargo doc`）を実行し、ドキュメントコメントが壊れていないか確認する。
4. **報告**: レビュー時は、満たしていない項目をチェックリストの番号で具体的に指摘し、修正案を提示する。

## やってはいけないこと

- フィールドや構造体のドキュメントコメントを省略する。
- `//!` と `///` を取り違える（フィールド説明に `//!` を使うなど）。
- コメントを英語に勝手に翻訳する（プロジェクトは日本語コメント）。
- `model.md` の規約と矛盾する独自フォーマットを導入する。
