
#![allow(unused)]
use std::collections::HashMap;

fn main() {

	let mut heroes = HashMap::new();
	heroes.insert("Superman", "Clark Kent");
	heroes.insert("Batman", "Bruse Wayne");
	heroes.insert("Wonder Woman", "Diana Prince");
	heroes.insert("Spider-Man", "Peter Parker");
	heroes.insert("Iron Man", "Tony Stark");
	heroes.insert("The Flash", "Barry Allen");
	heroes.insert("Green Lantern", "Hal Jordan");
	heroes.insert("Hulk", "Bruce Banner");

	// перебор всех ключей и значений
	for (k, v) in heroes.iter() {
		println!("{} => {}", k, v);
	}

	// вывод длины мапы
	println!("Length map = {}", heroes.len());

	// проверка существования ключа
	if heroes.contains_key(&"Batman") {
		println!("Да, у нас есть Бэтмен");
	} else {
		println!("Нет, у нас нет Бэтмена");
	}

	// проверка существования ключа
	if heroes.contains_key(&"Joker") {
		println!("Да, у нас есть Джокер");
	} else {
		println!("Нет, у нас нет Джокера");
	}

}

/*

	1) hashmap (хэш-мап) это структура, которая позволяет хранить пару ключ : значение
	2) hashmap не является упорядоченной структурой, пары в памяти находятся в произвольном порядке

*/ 
