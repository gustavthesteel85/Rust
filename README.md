# RUSTd
## 環境構築
### オンライン実行環境
https://play.rust-lang.org/

```
fn main() {
	println!("Hello wirld!")
}
```
### ローカル実行環境
* インストーラー(rustup)を入手  
https://rustup.rs/

* 動作確認
```
// ディレクトリ作成
cargo new hello_world
cd hello
// 実行
cargo run
```

* gccをインストール  
sudo apt install gcc

* Rust Language Serverをインストール  
rustup component add rls rust-analysis rust-src

* cargo-editをインストール  
cargo install cargo-edit
* リリーストレインモデル  
rust default xxxxxx  
・nightly: 毎晩新しい修正が反映される開発用  
・beta: リグレッションテストと重大な不具合修正が行われる  
・stable: 安定版  

---
## 基本的な文法

**特徴**
- 2006年にグレイドン・ホアレ氏が開発
- C++の代わりをも指しているので、システムソフトウェア(OS、コンパイラ)向き
- OS~Webアプリまで幅広く実装できる
- 「Firefox」を開発しているMozillaが支援するオープンソースのプログラミング言語
- 「所有権」「借用」「ライフタイム」でメモリ管理をする
- 仮想マシンを使用しない(仮想マシン語に変換しない)。機械語に直接コンパイルされる。
- GNUやLLVMが環境に応じた機械語を生成する
- ガベージコレクションがないので早い
- ゼロコスト抽象化
  - 元々C++で追求されていた概念
  - 追加コストを払う必要なく抽象化することで実行速度やメモリ使用量を増加する
- タイプセーフ
- メモリセーフ
    - ボローチェッカーによるスレッドセーフの保証
- 最小のランタイム
- ターゲットはベアメタル
- 強力な型推論
  - コード記載中に型を明記することはほとんどない
- RustによるOS作成
  - https://os.phil-opp.com/ja/
  - https://osblog.stephenmarz.com/
- プログラミング言語
	- 命令型計算モデル
		- Javaやpythonなど
	- 関数型計算モデル
		- OCaml, Scala, Haskell
		- ラムダ計算
	- 論理型計算モデル
		- Prolog
		- 帰納的関数

**機能**

- 階層
    - クレート
    - モジュール
    - パス
- Cargo
    - Rustのbuild tool
- 静的型付け
	- 数値型
		- 不動小数点型はf32とf64のみ
	- &str
		- スライス(コレクションの一つ)
		- プリミティブ型
		- ポインタとlengthの2つでできている
		- リサイズ不可
		- ダブルクオートで描こうとデフォルトで&strになる。to_string()をつけるとString型になる
		- **スタック**メモリに保管される
	- String
		- 標準ライブラリが提供
		- **ヒープ**メモリ(アプリ動的に確保)にアロケートされる
		- リサイズ可
- 構造体
    - 
- タプル
    - 異なる型を収めることができる集合。関数から複数の型を返す時などに使える。
	```
	let mut t = (1, "2");
	t.0 = 2;
	t.1 = "3";
	```
- Enum
    -
- Trait
	- Javaのinterfaceのような機能
	- 他言語ではインターフェースとその実装先のオブジェクトの紐付けは実行時に行われるのに対し、Rustではコンパイル時に行われるので早い

