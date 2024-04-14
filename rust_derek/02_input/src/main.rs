
use std::io;

fn main() {

	println!("What is your name?");
	let mut name = String::new();
	let greeting: &str = "Nice to meet you!";

	io::stdin()
		.read_line(&mut name)
		.expect("Error input name!");

	println!("Hello, {}! {}", name.trim(), greeting);

}

/*

	1) use std::io; => подключаем стандартную библиотеку ввода-вывода
	2) println!() => вызов стандартного макроса вывода строки на экран
	3) let mut name => создаем мутабельную (изменяемую) переменную
	4) String::new() => которая равна новой (пустой) строке
	5) let greeting: &str => создаем ссылку на строку (неизменяемую строку)
	6) io::stdin() => организуем ввод данных от пользователя
	7) .read_line(&mut name) => читаем строку в переменную name
	8) .expect("...") => если произошла ошибка, то вывести сообщение
	9) println!("{}") => вывод строки с внедрением значения переменной

*/ 