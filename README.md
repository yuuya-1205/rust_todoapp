# rust_todoapp

Rust 製の CLI Todo アプリケーションです。学習・教育を目的としたプロジェクトのため、
コードには手厚い日本語コメントが付いています。

## 機能

- ターミナル上で Todo（タイトルと本文）を追加できます。
- 「続けますか？(y/n)」の確認で、続けて入力するか終了するかを選べます。
- 終了すると、登録した Todo の一覧を表示します。

## 必要なもの

- Rust ツールチェイン（`cargo`）。`edition = "2024"` を使用しています。

## ビルド・実行・テスト

```sh
cargo build   # ビルド
cargo run     # 実行
cargo test    # テスト
cargo fmt     # フォーマット
cargo clippy  # Lint
```

## ディレクトリ構成

```txt
src/
  main.rs          # 起動地点。各レイヤーの入口を呼び出すだけに留める。
  model/           # データ構造（ドメイン）
    mod.rs
    todo.rs        # Todo 構造体の定義
  presentation/    # ユーザーとの接点（CLI）
    mod.rs
    cli.rs         # CLI の操作フロー
```

レイヤーごとにディレクトリを分け、各ディレクトリの入口を `mod.rs` にしています。
設計の詳細は `docs/architectured.md` を参照してください。

## ドキュメント

- `AGENTS.md` … AI エージェント向けの基本ルール
- `docs/requirements.md` … 要件
- `docs/architectured.md` … モジュール構成・設計
- `docs/coding-rules.md` … コーディングルール
- `docs/review-checklist.md` … レビュー観点
- `src/model/model.md` … モデルファイルの記述規約
- `src/presentation/presentation.md` … プレゼンテーション層の説明
