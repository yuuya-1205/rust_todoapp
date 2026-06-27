/// モジュール管理をしたおかげでmain.rsに書くコードを最小限に抑えることができる。
mod model;
mod presentation;

fn main() {
    // presentation モジュールが公開している run 関数を呼び出す。
    // cli モジュールの内部構造を意識せずに済むよう、再エクスポートされた入口を使う。
    presentation::run();
}
