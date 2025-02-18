fn clone_and_modify(s: &String) -> String {
	// Your code here
    let mut cloned_string = s.clone(); // Clone the original string
    cloned_string.push_str("World!"); // Modify the cloned string
    cloned_string // Return the modified clone
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}
