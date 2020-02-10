---
marp: true
theme: gaia
paginate: true
---

# (Rubist向け)Rust勉強会資料

## 目的

~~みんなこの沼に落ちろ~~
~~Rust強くなって私に教えて~~
Rustの独特な部分をざくっと説明して、入門コストを下げるぞー

### 発表者

@yusuke_ota

---

## 参考文献

* プログラミング言語 Rust, 2nd Edition 日本語訳
    <https://doc.rust-jp.rs/book/second-edition/>
    注: 2018年6月ごろの英語版(Rust2015版)ベースのため内容が少し古い

* 実践Rust入門  
    一言でいうと、たのしいRubyみたいな本
    プログラミング言語 Rust, 2nd Editionの次に読むと良い

---

## 目次

* Rustの特徴(3分)
* 変数(12分)
  <!-- - ミュータブル?イミュータブル?
  - シャドーイング
  - 演習 -->
* 型
  <!-- - 一覧(固定長？可変長？)
  - メモリ上のふるまい
  - オリジナルな型 -->

<!-- --- -->

<!-- ## 目次 2 -->

* 所有権
  <!-- - 大まかな考え方
  - 所有権
  - 参照
  - 参照(mut) -->
* 基本構文

<!-- --- -->

<!-- ## 目次 3 -->

* ライフタイム
  <!-- - ≒スコープ
  - ライフタイム注釈 -->
* エラー処理
  <!-- - panic!
  - Result型、Option型
  - パターンマッチ -->

* 構造体(struct)

---

## Rustの特徴 1

### メリット

* GCがないから、速い
* GCがないから、OSが無くても動く
* データ競合がないから非同期処理が安心して書ける
* FFIでRubyから簡単に呼べる

---

## Rustの特徴 2

### デメリット

* 所有権って何?(独自の概念)
* クラスのがない(structにメソッドを追加していく)
* (オブジェクト指向の人は) 関数型
* async/awaitの書き方が独特

---

## 変数

### 変数 結論

Rustの変数は基本再代入不可
再代入するには2つの方法がある

1. 可変変数として宣言する
2. 同じ名前の別の変数を新規作成する

---

### 変数 in Rust

Rustでは変数のことを
**変数束縛**と呼ぶ

---

### ミュータブル？イミュータブル？

Rustの変数は以下の様に宣言
しかし、コメントアウト部分でエラーがおきる

```rust
fn main(){
    let value = "変数1";

    // コンパイルエラー
    // value = "変数2";
}
```

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=9038c11efd6b7cf7060d32b1ce149fc2)

---

### なぜなのか？

Rustの変数は基本**イミュータブル(変更不可)**

---

### どうしたらよいか？

1. ミュータブル(変更可能)な変数として宣言する
2. 再定義する(シャドーイング)

---

#### 1. ミュータブル(変更可能)な変数として宣言する

```rust
fn main(){
    let mut value = "変数1";
    value = "変数2";

    // コンパイルエラー
    // "変数2"は&str型、2.0はf64型で型が違う
    // value = 2.0_f64;
}
```

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=31a5fc47dd3ff9252bc517c371518b9e)

---

#### 2. 再定義する(シャドーイング)

```rust
fn main(){
    let value = "変数1";
    let value = "変数2";

    // OK
    // valueをf64型として新規作成
    let value = 1.0_f64;
}
```

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=0c8929defa97460ec4c13963bc461809)

---

### どんな違いがあるのか

* ミュータブル mut
  変数の値を書き換える

* シャドーイング let
  同名の別の変数を作成する

---

#### ミュータブル mut 1

![h:13cm ミュータブル mut 1](./pictures/変数/mut1.jpg)

---

#### ミュータブル mut 2

![h:13cm ミュータブル mut 2](./pictures/変数/mut2.jpg)

---

#### ミュータブル mut 3

![h:13cm ミュータブル mut 3](./pictures/変数/mut3.jpg)

---

#### シャドーイング let 1

![h:13cm シャドーイング let 1](./pictures/変数/let1.jpg)

---

#### シャドーイング let 2

![h:13cm シャドーイング let 2](./pictures/変数/let2.jpg)

---

#### シャドーイング let 3

![h:13cm シャドーイング let 3](./pictures/変数/let3.jpg)

---

#### シャドーイング let 4

![h:13cm シャドーイング let 4](./pictures/変数/let4.jpg)

---

#### どんな違いがあるのか(コード)

[コメントでスタックの状況を説明しながら代入とシャドーイングを行うコード](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=413eae0e9edfd05b7732eb1a99d79e0a)

[ポインタを見ながら動作の違いを説明するコード](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=352946a094ed832632594292c6aa31d8)

---

### 変数 演習

[Runが通るように修正しましょう](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=1562d909fa6bab7d338289c1139729bb)

目標5分

<!-- 10分 + 5分 -->

---

## 型

### 型 結論

Rustは型によって変数作成時の動きが違う

1. **固定長型**の変数はメモリの**スタック**領域に作成　(整数型や、固定長文字列など)
2. **可変長型**の変数はメモリの**ヒープ**領域に作成　(可変長文字列や可変長配列など)

---

### 一覧(固定長？可変長？)

可変長の配列とそれ以外でメモリの使い方が違う  
-> **所有権システム(Rust独特)で振る舞いに影響**

---

#### 固定長1 数値

|bit数              |整数   |符号なし整数|浮動小数|
|:--:               |:--:   |:--:       |:--:   |
|8bit               |i8     |u8         |f8     |
|16bit              |i16    |u16        |f16    |
|32bit              |**i32**|u32        |f32    |
|64bit              |i64    |u64        |**f64**|
|128bit             |i128   |u128       |無し   |
|アーキテクチャ依存   |isize |usize     |無し     |

整数型はi32、浮動小数点型はf64推奨
出典：[プログラミング言語 Rust, 2nd Edition データ型](https://doc.rust-jp.rs/book/second-edition/ch03-02-data-types.html)

---

#### 固定長2 論理値型、文字型、タプル型、配列型

|型名   |記号   |備考                                               |
|:--:   |:--:   |:--:                                               |
|論理値型|bool  |true, false                                        |
|文字型 |char   |ユニコードスカラー値 (U+0000~U+D7FF, U+E000~U+10FFFF)|
|タプル型|( )   |読み取り専用の配列のようなもの                         |
|配列型 |[ ]    |初期化時の配列の長さから変更不可(固定長配列)           |

---

#### 固定長3 すごく独特 スライス型

|型名   |記号   |備考                                    |
|:--:   |:--:   |:--:                                   |
|文字列  |str   |固定長の文字列、ほとんどの場合&strで登場  |
|スライス|\| \| |                                       |

---

#### 可変長の配列

|型名   |記号   |備考              |
|:--:   |:--:   |:--:              |
|文字列 |String |可変長の文字列     |
|配列型 |Vec     |可変長の配列      |
|スマートポインタ|省略      |省略   |

---

※注：書き方は似てるがVec型(可変長)と[]型(固定長)は違う

```rust
fn main(){
    // 固定長
    let mut fixed_vector = [1,2,3,4,5];
    fixed_vector[2] = 0;
    assert_eq!(fixed_vector, [1,2,0,4,5]);
    // error: fixed_vectorはpush()が実装されてない(固定長)
    // fixed_vector.push(6);
}
```

```rust
fn main(){
    // 可変長
    let mut variable_vector = vec![1,2,3,4,5];
    variable_vector[2] = 0;
    variable_vector.push(6);
    assert_eq!(variable_vector, [1,2,0,4,5,6]);
}
```

---

### メモリ上のふるまい

str型とString型の違いって何？

---

### オリジナルな型

---

## 所有権システム

### 所有権システム 結論

所有権システムの働き

* 不正な値の変数を許さない (解放済みの変数、値の入っていない変数etc.)
* 変数の整合性を保証するため、ロックをかける ~~(DBかよ)~~

---

### 詳細説明

の前に、Rustでのunsafeについて

---

### Rustでのunsafe

以下の実装を行う場合にはunsafeブロックで囲む必要がある

* **生ポインタを参照外しする**
  生ポインタ: Cでいうポインタ
  参照外し: ポインタの参照先の値を直接読み取ること
* **可変のグローバル変数にアクセスしたり変更する**
* unsafeな関数やメソッドを呼ぶ
* unsafeなトレイトを実装する
  トレイト: Rustで使う、インターフェースみたいなもの

---

### つまり

Rustは

* 中身の保証されていないポインタの参照外し
* 無秩序なデータ書き換え

がunsafeだと考えている。

そのための**所有権システム**

---

### 所有権 大まかな考え方

(ざっくり)
所有権は変数の未定義動作やデータの競合を防ぐ仕組み
権利のある変数しか、値にアクセスできない

(メモリ解放にも関係するが、ここでは割愛)

---

### 所有権

既存の変数を他の変数に代入したときに固定長か可変長かで動作が変わる

```rust
// &str型は固定長(= 変数はスタック領域)
let fixed_length = "hello world!";
let str_value = fixed_length;
// OK
assert_eq!(fixed_length, str_value);
```

```rust
// String型は可変長(= 変数の本体はヒープ領域)
let variable_length = "hello world!".to_string();
let string_value = variable_length;
// compile error
// assert_eq!(variable_length, string_value)
```

---

#### 図解 所有権1

<!-- 固定長 TODO: 図を追加 -->
コードを書いている時点で、どのくらいメモリを使うかわかる
-> 変数の中身(値)をコピーしても、想定外に時間がかかるなんてことないよね？
-> 値がコピーされる (ディープコピー)

スタック：　コピー(copy)

---

#### 図解 所有権2

<!-- 可変長 TODO: 図を追加 -->
コードを書いている時点で、どのくらいメモリを使うかわからない
-> その変数の中身(値)が動画だったら、コピーに時間がかかるだろう
-> ヒープのアドレスがコピーされる(シャローコピー)
-> 参照カウンターはGCがいるから、コピー元は使用禁止ね

---

#### 図解 所有権3

<!-- 可変長 TODO: 図を追加 -->

---

#### 図解 所有権4

一つのヒープ上の値に対して、
複数のスタック上の変数を割り当てると
GCが必要になる

ヒープ： ムーブ(move)

[所有権で解放後の変数へのアクセスを制限していることを説明するコード](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=99c3f2da10dbca4c3c7cf1c7d97491b0)

---

でも**代入するたびに、ムーブやコピーされるのはつらい**

---

### 参照

≒ 読み取り専用のアクセス権

読み取りだけなら、いくらでも作れる
(書き込みが無いなら、データ競合は考えなくても良い)

<!-- TODO:図を追加 -->

---

### 可変の参照(mut)

≒ 読み書き可能なアクセス権

書き込みがあるとデータ競合を考えなくてはならない
-> 1人しかアクセスできなければ、データ競合を考えなくて良い
-> 占有ロック(みたいなこと)しよう！

* 2つ以上作れない
* 不変参照が存在する場合は作れない

[不変参照と可変参照は共存不可](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=ba1b4fa8f92a8c738ea2ba7dcb72b1dd)

---

## 基本構文

---

## ライフタイム

参照が参照先より長生きするのを防ぐ仕組み
これが無いと、解放後のメモリにアクセスしてしまう

```rust
fn main(){
    let long_lifetime;
    {
        let short_lifetime = 2;
        long_lifetime = &short_lifetime;
    } // ここでshort_lifetimeか解放される
    // compile error: 解放されたshort_lifetimeにアクセスする
    // println!("{}", long_lifetime);
} // ここでlong_lifetimeか解放される
```

---

### ライフタイム注釈

```rust
// compile error: 返り値がいつまで使われるのか分からない
fn compare_str_lenght(a: &str, b: &str) -> &str{
    if a.len() >= b.len()　{
        a
    } else {
        b
    }
}
```

```rust
// OK: 返り値は'a(a,b)と同じだけの生存しなければならない
fn compare_str_lenght<'a>(a: &'a str, b: &'a str) -> &'a str{
    // 省略
}
```

---

### 変数と参照のライフタイム

// todo: 追記

```rust
fn main(){
    let value = 10;
    let ref_value = &value;
}

```

---

## エラー処理

### panic!

プログラムを異常終了させるためのマクロ

```rust
fn main(){
    print!("hello");
    panic!();
    print!("world"); // ここにはたどり着けない
}
```

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=99f97f587da0ca0f9f5f87f3de4cc655)

---

### Result型、Option型

[Result型](https://doc.rust-lang.org/std/result/enum.Result.html): エラーになりうる結果を返す時に使う

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

[Option型](https://doc.rust-lang.org/std/option/enum.Option.html): 他言語で言う、Nullになりうる結果を返す時に使う

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

---

### パターンマッチ

Result型やOption型はそのままでは使えない
パターンマッチで、中身を取り出す
unwrap()は非推奨

```rust
fn main(){
    use std::convert::TryFrom;
    let value = u32::try_from(-1); // キャストは失敗するかもしれないのでResult型
    match value{
        Ok(x) => println!("{}", x),
        Err(x) => println!("{}", x),
    }
}
```

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=227c23cf9f037e677d674c519f18e166)

---

## 構造体(struct)

### 構造体(struct) 結論

* 構造体≒変数のみクラス
* 構造体にメソッドを組み込んでクラスの代わりにする
* インターフェースも使える

---

### 構造体って何？

(ざっくり)メソッドが持てない、**変数だけのクラス**

```rust
// rust
struct Vector2 {
    x: f64,
    y: f64,
}
```

```ruby
# ruby
class Vector2 {
    @x
    @y
}
```

---

### お前のメソッド実装はおかしい

すでにある構造体(struct)に
メソッド(fn)を
組み込む(impl)

---

### new、halfメソッド実装例

```rust
struct Vector2 { /* 省略 */ }
impl Vector2 { // Vector2に組み込むという意味
    // 別にメソッド名はnewでなくても良い buildでもhogehogeでも
    fn new(x_pos: f64, y_pos: f64) -> Self{
        Self {
            x: x_pos,
            y: y_pos,
        }
    }
    // 各要素を半分にする
    fn half(&mut self) {
        self.x /= 2.0;
        self.y /= 2.0;
    }
}
```

---

### Selfとselfって何が違うの?

Selfは型、selfは変数(メソッドの対象)

C#で例えると

```C#
// C#のHttpClientの宣言
HttpClient httpClient = new HttpClient();
    ↑Self       ↑self    ↑ HtmlClient::new()
```

---

### トレイト≒インターフェース

```rust
struct Vector2{x: f64, y: f64}
struct Circle{r: f64}

trait AreaCalculable{
    fn calc_area(&self) -> f64;
}

impl AreaCalculable for Vector2 {
    fn calc_area(&self) -> f64 {
        &self.x * &self.y
    }
}
impl AreaCalculable for Circle {// 省略
```

[全コード](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=79688fcc8d8d56423cd074e6a3110612)

---
