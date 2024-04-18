
#![allow(unused)]
use std::fmt::Debug;

fn main() {

	// примеры на тип ссылок Box
	// примеры на тип ссылок Box
	// примеры на тип ссылок Box

	let box_ref_int1: Box<i32> = Box::new(10);
	println!("box_ref_int1 = {}", box_ref_int1);

	// структура типа "двоичное дерево"
	// а) возможны ошибки структуры из-за строк в виде ключей
	// б) RUST не любит пустые (нулевые) значения в структурах

	// struct TreeNode1<T> {
	// 	pub left: TreeNode1<T>,
	// 	pub right: TreeNode1<T>,
	// 	pub key: T,
	// }

	struct TreeNode<T> {
		pub left: Option<Box<TreeNode<T>>>,
		pub right: Option<Box<TreeNode<T>>>,
		pub key: T,
	}

	// комментарий к структуре
  // 1) это структура данных типа "бинарное дерево"
  // 2) left, right это поля для левого или правого потомка
  // 3) Option используется чтобы указать на то, что потомок может быть узлом или отсутствовать 
  // 4) Box используется для обертывания TreeNode<T> и он будет управлять временем жизни узлов в куче
  // 5) обертывание Box обычная практика для RUST, если у структур динамический размер (они растут)
  // 6) key это поле, которое содержит ключ (типа данных T = тик, который будет хранится в узле)

  // создадим пару функций для работы с двоичным деревом
  impl<T> TreeNode<T> {
  	
  	pub fn new(key: T) -> Self {
  		TreeNode {
  			left: None,
  			right: None,
  			key: key,
  		}
  	}

  	pub fn left(mut self, node: TreeNode<T>) -> Self {
  		self.left = Some(Box::new(node));
  		self
  	}

  	pub fn right(mut self, node: TreeNode<T>) -> Self {
  		self.right = Some(Box::new(node));
  		self
  	}

  	// комментарий к коду
  	// 1) pub fn new(key: T) -> Self {...} создаем новый узел дерева с заданным ключом
  	// 2) это ассоциированный метод структуры, т.е. его можно вызывать через тип или через экземпляр 
  	// 3) new() принимает значение типа T, а возвращает экземпляр TreeNode<T> с ключом и двумя нулями слева и справа
  	// 4) pub fn left(mut self, node: TreeNode<T>) -> Self {...} метод добавления левого потомка
  	// 5) он принимает текущий узел (как мутательный) и новый узел node, который становится left
  	// 6) он создает left как Some(Box::new(node)) и возвращает измененный узел как результат
  	// 7) pub fn right(mut self, node: TreeNode<T>) -> Self {...} метод добавления правого потомка
  	// 8) все методы возвращат self, что позволяет их использовать в цепочке
  	// 9) let tree = TreeNode::new(5).left(TreeNode::new(3)).right(TreeNode::new(7))

  }

  // создаем корневой узел
  let my_node = TreeNode::new(1)
  	.left(TreeNode::new(2))
  	.right(TreeNode::new(3));

  // как вывести дерево на экран
  // 1) нам нужна рекурсивная функция, которая будет проходить по дереву
  // 2) ее нужно имплементировать в функции бинарного дерева и вызвать на главном узле

  impl<T: std::fmt::Debug> TreeNode<T> {
    pub fn display(&self, depth: usize) {
        if let Some(ref right) = self.right {
            right.display(depth + 1);
        }
        println!("{}{:?}", "  ".repeat(depth), self.key);
        if let Some(ref left) = self.left {
            left.display(depth + 1);
        }
    }
	}

	// вывод дерева
	// my_node.display(0);

	// более сложное дерево
	let my_node1 = TreeNode::new(1)
	    .left(TreeNode::new(2)
	        .left(TreeNode::new(4)
	            .right(TreeNode::new(5)))
	        .right(TreeNode::new(3)
	            .left(TreeNode::new(6)
	                .right(TreeNode::new(7)))));


	my_node1.display(0);

}
