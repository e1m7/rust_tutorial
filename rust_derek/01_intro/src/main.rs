
#![allow(unused)]

fn main() {
    println!("Hello, World!");
    let a = 10;
}

/*

	1) высокая скорость работы
	2) нет сборщика мусора
	3) параллельное программирование
	4) простые ошибки

	>>> cargo new name_project
	>>> cd name_project
	>>> cargo run

	main.rs = основной код проекта (в каталоге src)
	cargo.toml = версии зависимостей (которые выставлены)
	cargo.lock = версии зависимостей (которые сработали)

	#![allow(unused)] = не предупреждать

*/
