fn main() {
    // let, mut, {}を追加して、Runが通るようにしてください
    let value = "変数1";
    assert_eq!(value, "変数1");

    value = 1.0;
    assert_eq!(value, 1.0);

    value = 2.0;
    assert_eq!(value, 2.0);

    value = "変数2";
    assert_eq!(value, "変数2");
}

// 解法1

// let value = "変数1";
// assert_eq!(value, "変数1");

// {
//     let mut value = 1.0;
//     assert_eq!(value, 1.0);

//     value = 2.0;
//     assert_eq!(value, 2.0);
// }

// value = "変数2";
// assert_eq!(value, "変数2");


// 解法2

// let value = "変数1";
// assert_eq!(value, "変数1");

// let mut value = 1.0;
// assert_eq!(value, 1.0);

// value = 2.0;
// assert_eq!(value, 2.0);

// let value = "変数2";
// assert_eq!(value, "変数2");
