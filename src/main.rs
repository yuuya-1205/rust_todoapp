/// モジュール管理をしたおかげでmain.rsに書くコードを最小限に抑えることができる。
mod model;
mod presentation;

fn main() {
    presentation::cli::run();
}
