use std::io;
use std::char;
pub const MAX: char = char::MAX; 

fn main() {
    let input = input_taker(); // in tuple code word and message
    let answer = encoder(input); 
    println!("{}", answer);
}

fn input_taker() -> (String, String) {
	println!("Input message");
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    let message = buffer.trim().to_string();
    println!("Input code");
    buffer = String::new();
    stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    let code = buffer.trim().to_string();
    return(message,code);
}

fn encoder(a: (String, String)) -> String {
	let message: Vec<char> = (a.0).chars().collect();
	let code: Vec<char> = (a.1).chars().collect();
	let length = message.len();
	let encode = encoder_constructor(code,length);
	let limit = MAX as u32;
	let mut encode_vec: Vec<char> = Vec::new();
	let mut position = 0; 
    for charecter in message {
		let mut encode_char = charecter as u32 + encode[position] as u32;
		if encode_char > limit {
			encode_char = encode_char - limit;
		} 
		unsafe {encode_vec.push(char::from_u32_unchecked(encode_char));}
		position += 1;
	}
	println!("{:?}", encode_vec);
	
	let encode_message = encode_vec.into_iter().collect();
	
	return(encode_message);
}
    
fn encoder_constructor(code: Vec<char>, length: usize) -> Vec<char> {
	let mut encode: Vec<char> = Vec::new();
    if code.len() < length {
	    let mut position = 0;
	    let mut end = 0;
		loop {
			encode.push(code[position]);
			position += 1;
			end +=1;
			if position == code.len(){
				position = 0
			}
		    if end == length {
				break
			}
		}
	    return(encode);
	
	}else {
		return(code);
	}
}

