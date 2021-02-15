
pub fn join_string_to_string() {
    let mut a = "Yuma".to_string();
    let mut b = String::from("Taesu");

    // let result1 = a + &b; //aにムーブが起きる
    let result2 = format!("{}{}", a,  b); //一番オススメなやりたかたっぽい
    b.push_str(&a); //場合によっては、format!と併用してもいいかもしれない、

    // let err = a + b;
    println!("{}", result2);
    println!("{}", b);
}
