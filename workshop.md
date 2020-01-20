---
marp: true
---

# Rust勉強会資料

## 目的

Rustの独特な部分をざくっと説明して、入門コストを下げるぞー  
~~Rust強くなって私に教えてください~~

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

---

## Rustの特徴 1

### メリット

- GCがないから、OSが無くても使える
- 速い(C++と同程度の速度で動くらしい)
  <https://qiita.com/ACUVE/items/598cecf687cb771f7242>
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

### ミュータブル？イミュータブル？

Rustの変数は以下の様に宣言
しかし、コメントアウト部分でエラーがおきる

    '''rust
    fn main(){
        let value = "変数1";

        // コンパイルエラー
        // value = "変数2";
    }
    '''

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=9038c11efd6b7cf7060d32b1ce149fc2)

---

### なぜなのか？

Rustの変数は基本**イミュータブル(変更不可)**

---

### どうしたらよいか？

方法は2つ

1. ミュータブル(変更可能)な変数として宣言する
1. 再定義する際にletを付ける(シャドーイング)

---

#### 1. ミュータブル(変更可能)な変数として宣言する

    '''rust
    fn main(){
        let mut value = "変数1";
        value = "変数2";

        // コンパイルエラー
        // "変数2"は&str型、2.0はf64型で型が違う
        // value = 2.0_f64;
    }
    '''

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=31a5fc47dd3ff9252bc517c371518b9e)

---

#### 2. 再定義する際にletを付ける(シャドーイング)

    '''rust
    fn main(){
        let value = "変数1";
        let value = "変数2";

        // OK
        // valueをf64型として新規作成
        let value = 1.0_f64;
    }
    '''

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=0c8929defa97460ec4c13963bc461809)

---

### どんな違いがあるのか(イメージ)

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

[ポインタを見ながら動作の違いを説明するコード](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=8648408835ace394f49fc8b9de4567ca)

---

### 変数 演習

[Runが通るように修正しましょう](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=1562d909fa6bab7d338289c1139729bb)

目標5分

<!-- 10分 + 5分 -->

---

## 型

### 一覧(可変長の配列？それ以外？)

可変長の配列とそれ以外でメモリの使い方が違う  
-> 所有権システム(Rust独特)で振る舞いが違う

---

#### それ以外1 数値

|bit数              |整数   |符号なし整数|浮動小数|
|:--:               |:--:   |:--:       |:--:   |
|8bit               |i8     |u8         |f8     |
|16bit              |i16    |u16        |f16    |
|32bit              |**i32**|u32        |f32    |
|64bit              |i64    |u64        |**f64**|
|アーキテクチャ依存   |isize |usize     |無い     |

整数型はi32推奨
浮動小数点型はf64推奨

出典：[プログラミング言語 Rust, 2nd Edition データ型](https://doc.rust-jp.rs/book/second-edition/ch03-02-data-types.html)

---

#### それ以外2 論理値型、文字型、タプル型、配列型

|型名   |記号   |備考                                               |
|:--:   |:--:   |:--:                                               |
|論理値型|bool  |true, false                                        |
|文字型 |char   |ユニコードスカラー値(U+0000~U+D7FF, U+E000~U+10FFFF)|
|タプル型|()    |読み取り専用の配列のようなもの                         |
|配列型 |[]     |初期化時の配列の長さから変更不可(固定長配列)           |

---

#### それ以外3 すごく独特 スライス型

---

#### 可変長の配列

str型とString型の違いって何？

---

### メモリ上のふるまい

---

### オリジナルな型

---

## 所有権システム

### 大まかな考え方

(ざっくり)
所有権は変数の未定義動作や意図しない書き換えを防ぐための仕組み
権利のある変数しか、値にアクセスできない

---

### 所有権

---

### 参照

---

### 可変の参照(mut)

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
