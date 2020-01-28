---
marp: true
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

- プログラミング言語 Rust, 2nd Edition 日本語訳  
    <https://doc.rust-jp.rs/book/second-edition/>  
    注: 2018年6月ごろの英語版(Rust2015版)ベースのため内容が少し古い

- 実践Rust入門  
    一言でいうと、たのしいRubyみたいな本  
    プログラミング言語 Rust, 2nd Editionの次に読むと良い

---

## 目次

- Rustの特徴(3分)
- 変数(12分)
  <!-- - ミュータブル?イミュータブル?
  - シャドーイング
  - 演習 -->
- 型
  <!-- - 一覧(固定長？可変長？)
  - メモリ上のふるまい
  - オリジナルな型 -->

<!-- --- -->

<!-- ## 目次 2 -->

- 所有権
  <!-- - 大まかな考え方
  - 所有権
  - 参照
  - 参照(mut) -->
- 基本構文

<!-- --- -->

<!-- ## 目次 3 -->

- ライフタイム
  <!-- - ≒スコープ
  - ライフタイム注釈 -->
- エラー処理
  <!-- - panic!
  - Result型、Option型
  - パターンマッチ -->

- 構造体(struct)

---

## Rustの特徴 1

### メリット

- GCがないから、速い
- 非同期処理が安心して書ける
- FFIでRubyから簡単に呼べる

---

## Rustの特徴 2

### デメリット

- 所有権って何?(独自の概念)
- クラスの書き方が独特(structから始める)
- async/awaitの書き方が独特

---

## 変数

### 変数 結論

Rustの変数は基本再代入不可
再代入するには2つの方法がある

1. 可変変数として宣言する
2. 同じ名前の別の変数を新規作成する

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

- ミュータブル mut
  変数の値を書き換える

- シャドーイング let
  同名の別の変数を作成する

---

#### ミュータブル mut 1

![ミュータブル mut 1](./pictures/変数/mut1.jpg)

---

#### ミュータブル mut 2

![ミュータブル mut 2](./pictures/変数/mut2.jpg)

---

#### ミュータブル mut 3

![ミュータブル mut 3](./pictures/変数/mut3.jpg)

---

#### シャドーイング let 1

![シャドーイング let 1](./pictures/変数/let1.jpg)

---

#### シャドーイング let 2

![シャドーイング let 2](./pictures/変数/let2.jpg)

---

#### シャドーイング let 3

![シャドーイング let 3](./pictures/変数/let3.jpg)

---

#### シャドーイング let 4

![シャドーイング let 4](./pictures/変数/let4.jpg)

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
|文字型 |char   |ユニコードスカラー値(U+0000~U+D7FF, U+E000~U+10FFFF)|
|タプル型|( )   |読み取り専用の配列のようなもの                         |
|配列型 |[ ]    |初期化時の配列の長さから変更不可(固定長配列)           |

---

#### 固定長3 すごく独特 スライス型

|型名   |記号   |備考                                               |
|:--:   |:--:   |:--:           |
|文字列  |str   |固定長の文字列 |
|スライス|\| \| |               |

---

#### 可変長の配列

|型名   |記号   |備考                                               |
|:--:   |:--:   |:--:                                               |
|文字列|String |可変長の文字列                 |
|配列型|Vec     |可変長の配列                   |
||Box      ||

※注：書き方は似てるがVec型(ヒープ)と[]型(スタック)は違う
<!-- todo: コード作成 -->

---

### メモリ上のふるまい

str型とString型の違いって何？

---

### オリジナルな型

---

## 所有権システム

### 所有権システム 結論

所有権システムの働き

- 不正な値の変数を許さない (解放済みの変数、値の入っていない変数etc.)
- 変数の整合性を保証するため、ロックをかける ~~(DBかよ)~~

---

### 詳細説明

の前に、Rustでのunsafeについて

---

### Rustでのunsafe

以下の実装を行う場合にはunsafeブロックで囲む必要がある

- **生ポインタを参照外しする**
- **可変のグローバル変数にアクセスしたり変更する**
- unsafeな関数やメソッドを呼ぶ
- unsafeなトレイトを実装する

---

### 所有権 大まかな考え方

(ざっくり)
所有権は変数の未定義動作や意図しない書き換えを防ぐための仕組み
権利のある変数しか、値にアクセスできない

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

<!-- 固定長 -->

---

#### 図解 所有権2

<!-- 可変長 -->

---

#### 図解 所有権3

[所有権で解放後の変数へのアクセスを制限していることを説明するコード](
https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=39dc9cf912897eade2aab167391ca7a5)

---

### 参照

---

### 可変の参照(mut)

- 2つ以上作れない
- 不変参照が存在する場合は作れない(todo: 検証)

---

## 基本構文

---

## ライフタイム

### ≒スコープ

---

### ライフタイム注釈

---

## エラー処理

### panic!

---

### Result型、Option型

---

### パターンマッチ

---

## 構造体(struct)

### 構造体(struct) 結論

- 構造体≒変数のみクラス
- 構造体にメソッドを組み込んでクラスの代わりにする
- もちろんインターフェースも使える

---

### 構造体って何？

メソッドが持てない、**変数だけのクラス**
上下のコードはほぼ等価

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
struct Vector2 {
// 省略
}
impl Vector2 { // Vector2に組み込むという意味
    // 別に関数名はnewでなくても良い buildでもhogehogeでも
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

例(C#)

```C#
// C#のhttpClientの宣言
HttpClient httpClient = new HttpClient();
    ↑Self       ↑self    ↑ HtmlClient::new()
```

---

### トレイト≒インターフェース

```rust
struct Vector2{x: f64, y: f64}
struct Circle{r: f64}

// ≒インターフェース
trait AreaCalculable{
    fn calc_area(&self) -> f64;
}

impl AreaCalculable for Vector2 {
    fn calc_area(&self) -> f64 {
        &self.x * &self.y
    }
}

impl AreaCalculable for Circle {
// 省略
}
```

[全コード](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=79688fcc8d8d56423cd074e6a3110612)

---
