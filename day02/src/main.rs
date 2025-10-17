// Day 2: Ownership & Borrowing examples

fn takes_ownership(s: String) {
	println!("takes_ownership received: {}", s);
} // s is dropped here

fn makes_copy(x: i32) {
	println!("makes_copy received: {}", x);
} // x is Copy, so caller keeps ownership

fn borrow_string(s: &String) {
	println!("borrow_string saw: {}", s);
}

fn borrow_string_mut(s: &mut String) {
	s.push_str(" (modified)");
}

fn main() {
	// Ownership move
	let s1 = String::from("hello");
	takes_ownership(s1);
	// s1 is moved, cannot be used anymore

	// To keep a copy, clone explicitly
	let s2 = String::from("world");
	let s3 = s2.clone();
	println!("s2 cloned into s3: s2='{}', s3='{}'", s2, s3);

	// Copy types (stack-allocated) are copied, not moved
	let x = 42;
	makes_copy(x);
	println!("x after makes_copy: {}", x);

	// Immutable borrow
	let s4 = String::from("borrow me");
	borrow_string(&s4);
	println!("s4 still usable: {}", s4);

	// Mutable borrow
	let mut s5 = String::from("mutable");
	borrow_string_mut(&mut s5);
	println!("s5 after mutable borrow: {}", s5);

	// Demonstrate references and scopes
	let mut s6 = String::from("scoped");
	{
		let r1 = &s6; // immutable borrow
		let r2 = &s6; // another immutable borrow
		println!("r1='{}' r2='{}'", r1, r2);
	} // r1 and r2 go out of scope here

	let r3 = &mut s6; // now mutable borrow allowed
	r3.push_str("!");
	println!("s6 via r3: {}", r3);

	println!("Day 2: ownership & borrowing demo complete.");
}
