
#![allow(unused)]

fn main() {

	// конвертация из типа в тип
	let int1_u8: u8 = 5;
	let int2_u8: u8 = 6;
	let int3_u32: u32 = (int1_u8 as u32);
	println!("int1_i8 = {}", int1_u8);
	println!("int2_i8 = {}", int2_u8);
	println!("int3_u32 = {}", int3_u32);

	println!("=========================");

	// определение нового типа данных Days (дни недели)
	enum Days {
		Monday,
		Tuesday,
		Wednesday,
		Thursday,
		Friday,
		Saturday,
		Sunday
	}

	// определение функции для нового типа данных
	impl Days {

		// -----------------------------------------------------------------------------
		fn is_weekend(&self) -> bool {
			match self {
				Days::Saturday => true,
				Days::Sunday => true,
				_ => false
		  	}
			}
		// -----------------------------------------------------------------------------

		// -----------------------------------------------------------------------------
		fn is_party(&self) -> bool {
			match self {
				Days::Friday => true,
				_ => false
			}
		}
		// -----------------------------------------------------------------------------

	} // impl

	let first:Days = Days::Monday;

	println!("first day Weekend = {}", first.is_weekend());
	println!("first day Patry = {}", first.is_party());

	println!("=========================");

	let second:Days = Days::Friday;

	println!("second day Weekend = {}", second.is_weekend());
	println!("second day Patry = {}", second.is_party());

	println!("=========================");

	/*
	let simple_val: u8 = 10;
	println!("Weekend = {}", simple_val.is_weekend());
	println!("Weekend = {}", simple_val.is_party());
	*/

}

/*

	1) enum это новый тип данных, который мы можем создавать сами
	2) мы сами решаем какие значения принимает этот тип данных
	3) мы сами решаем какие функции работают с этим типом данных
	4) функции надо имплементировать в код (implementation = выполнение)

*/
