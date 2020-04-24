fn main() {
    println!("---初期状態---");
    let mut value = 1;
    let value_pointer1 = &value as *const i32; // assert_eq!で使用するために宣言
    println!("アドレス:{:p}, 値:{:?}", &value, &value);


    println!("---ミュータブル---");
    value = 2;
    let value_pointer2 = &value as *const i32; // assert_eq!で使用するために宣言
    println!("アドレス:{:p}, 値:{:?}", &value, &value);

    // ミュータブルでは、アドレスはそのままで、値を書き換える
    assert_eq!(value_pointer1, value_pointer2);


    println!("---シャドーイング---");
    let value = "文字列";
    let value_pointer3 = &value as *const &str; // assert_eq!で使用するために宣言
    println!("アドレス:{:p}, 値:{:?}", &value, &value);

    // シャドーイングでは、そもそも、値を保存している場所(アドレス)が違う
    assert_ne!(format!("{:?}", value_pointer1), format!("{:?}", value_pointer3));
}