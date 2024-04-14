
#![allow(unused)]

fn main() {
  
	println!("Maximum number u32: {}", u32::MAX);
	println!("Maximum number u64: {}", u64::MAX);
	println!("Maximum number u128: {}", u128::MAX);
	println!("Maximum number f32: {}", f32::MAX);
	println!("Maximum number f64: {}", f64::MAX);

	let is_true: bool = true;
	let is_false = false;
	println!("Boolean {} {}", is_true, is_false);

	let my_litera_a: char = 'A';
	let my_litera_b = 'B';
	println!("My liters {} {}", my_litera_a, my_litera_b);

}

/*

	1) все переменные в программе должны иметь определенный тип
	2) тип указываетс программистом явно или выводится компилятором
	3) unsigned integer: u8, u16, u32, u64, u128 (только положительные числа)
	4) signed integer: i8, i16, i32, i64, i128 (положительные и отрицательные)
	5) float: f32, f64 (дробные числа)
	6) boolean: true, false (булевые переменные)
	7) character: char (символы, строка из одного символа)
	8) размер числа зависит от разрядности компьютера

*/
