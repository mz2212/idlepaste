extern crate reqwest;
extern crate select;
extern crate url;

use std::io::{self, Read, Write};
use std::thread::sleep;
use std::time;
use select::document::Document;
use select::predicate::{Class, Predicate, Name};

fn main() {
	let site = "https://pastebin.com";
	let delay = time::Duration::from_secs(1);
	loop {
		let mut response = reqwest::get(site).unwrap();

		let mut body = String::new();
		response.read_to_string(&mut body).unwrap();
		let mut dom = Document::from(body.as_str());

		let pre_node = dom.find(Class("right_menu")
			.descendant(Name("a")))
			.next()
			.unwrap();

		let link = pre_node.attr("href").unwrap();
		sleep(delay);
		response = reqwest::get(format!("{}/raw{}", site, link).as_str()).unwrap();
		body = String::new();
		response.read_to_string(&mut body).unwrap();
		slow_print(body);
		sleep(delay);
	}
}

fn slow_print(data: String) {
	let delay = time::Duration::from_millis(125); // ~120 WPM
	for c in data.chars() {
		print!("{}", c);
		io::stdout().flush().unwrap();
		sleep(delay);
	}
	println!();
}
