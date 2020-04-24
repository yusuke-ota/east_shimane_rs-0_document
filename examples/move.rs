fn main() {
    let word = "hello".to_string();
    println!("word変数のアドレス       {:p}", &word);
    println!("ヒープ上のデータのアドレス {:p}", word.as_ptr());
    // 生ポインタの作成はsafe
    let pointer = &word as *const String;

    // wordの所有権を渡す
    say_once(word, pointer);

    // wordが解放されたので、ヒープ上のデータにアクセスしても空
    // 解放されたメモリにアクセスするのは未定義動作を引き起こして危険
    // これを所有権で防いでいる
    unsafe {
        println!("解放後のヒープ上のデータ(生ポインタ経由）{:?}", *pointer);
    }
}

fn say_once(message: String, pointer: *const String) {
    // messageに引数の所有権が移る
    // 参照はスタック上にあるので、所有権が移る際にコピーされる
    println!("message変数のアドレス    {:p}", &message);
    println!("ヒープ上のデータのアドレス {:p}", message.as_ptr());
    // println!("{:?}", pointer);

    // 生ポインタでの参照外しはunsafe
    // wordのヒープ上のデータにポインタ経由でアクセスする
    // 生ポインタは所有権システムのチェックにかからない
    unsafe {
        println!("wordのヒープ上のデータ(生ポインタ経由）{:?}", *pointer);
    }

    println!("messageのヒープ上のデータ           {:?}", message);
}   // messageのスコープが外れる
