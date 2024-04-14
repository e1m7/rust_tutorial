
#![allow(unused)]

fn main() {

	let arr1: [i32; 4] = [1,-2,3,4];
	println!("first element = {}", arr1[0]);
	println!("second element = {}", arr1[1]);
	println!("length = {}", arr1.len());
	println!("====================================");

	let arr2: [i32; 9] = [-2,4,3,55,66,1,2,3,4];
	println!("first element = {}", arr2[0]);
	println!("second element = {}", arr2[1]);
	println!("length = {}", arr2.len());
	println!("====================================");

	let arr3: [i32; 10] = [1, 51, 3, 4, 5, 6, 7, 8, 9, 11];
	let mut loop_index: usize = 0;

	loop {
		
		if loop_index == 10 {										// условие 1
			break;
		}
		
		if arr3[loop_index] % 2 == 0 {					// условие 2
			loop_index = loop_index + 1;
			continue;
		}
		
		println!("index = {}, value = {}", loop_index, arr3[loop_index]);
		loop_index = loop_index + 1;

	} // loop

	println!("====================================");

	loop_index = 0;
	while loop_index < arr3.len() {
		println!("index = {}, value = {}", loop_index, arr3[loop_index]);
		loop_index += 1;
	} // while

	println!("====================================");

	let arr4: [i32; 10] = [9,8,7,6,5,4,3,2,1,0];
	let mut loop_idx: usize = 0;

	for val in arr4.iter() {
		println!("element {}", val);
	} // for

	// println!("val = {}", val);

}

/*

	1) определение массива: let name: [type; maximum-elements] = [1,2,3];
	2) loop бесконечно повторяет то, что находится внутри его тела
	3-1) break => немедленный выход из тела цикла
	3-2) contunue => немедленный переход на начало тела
	4) условие 1 = заканчивает цикл, если loop_index = 10
	5) условие 2 = увеличивает loop_index и начинает тело заново
	6) while продолжает тело цикла, пока верно условие
	7) for..in проходит по всему массиву, закидывая его значения в переменную

*/