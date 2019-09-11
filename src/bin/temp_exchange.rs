use std::io;
fn main () {
    let mut temp_type = String::new();
    println!("Please choose input temp type F or C.");
    io::stdin()
        .read_line(&mut temp_type)
        .expect("Failed to read line");
    let temp_type = temp_type
        .trim();
    if temp_type == "F" {
        println!("set input temp type to ℉")
    } else if temp_type == "C" {
        println!("set input temp type to ℃")
    } else {
        println!("temp type is not divisible by 'F' or 'C'");
        main();
    }
    println!("Please input exchange value");
    let mut start_value = String::new();
    io::stdin()
        .read_line(&mut start_value)
        .expect("Failed to read line");
    let start_value:f64 = start_value
        .trim()
        .parse()
        .expect("Please type a number!");
    if temp_type == "F" {
        println!("{}℉ exchange to {:+.2}℃.", start_value, f2c(start_value))
    } else {
        println!("{}℃ exchange to {:+.2}℉", start_value, c2f(start_value))
    }
}
fn f2c (f: f64) -> f64 {
    return (f - 32.0) / 1.8
}
fn c2f (c: f64) -> f64 {
    return (c + 32.0) * 1.8
}
//3.1415926	{:.2}	{:.2f}	3.14	保留小数点后两位
//3.1415926	{:+.2}	{:+.2f}	+3.14	带符号保留小数点后两位
//-1	{:+.2}	{:+.2f}	-1(R)/-1.00(P)	带符号保留小数点后两位
//-1.0	{:+.2}	{:+.2f}	-1.00	带符号保留小数点后两位
//2.71828	{:.0}}	{:.0f}	3	不带小数
//5	{:0>2}/{:02}	{:0>2d}/{:02d}	05	数字补0 (填充左边, 宽度为2)
//5	{:x^10}	{:x^10d}	xxxx5xxxxx	居中对齐
//5	{:x<4}	{:x<4d}	5xxx	数字补x (填充右边, 宽度为4)
//1000000	NA.	{:,}	1,000,000	以逗号分隔的数字格式
//0.25	NA.	{:.2%}	25.00%	百分比格式
//1000000000	NA.	{:.2e}	1.00e+09	指数记法
//1000000000.0	{:2e}	{:.2e}	1e9(R)/1.00e+09(P)	指数记法
//1000000000.0	{:2E}	{:.2E}	1E9(R)/1.00E+09(P)	指数记法
//42	{:b}	{:b}	101010	二进制
//42	{:o}	{:o}	52	八进制
//42	{:x}	{:x}	2a	十六进制
//42	{:X}	{:X}	2A	十六进制
//42	{:#b}	{:#b}	0b101010	带前缀的二进制
//42	{:#o}	{:#o}	0o52	带前缀的八进制
//42	{:#x}	{:#x}	0x2a	带前缀的十六进制
//42	{:#X}	{:#X}	0x2A(R)/0X2A(P)	带前缀的十六进制
