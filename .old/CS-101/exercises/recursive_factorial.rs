fn factorial(n: u32) -> u32 {
	if n == 0 {
		return 1;
	}
	factorial(n - 1) * n
}

fn main() {
  println!("{:?}", factorial(10));
}
