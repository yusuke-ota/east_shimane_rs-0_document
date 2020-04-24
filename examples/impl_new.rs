struct Vector2 {
    x: f64,
    y: f64,
}
impl Vector2 { // Vector2に実装する
// 別にメソッド名はnewでなくても良い buildでもhogehogeでも
fn new(x_pos: f64, y_pos: f64) -> Self{
    Self {
        x: x_pos,
        y: y_pos,
    }
}
    // 各要素を半分にする
    fn half(&mut self) {
        self.x /= 2.0;
        self.y /= 2.0;
    }
}

fn main() {
    let mut vec2 = Vector2::new(2.0,2.0);
    vec2.half();
    println!("{:?}", (vec2.x, vec2.y));
}