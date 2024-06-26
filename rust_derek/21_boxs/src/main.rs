
#![allow(unused)]

/*

	1) указатели это способ ссылаться на данные, расположенные в памяти
	2) указатели это переменные, которые модержат адреса памяти, где хранятся данные
	3) например, по адресу 0x1234 есть данное = 100 и есть указатель name1 = 0x1234
	4) в RUST есть несколько разных типов указателей ос своими особенностями  

	а) указатели на ссылки (References)
		- это ссылка типа &T, T это тип данных, на который она ссылаются
		- указатели на ссылки не имеют владения
		- указатели на ссылки обеспечивают безопасность во время выполнения 
		- указатели на ссылку не дают изменить значение, только получить его копию
		- указатели на ссылку имеют время жизни (контролируется компилятором)
		- указатели на ссылку не могут быть 0-ыми

	б) умные указатели (Smart Pointers)
		- есть три типа умных указателей: Box, Rc, Arc
		- они предоставляют владение данными и позволяют распределить данные между владельцами
		- "Box" для выделения памяти в куче для единственного владельца
		- "Rc" для подсчета ссылок для нескольких владельцев, но им нельзя изменять данные
		- "Arc" для обеспечения работы в параллельных программных средствах

	в) указатели на функцию (Function Pointers)
		- это ссылка типа fn, она позволяет передавать функции как аргументы функции
		- используются в замыканиях для реализации коллбэков (см. пример closures)

*/


// =============================для указателя Box==========================

struct Person {
  name: String,
  age: u32,
}

fn create_person(name: String, age: u32) -> Box<Person> {
  Box::new(Person { name, age })
}

// =============================для указателя Fn==========================

fn add(a: i32, b: i32) -> i32 {
   a + b
}

// =======================================================================


fn main() {

	// а) указатели на ссылки
	// а) указатели на ссылки
	// а) указатели на ссылки

	let x = 5;																			// x = 5 (значение)
	let reference = &x;															// reference = указатель на x
	println!("Value of x: {}", x);									// чему равно x? (5)
	println!("Value of reference: {}", reference); 	// чему равно reference? (5)

	// более правильно задать ссылку: let reference: &i32 = &x;
	// x = reference, т.к. при выводе на экран RUST автоматически &x ==> x (5)
	// такой механизм преобразования ссылки в значение, на которое она ссылается, называется дереференциация
	// println!("Value of reference: {}", reference); ==> произошла дереференциация
	// нет способа через код вывести на экран адрес, который внутри ссылки, всегда будет преобразование

	// б) умные указатели (Box)
	// б) умные указатели (Box)
	// б) умные указатели (Box)

	// задача: у нас есть некотоаря структура Person, состоит из двух полей (строка и число)
	// задача: надо создать функцию, которая создает такую структуру в куче (новый экземпляр структуры) 
	// задача: мы будем общатся к элементам экземпяра через Box, чтобы гарантировать, что у него один хозяин

	let alice_box = create_person(String::from("Alice"), 30);
	println!("Name: {}", alice_box.name);
  println!("Age: {}", alice_box.age);

  // в) указатель на функцию
  // в) указатель на функцию
  // в) указатель на функцию

  let func_pointer: fn(i32, i32) -> i32 = add;
  let result = func_pointer(10, 20);
  println!("Result of add function: {}", result);

}
