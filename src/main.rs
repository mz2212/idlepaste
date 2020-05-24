extern crate reqwest;
extern crate select;
extern crate url;

use std::io::{self, Read, Write};
use std::thread::sleep;
use std::time;
use select::document::Document;
use select::predicate::{Class, Predicate, Name};

fn main() {
	let site = "https://gist.github.com";
	let delay = time::Duration::from_secs(1);
	loop {
		let mut response = reqwest::get(format!("{}/discover", site).as_str()).unwrap();

		let mut body = String::new();
		response.read_to_string(&mut body).unwrap();
		let dom = Document::from(body.as_str());

		let mut pre_node = dom.find(Class("d-inline-block")
			.descendant(Name("a")))
			.last()
			.unwrap();

		let mut link = pre_node.attr("href").unwrap();
		sleep(delay);
		response = reqwest::get(format!("{}/{}", site, link).as_str()).unwrap();
		body = String::new();
		response.read_to_string(&mut body).unwrap();
		let dom = Document::from(body.as_str());
		pre_node = dom.find(Class("file-actions")
			.descendant(Name("a")))
			.next()
			.unwrap();

		link = pre_node.attr("href").unwrap();
		response = reqwest::get(format!("{}/{}", site, link).as_str()).unwrap();
		body = String::new();
		response.read_to_string(&mut body).unwrap();
		slow_print(body);
		sleep(delay);
	}
}

fn slow_print(data: String) {
	let delay = time::Duration::from_millis(30);
	for c in data.chars() {
		print!("{}", c);
		io::stdout().flush().unwrap();
		sleep(delay);
	}
	println!();
}
