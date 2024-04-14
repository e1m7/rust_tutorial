
#![allow(unused)]

fn main() {

	const ONE_MILLION: u32 = 1_000_000;
	const PI: f32 = 3.141592;
	let age: &str = "42";											// age = неизменяемая строка
	let mut age: u32 = age										// age = изменяемое число u32
		.trim()
		.parse()
		.expect("Please enter a number!");
	age = age + 1;

	println!("Cost million {}", ONE_MILLION);
	println!("Const pi {}", PI);
	println!("Print age {}", age);

	let a: u32 = 10;													// a = неизменяемое число
	let a: u32 = 11;													// a = неизменяемое число
	println!("a = {}", a);

}

/*

	1) все константы пишутся заглавными буквами
	2) переменные могут затенять друг друга
	3) a = 10, a = 11 => a = 11 (но будет предупреждение)
	4) age = "42", age = 42 => age = 42 (не будет предупреждения) 
	4-1) .trim() = убрать пробелы до и перед строкой
	4-2) .parse() = сделать парсинг строки в число
	4-3) .expect() = прекратить выполнение и выдать предупреждение

*/