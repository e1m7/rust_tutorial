
#![allow(unused)]

// библиотека работы с файлами
use std::fs::File;
// библиотеки для чтения и записи файлов
use std::io::{BufRead, BufReader, ErrorKind, Write};

/*

	1) в любой программе есть простой код, а есть потенциально опасный код
	2) простой код это, например, создание немутабельной переменной или вывод на экран строки
	3) потенциально опасный код это, например, спросить у пользователя число или прочитать файл
	4) после потенциально опасного когда надо ВСЕГДА проверять его результат
	4) результат может быть либо Ok(result), либо Err(error)

		Например, мы попросили у пользователя ввести число
		=> Ok(), значит нам пришло Ok(result)
		=> Err(), значит нам пришло Err(error)

		Что за тип данных может быть сразу Ok(число) || Err(строка)?
		=> enum Result {
			Ok(),
			Err(),
		}

*/

// вспомогательная функция деления двух чисел
fn divide(x: f64, y: f64) -> Result<f64, String> {				// 1) определение функции, получает два числа, возвращает Result {число, строка}
	if y == 0.0 {																						// 2) если второе число = 0.0 (деление в этом случае невозможно по математике)
		Err("Division by zero".to_string())										// 3) ... возвращаем из функции Err(строка)
	} else {																								// 4) если второе число != 0.0 (деление возможно и не будет опасности)
		Ok(x/y)																								// 5) ... возвращаем из функции Ok(результат деления x / y)
	}       																								// 6) конец условия
}         																								// 7) конец функции

fn main() {

	// способ #1: пример с делением двух чисел
	// способ #1: пример с делением двух чисел
	// способ #1: пример с делением двух чисел

	let res1 = divide(10.4, 5.0);
	println!("Что такое res1 = {:?}", res1);

	// проверка результата работы функции деления
	match res1 {
		Ok(result) => {
			println!("Деление успешно, ответ = {}", result);
		}
		Err(message) => {
			println!("Деление было с ошибкой = {}", message);
		}
	}

	// способ #2: кидать панику, если была ошибка
	// способ #2: кидать панику, если была ошибка
	// способ #2: кидать панику, если была ошибка

	let girl_age = 22;
	if girl_age >= 18 {
		println!("Хорошая девушка, продолжаем знакомство...");
	} else {
		panic!("Паника! Прекращаем работу программы!");
	}

	// как работать с файлами
	// как работать с файлами
	// как работать с файлами

	let path: &str = "lines.txt";
	let output_file = File::create(path);

	let mut output_file = match output_file {
		Ok(result) => result,
		Err(message) => panic!("Error create file: {}", message),
	};

	write!(output_file, "Hello World\nHello User\nHello Universe").expect("Error write data to file!");

	let input_file = File::open(path).unwrap();
	let input_buffer = BufReader::new(input_file);

	for buffer in input_buffer.lines() {
		println!("Строка = {}", buffer.unwrap());
	}

	/*

		5) output_file это файловый дескриптор, структура сложного типа (имя файла, путь, свойства и проч.)
		6) сразу после создания дескриптора мы его проверяем, если все ок, то пересоздаем мутабельным и используем
		7) write!() позволяет записывать в файл текстовые строки (\n это переход на новую строку)
		8) expect() определяем сообщение, которое мы получим если была оибка записи
		9) нам не надо перед записью строк открывать файл, а в конце закрывать его (не наша забота)
		10) input_file это файловый дескриптор, но не для создания, а для чтения файла
		11) unwrap() это функция, которая заставляет Rust пропустить match и сразу положить в переменную Ok(result)
		12) input_buffer это переменная для чтения всего содержимого файла
		13) внутри цикла for переменная buffer проходит по всем линиям input_buffer
		12) unwrap() это функция, которая заставляет Rust пропустить match и сразу положить в переменную Ok(result)

	*/

}

/*

	13) таким образом в Rust нет исключений, т.е. участков опасного кода
	14) если мы контролируем опасные строки через Result, то код будет хороший

*/
