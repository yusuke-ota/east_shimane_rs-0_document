fn main(){
    use std::convert::TryFrom;
    // キャストは失敗するかもしれないのでResult型
    let value = u32::try_from(-1);
    match value{
        Ok(x) => println!("{}", x),
        Err(x) => println!("{}", x),
    }
}