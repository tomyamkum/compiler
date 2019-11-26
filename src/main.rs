fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() != 2 {
		println!("引数の数が正しくありません");
	} else {
		let num: i32 = args[1].parse().unwrap();
		println!(".globl _main");
		println!("_main:");
		println!(" mov ${}, %rax", num);
		println!(" retq");
	}
	return
}
