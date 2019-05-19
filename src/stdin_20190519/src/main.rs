use std::io;
fn main() {
    println!("標準入力テストです。任意の値を入力してください。");
    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("入力エラー。read_line()で失敗しました。");
    println!("入力値: {}", x);
}
