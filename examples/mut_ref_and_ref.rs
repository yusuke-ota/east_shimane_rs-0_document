fn main(){
    let mut number = 0;
    let ref_number = &number;
    // compile error: 参照がある状態で、可変参照は作成できない
    let ref_mut_number = &mut number;
    // 参照は最後に使われたところで解放される
    // 使わないと、let ref_number = &number;でライフタイムが切れる
    println!("{:?}", ref_number);
    println!("{:?}", ref_mut_number);
}