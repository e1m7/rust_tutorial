
// название модуля
mod restaurant_order {

	// задаем структуру данных 
	pub struct Pizza {
		pub basic: String,				// основа 					(неизменно для всех пицц в ресторане)
		pub cheese: String,				// название сыра 		(неизменно для всех пицц в ресторане)
		pub additive: String,			// название добавки
	}

	// задаем функциональность модуля
	impl Pizza {
		pub fn create(additive: &str) -> Pizza {
			// после Pizza {} нет точки с запятой (это выражение, а не оператор)
			// созданный экземпляр структуры вернется из функции назад, где его вызывали
			Pizza {
				basic: String::from("standart"),
				cheese: String::from("mozzarella"),
				additive: String::from(additive),
			}												
		}
	} // impl Pizza

	// задаем основной блок модуля (это будут три функции)
	// 1) seat_table() = приватная функция модуля, доступ только тут				(внутренняя кухня модуля)
	// 2) start_table() = приватная функция модуля, доступ только тут				(внутренняя кухня модуля)
	// 3) create_order() = публичная функция модуля, она будет видна из модуля
	// суть работы restaurant_order: 1) сказать привет, 2) сделать пиццу, 3) поставить ее на стол

	// создаем модуль #2 внутри модуля #1
	// restaurant_order (#1) => help_customer (#2)
	pub mod help_customer {

		fn seat_table() {
			println!("Hello, customer!");
		}

		fn start_table(pizza: super::Pizza) {
			println!("Your order: {} completed!", pizza.additive);
		}

		// <== точка входа в модуль, которая все запускает
		// <== точка входа в модуль, которая все запускает
		// <== точка входа в модуль, которая все запускает
		pub fn create_order() {				
			seat_table();
			let current: super::Pizza = super::Pizza::create("ananas");
			start_table(current);
		}		
		// <== точка входа в модуль, которая все запускает
		// <== точка входа в модуль, которая все запускает
		// <== точка входа в модуль, которая все запускает

	} // mod help_customer()

} // mod restaurant_order




pub fn order_food() {
	// место :: каталог :: модуль 1 :: модуль 2 :: create_order()
	crate::restaurant::restaurant_order::help_customer::create_order();
}

/*

	src
		restaurant
			mod.rs
		main.rs

		crate 											(текущее место)
		restaurant 									(название каталога)
			модуль restaurant_order 	(1-ый модуль)
				модуль help_customer 		(2-ой модуль)
					функция create_order  <== что надо запустить

*/