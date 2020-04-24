fn main(){
    println!("Step 1: String型作成");
    // |stack|id |val     |heap|id  |val        |
    // |:--:|:--:|:--:    |:--:|:--:|:--:       |
    // |    |1   |heap id1|    |1   |hello world|
    let mut string = "hello world".to_string();
    println!("string stack pointer:{:p}", &string);
    println!("string heap pointer :{:p}", string.as_ptr());
    let string_pointer = string.as_ptr();

    println!();
    println!("Step 2: str型作成");
    // |stack|id |val      |heap|id  |val        |
    // |:--:|:--:|:--:     |:--:|:--:|:--:       |
    // |    |2   |stack id1|    |    |           |
    // |    |1   |heap id1 |    |1   |hello world|
    let str = &mut string;
    assert_eq!("hello world", *str);
    println!("str stack pointer   :{:p}", &str);
    println!("str heap pointer    :{:p}", str.as_ptr());

    println!();
    println!("Step 3: strの参照先に別の値を割り当て");
    // |stack|id |val      |heap|id  |val        |
    // |:--:|:--:|:--:     |:--:|:--:|:--:       |
    // |    |2   |stack id1|    |2   |hello rust |
    // |    |1   |heap id2 |    |1   |           |
    *str = "hello rust".to_string(); // ヒープの1番が使われなくなる
    unsafe {println!("old heap:{:?}", *string_pointer)}; // ヒープの1番の中身が空
    unsafe {println!("new heap:{:?}", *str.as_ptr())}; // ヒープの2番の中身はある
    println!("str stack pointer   :{:p}", &str);
    println!("str heap pointer    :{:p}", str.as_ptr());
    assert_eq!("hello rust", string);
}