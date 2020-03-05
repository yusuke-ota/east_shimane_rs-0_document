fn main() {
    println!("--- Step1 ---");
    // 変数新規作成
    // | id | value |
    // |:--:| :--:  |
    // |  1 |"変数1"|
    let value = "変数1";
    println!("{}", value);

    println!("--- Step2 ---");
    // シャード―イング
    // | id | value |
    // |:--:| :--:  |
    // |  2 |  1.0  |
    // |  1 |"変数1"|
    let value = 1.0;
    println!("{}", value);

    {
        // シャード―イング
        // | id | value |
        // |:--:| :--:  |
        // |  3 |  2.0  |
        // |  2 |  1.0  |
        // |  1 |"変数1"|
        println!("--- Step3 ---");
        let value = 2.0;
        println!("{}", value);
    } // <- スコープを抜ける
    // | id | value |
    // |:--:| :--:  |
    // |  2 |  1.0  |
    // |  1 |"変数1"|
    println!("--- Step4 ---");
    println!("{}", value);

    // シャード―イング
    // | id | value |
    // |:--:| :--:  |
    // |  3 |"変数2"|
    // |  2 |  1.0  |
    // |  1 |"変数1"|
    println!("--- Step5 ---");
    let value = "変数2";
    println!("{}", value);
}
