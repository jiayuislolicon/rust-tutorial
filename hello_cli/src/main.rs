fn main() {
    let s = String::from("你好世界");
    s.len(); // 12（bytes，不是 4）
    s.chars().count(); // 4（字數）
    &s[0..1]; // 執行時 panic
}
