// Arc<Mutex<T>>
use std::sync::{Mutex, Arc};
use std::thread;
fn main(){
    // アトミック性と、ロック機構を持ったスレッド間共有用の変数
    // アトミック性: 値の書き換えが一瞬で終わるように**見える**こと
    //             アトミック性がない場合、書き換え途中の値が見えてしまうことがある
    let share_number = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for index in 0..10{
        // share_numberがi32型の場合、ここで可変参照を複製できない
        let share_number = Arc::clone(&share_number);

        let handle = thread::spawn(move || {
            // share_numberの参照を取得しているスレッドがない場合、
            // 可変参照を取得し、他から使えないようロックをかける
            // ほかで使われている場合ロックが空くまで待つ
            let mut num = share_number.lock().unwrap();
            *num += index;
            // 現在使用しているスレッドの番号を表示する
            println!("{:?}", thread::current().id());
        });

        handles.push(handle);
    }

    // handlesの中身を全部待つ
    for handle in handles{
        // これは、unwrap()を使っていいパターンだけど参考のためパターンマッチ
        // unwrap()を使っていいパターン
        // 1.絶対にOk(T)が帰る場合
        // 2.Err(E)が起こるパターンが致命的なので、強制終了したい場合
        match handle.join(){
            Ok(()) => (), // 返り値は空なので、何もしない
            _ => println!("予期しない動作"), // 空じゃないほうがおかしい
        };
    }

    println!("{}", share_number.lock().unwrap());
}