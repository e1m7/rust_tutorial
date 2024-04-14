
#![allow(unused)]
use rand::Rng;

fn main() {

	let num_1: f32 = 1.111111111111111;
	println!("f32 = {}", num_1 + 0.111111111111111);

	let num_2: f64 = 1.111111111111111;
	println!("f64 = {}", num_2 + 0.111111111111111);

	let num_3: u32 = 5;
	let num_4: u32 = 4;

	println!("5 + 4 = {}", num_3 + num_4);
	println!("5 - 4 = {}", num_3 - num_4);
	println!("5 * 4 = {}", num_3 * num_4);
	println!("5 / 4 = {}", num_3 / num_4);
	println!("5 % 4 = {}", num_3 % num_4);

	println!("==================================");

	let mut num_3: u32 = 5;
	let mut num_4: u32 = 4;

	num_3 += 8;
	num_4 += 2;

	println!("13 + 6 = {}", num_3 + num_4);
	println!("13 - 6 = {}", num_3 - num_4);
	println!("13 * 6 = {}", num_3 * num_4);
	println!("13 / 6 = {}", num_3 / num_4);
	println!("13 % 6 = {}", num_3 % num_4);

	println!("==================================");

	let random_num: i32 = rand::thread_rng().gen_range(1..101);
	println!("Random number = {}", random_num);

}

/*

	1) f32 дает точность до 6 знаков после запятой
	2) f64 дает точность до 14 знаков после запятой
	3) неизменным переменным нельзя присвоить новое значение
	4) изменяемым переменным можно присвоить новое значение
	5) чтобы подключить библиотеку генерации случайных чисел
	5-1) rand = "0.8.5" => дописать в carbo.tomp
	5-2) use rand::Rng; => дописать в начало main.rs

*/ 
