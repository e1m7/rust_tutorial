
#![allow(unused)]

/*

	1) closures (замыкания) это анонимные функции, которые можно сохранить в переменной
	2) замыкания можно передавать как аргументы в другие функции, а также возвращать из функций как результат
	3) замыкания "захватывают" из окружающего контекста переменные и используют их в своем теле
	4) замыкания позволяют писать гибкий и компактный код

*/

fn main() {

	// пример замыкания
	// пример замыкания
	// пример замыкания

	let closure1 = |x| x + 1;
	let result = closure1(5);
	println!("result = {}", result);
	println!("==============================");

	// пример замыкания
	// пример замыкания
	// пример замыкания
	
	let mut a = 5;
	let mut closure2 = || {
		a = a + 1;
		println!("a внутри замыкания = {}", a);
	};
	closure2();
	println!("a внутри основной области = {}", a);
	println!("==============================");

	// когда использование замыкания удобнее чем функция
	// когда использование замыкания удобнее чем функция
	// когда использование замыкания удобнее чем функция

	// задача: отфильтровать значения вектора, взяв те, которые больше 10
	let numbers = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
	let my_limit = 10;

	let new_numbers: Vec<_> = numbers
		.iter()
		.filter(|&x| *x > my_limit)
		.collect();

	println!("new_numbers = {:?}", new_numbers);
	println!("==============================");

	// 5) замыкания это функции без имени (как правило, оно = переменной, но имени у функции нет)
	// 6) формат создания замыкания: let name = |param| -> return_type { body };

	// замыкание: может ли человек голосовать?
	// замыкание: может ли человек голосовать?
	// замыкание: может ли человек голосовать?

	let can_vote = |age: i32| -> bool {
		age >= 18
	};
	println!("Can vote Polina: {}", can_vote(20));
	println!("Can vote Marina: {}", can_vote(15));
	println!("Can vote Dima: {}", can_vote(50));
	println!("==============================");

	// замыкание: как можно получать доступ к переменным вне замыкания
	// замыкание: как можно получать доступ к переменным вне замыкания
	// замыкание: как можно получать доступ к переменным вне замыкания

	let mut val1 = 10;
	let mut val2 = 20;
	let mut can_print = || {
		println!("val1 = {}", val1);
		println!("val2 = {}", val2);
		val1 = val1 + 5;
	};
	can_print();
	println!("val1 = {}", val1);
	println!("val2 = {}", val2);
	println!("==============================");

	// замыкания: передать в функцию
	// замыкания: передать в функцию
	// замыкания: передать в функцию

	// допустим, что мы хотим создать функцию, куда приходят два числа и функция
	fn my_function<T>(a: i32, b: i32, action: T) -> i32 where T: Fn(i32, i32) -> i32 {
		action(a,b)
	}

	let x = 10;
	let y = 20;
	let add = |a: i32, b: i32| a + b;
	let sub = |a: i32, b: i32| a - b;
	let div = |a: i32, b: i32| a / b;
	let mul = |a: i32, b: i32| a * b;
	println!("a + b = {}", my_function(x, y, add));
	println!("a - b = {}", my_function(x, y, sub));
	println!("a / b = {}", my_function(x, y, div));
	println!("a * b = {}", my_function(x, y, mul));

}
