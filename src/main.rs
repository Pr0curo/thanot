fn main() {
    let x=12;

    add_one(x);
    add_two(x);
    add_three(x);

    println!("Hello, world!");

    let test = r#"         the quick brown fox    junps over the            lazy wolve       "#.to_string();
    println!("{:?}", test);
    let test2 = test.trim();
    println!("{:?}", test2);
    let words: Vec<&str>=test.split_whitespace().collect();
    println!("{:?}", words);


    let v : Vec<&str>= test2.splitn(2,' ').collect();
    println!("{:?}", v);
    if v.len()==2 {
    	if v[0] == "the"{
    		println!("befehl erkannt {:?}", v[0]);	
    	} else {
    		println!("kein befehl erkannt {:?}", v[0]);
    	}
    }
}

fn add_one(number:i32) -> i32{
	number+1
}

fn add_two(number:i32) -> i32{
	number+2
}

fn add_three(number:i32) -> i32{
	number+3
}

