struct Vector2{x: f64, y: f64}
struct Circle{r: f64}

// ≒インターフェース(C# , Java)
trait AreaCalculable{
    fn calc_area(&self) -> f64;
}

impl AreaCalculable for Vector2 {
    fn calc_area(&self) -> f64 {
        &self.x * &self.y
    }
}

impl AreaCalculable for Circle {
    fn calc_area(&self) -> f64 {
        &self.r.powi(2) * 3.14
    }
}

// トレイト境界を使って、
// AreaCalculableを実装しているインスタンスのみに使えるメソッドを作る
fn print_result<T>(area_calculable: &T)
    where T: AreaCalculable {
    println!("{:?}", area_calculable.calc_area())
}

fn main(){
    let vec = Vector2{x:2.0, y:5.0};
    let circle = Circle{r: 10.0};
    print_result(&vec);
    print_result(&circle);
}