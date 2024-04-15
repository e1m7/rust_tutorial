
#![allow(unused)]

/*

	1) векторы это динамически расширяемые массивы с элементами одного типа
	2) векторы это структуры формата Vec<T>, где T = тип элементов, из которых состоит вектор
	3) векторы динамически расширяются: элементы можно добавлять, удалять в процессе работы 
	4) векторы однородны: только один тип данных, например Vec<i32> содержит значеения i32
	5) векторы владеют своими данными: сами контролируют выделение и потерю памяти под элементы

*/


fn main() {

	// создание пустого вектора
	let mut numbers1: Vec<i32> = Vec::new();
	numbers1.push(10);
	numbers1.push(20);
	numbers1.push(30);
	println!("numbers1 = {:?}", numbers1);

	// создание вектора с начальными значениями
	let mut numbers2: Vec<i32> = vec![1,2,3,4,5,6,7];
	numbers2.push(100);
	numbers2.push(200);
	numbers2.push(300);
	println!("numbers2 = {:?}", numbers2);

	// количество элементов вектора
	println!("length element of numbers1 = {}", numbers1.len());
	println!("length element of numbers2 = {}", numbers2.len());

	// какова capacity вектора (сколько еще можно добавить элементов, а потом будет увеличена память)
	let capacity1 = numbers1.capacity();
	let capacity2 = numbers2.capacity();
	println!("capacity1 = {}", capacity1);
	println!("capacity2 = {}", capacity2);


	// доступ к элементам вектора
	println!("first element numbers1 = {}", numbers1[0]);
	println!("first element numbers2 = {}", numbers2[0]);

	// итерация по элементам вектора через for .. in
	for num1 in &numbers1 {
		println!("{}", num1);
	}

	for num2 in &numbers2 {
		print!("{} | ", num2);
	}
	println!();

	// итерация по элементам вектора через iter()
	for num in numbers1.iter() {
    println!("{}", num);
	}

	// удаление элемента из вектора
	let delete_elem0 = numbers2.remove(0);
	println!("delete_elem0 = {}", delete_elem0);

	for num2 in &numbers2 {
		print!("{} | ", num2);
	}
	println!();

	// удаление элемента с конца вектора
	let last_elem = numbers2.pop();
	println!("last_elem = {:?}", last_elem);					// Some(300) = такое значение

	for num2 in &numbers2 {
		print!("{} | ", num2);
	}
	println!();
	
	// безопасное обращение к элементу по индексу
	// numbers2 = 2 | 3 | 4 | 5 | 6 | 7 | 100 | 200
	//            0   1   2   3   4   5   6     7

	// пробуем 6-ой элемент
	if let Some(elem) = numbers2.get(6) {
		println!("Да, есть 6-ой элемент = {}", elem);
	} else {
		println!("Нет, такого элемента нет");
	}

	// пробуем 10-ый элемент
	if let Some(elem) = numbers2.get(10) {
		println!("Да, есть 10-ой элемент = {}", elem);
	} else {
		println!("Нет, такого элемента нет");
	}

	println!("========================");

	// некоторые извращения
	// некоторые извращения
	// некоторые извращения

	// 1) создали новый тип данных = целое число || дробное число || строка
	enum Value {
    Int(i32),
    Float(f64),
    Text(String),
	}

	// 2) создали новый пустой векто и добавили туда два элемента
	let mut data: Vec<Value> = Vec::new();
	data.push(Value::Int(42));
	data.push(Value::Text(String::from("hello")));

	// 3) при выводе вектора надо делать сопоставления
	for value in &data {
    match value {
        Value::Int(num) => println!("Integer: {}", num),
        Value::Float(num) => println!("Float: {}", num),
        Value::Text(text) => println!("Text: {}", text),
    }
	}

}

