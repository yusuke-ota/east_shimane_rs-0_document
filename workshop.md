---
marp: true
---

# Rust勉強会資料

## 参考文献

- プログラミング言語 Rust, 2nd Edition 日本語訳  
    <https://doc.rust-jp.rs/book/second-edition/>  
    注: 2018年6月ごろの英語版(Rust2015版)ベースのため内容が少し古い

- 実践Rust入門  
    一言でいうとたのしいRubyみたいな本

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

---

### なぜなのか？

Rustの変数は基本**イミュータブル(変更不可)**

---

### どうしたらよいか？

方法は2つ

#### 1. ミュータブル(変更可能)な変数として宣言する

    '''rust
    fn main(){
        let mut value = "変数1";
        value = "変数2";

        // コンパイルエラー
        // value = 2.0_f64;
        // "変数2"は&str型、2.0はf64型で型が違う
    }
    '''

---

#### 2. 再代入する際にletを付ける(シャドーイング)

    '''rust
    fn main(){
        let value = "変数1";
        let value = "変数2";

        // OK
        // let value = 1.0_f64;
        // valueをf64型として新規作成
    }
    '''

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

#### シャドーイング

ちょっと詳しいコード
<https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=413eae0e9edfd05b7732eb1a99d79e0a>

---

## 型

## 基本構文

## エラー処理

## 所有権

## ライフタイム
