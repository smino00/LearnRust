fn main() {
	let guess: u32 = "42".parse().expect("Not a number!");
	
	let x = 2.0; // f64
	let y: f32 = 3.0; // f32

	let sum = 5 + 10;

	let difference = 95.5 - 4.3;

	let product = 4 * 30;

	let quotient = 56.7 / 32.2;

	let truncated = -5 / 3; // Results in -1

	let remainder = 43 % 5;

	let t = true;

	let f: bool = false;

	let c = 'z';
	let z: char = 'ℤ';
	let heart_eyed_cat = '😻';

	let tup: (i32, f64, u8) = (500, 6.4, 1);
	let (x, y, z) = tup; //x = 500, y = 6.4, z = 1
	println!("The value of y is: {y}");
	
	let x: (i32, f64, u8) = (500, 6.4, 1);
	let five_hundred = x.0; //500
	let six_point_four = x.1; //6.4
	let one = x.2; //1

	let a = [1, 2, 3, 4, 5];

	let months = ["January", "February", "March", "April", "May", "June", "July",
		"August", "September", "October", "November", "December"];

	let a: [i32; 5] = [1, 2, 3, 4, 5];
	
	let a = [3; 5]; // equal [3, 3, 3, 3, 3]
	
	let a = [1, 2, 3, 4, 5];
	let first = a[0]; //1
	let second = a[1]; //2

	// do panics!
	let a = [1, 2, 3, 4, 5];

	println!("Please enter an array index.");
		   // 配列の何番目の要素にアクセスするか指定してください

	let mut index = String::new();

	io::stdin()
		.read_line(&mut index)
		.expect("Failed to read line");
			  // 値の読み込みに失敗しました

	let index: usize = index
		.trim()
		.parse()
		.expect("Index entered was not a number");

	let element = a[index];

	println!("The value of the element at index {index} is: {element}");
}
