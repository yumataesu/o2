pub fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
}

/*
use regex::Regex;

// let udid : i64 = 0;
// let score : f64 = 0.546;
// let id = "id";
// let name = "name".to_string();
// let age = String::from("43");
// let mut misc = String::new();
// misc = "misc".to_string();

pub fn join_string_to_string() {
    let a = "Yuma".to_string();
    let mut b = String::from("Taesu");

    // let result1 = a + &b; //aにムーブが起きる
    let result2 = format!("{}{}", a,  b); //一番オススメなやりたかたっぽい
    b.push_str(&a); //push_strで後ろにつけることができる。場合によっては、format!と併用してもいいかもしれない、

    // let err = a + b;
    println!("{}", result2);
    println!("{}", b);
}

// &strと&strの連結
// ２つの&strを簡単にくっつける方法は無い。らしい
pub fn join_strref_to_strref() {
    let country = "Japan";
    let prefecture = "Tokyo";

    let result = format!("{}, {}", country, prefecture);

    println!("{}", result);
}

//その文字列で始まるか含まれているか
pub fn starts_with(a : &str, b : &str) -> bool {
    a.starts_with(b)
}

//https://murashun.jp/article/programming/regular-expression.html
pub fn is_phone_number(a : &str) -> bool {
    let re = Regex::new(r"\d{3}-\d{4}-\d{4}$").unwrap();
    re.is_match(a)
}
*/