//Добавить sqlite- это точно потребуеться

/*	md5sum
File: "for tests\\test file.txt" 	 		5d41402abc4b2a76b9719d911017c592
File: "for tests\\file (3).txt" 	 		410aafaed362fbdbeac4006be9f49c57
File: "for tests\\file (4).txt" 			ed4e1817915d545d85c977437bf95770
File: "for tests\\folder 1\\file (1).txt" 	713fa9d7e765ad41a2695c11098179f3
File: "for tests\\folder 1\\file (2).txt" 	8149a531c792ec959f8c548e5ced955b
File: "for tests\\folder 2\\file (1).txt" 	713fa9d7e765ad41a2695c11098179f3
File: "for tests\\folder 2\\file (3).txt" 	410aafaed362fbdbeac4006be9f49c57
*/

use std::path;
// use std::path::Path;
use std::collections::HashSet;

// const TEST_FILE_NAME: &str = "test file.txt";
const TEST_FILE_PATH: &str = "for tests"; // "."

type TagsList<'a> = Vec<&'a str>; //важен id, по этому Vec
//Ожидался lifetime параметр, почему?
// https://stackoverflow.com/questions/45339659/why-does-this-rust-type-alias-need-a-lifetime-parameter

#[derive(Debug)]
struct FileWithTags<'a> {
	check_sum: &'a str, //Ожидался lifetime параметр, почему?
	//HashSet or B-treeSet?
	tags: HashSet<u32>, 
	path: &'a path::Path // с & не - должен быть в конце т.к. `[u8]` does not have a constant size known at compile-time
}


fn main() {
//======initialize=======
	let mut all_tags_list: TagsList = TagsList::new();
	all_tags_list.push("home");
	all_tags_list.push("food");
	all_tags_list.push("fun");
	all_tags_list.push("book");
	all_tags_list.push("other");
//--
	let mut set_tags_of_firt_row = HashSet::new();
		set_tags_of_firt_row.insert(3);
		set_tags_of_firt_row.insert(4);


	let firt_row = FileWithTags{
		check_sum:"5d41402abc4b2a76b9719d911017c592",
		tags: set_tags_of_firt_row, //возможно порождаеться множество копий одинаковых set'ов
		path: path::Path::new("for tests\\test file.txt")
	};
//======initialize=end===

	let mut db: Vec<FileWithTags> = Vec::new();
		db.push(firt_row);

	println!("Hello, world!");
}
