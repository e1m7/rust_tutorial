
#![allow(unused)]
use std::time::Instant;
use std::thread;

fn main() {

	// Найти сумму 1 - 1_000_000 одним циклом
	// Найти сумму 1 - 1_000_000 одним циклом
	// Найти сумму 1 - 1_000_000 одним циклом

	println!("Сумма через простой цикл:");
  let start = Instant::now();
  let sum1 = (1..=1_000_000).sum::<u64>();
  let end = Instant::now();
  println!("Сумма чисел от 1 до миллиона: {}", sum1);
  println!("Время выполнения: {:?}", end.duration_since(start));


  // Найти сумму в два потока
  // Найти сумму в два потока
  // Найти сумму в два потока

	println!("\nСумма через потоки:");
	let start = Instant::now();
  let handle_sum1 = thread::spawn(|| {
    let sum1 = (1..=500_000).sum::<u64>();
    sum1
  });

  let handle_sum2 = thread::spawn(|| {
    let sum2 = (500_001..=1_000_000).sum::<u64>();
    sum2
  });

  let sum1 = handle_sum1.join().unwrap();
  let sum2 = handle_sum2.join().unwrap();
  let sum_final = sum1 + sum2;
  let end = Instant::now();

  println!("Сумма чисел от 1 до миллиона: {}", sum_final);
  println!("Время выполнения: {:?}", end.duration_since(start));

}

/*

		Сумма через простой цикл:
		Сумма чисел от 1 до миллиона: 500000500000
		Время выполнения: 43.513205ms

		Сумма через потоки:
		Сумма чисел от 1 до миллиона: 500000500000
		Время выполнения: 31.840401ms

*/
