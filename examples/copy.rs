fn main(){
    let mut a = 1;
    plus_one(a);
    assert_eq!(a, 1); // aの値は変わらない
}

fn plus_one(mut input: i32){ // aとは別のinputが生成される
    input += 1 // inputの値が変わってもaには何の影響もない
}