extern crate crypto;
//extern crate regex;

use std::fs::File;
use std::io::prelude::*;
//use std::io::*;
use std::cmp;
use std::collections::HashSet;
use std::collections::HashMap;
use crypto::md5::Md5;
use crypto::digest::Digest;
//use regex::Regex;

#[allow(dead_code)]
fn day1()
{
	let input = "()()(()()()(()()((()((()))((()((((()()((((()))()((((())(((((((()(((((((((()(((())(()()(()((()()(()(())(()((((()((()()()((((())((((((()(()(((()())(()((((()))())(())(()(()()))))))))((((((((((((()())()())())(())))(((()()()((((()(((()(()(()()(()(()()(()(((((((())(())(())())))((()())()((((()()((()))(((()()()())))(())))((((())(((()())(())(()))(()((((()())))())((()(())(((()((((()((()(())())))((()))()()(()(()))))((((((((()())((((()()((((()(()())(((((()(()())()))())(((()))()(()(()(()((((()(())(()))(((((()()(()()()(()(((())())(((()()(()()))(((()()(((())())(()(())())()()(())()()()((()(((()(())((()()((())()))((()()))((()()())((((()(()()(()(((()))()(()))))((()(((()()()))(()(((())()(()((()())(()(()()(()())(())()(((()(()())()((((()((()))))())()))((()()()()(())()())()()()((((()))))(()(((()()(((((((())()))()((((()((())()(()())(())()))(()(()())(((((((())))(((()))())))))()))())((())(()()((())()())()))))()((()()())(())((())((((()())())()()()(((()))())))()()))())(()()()(()((((((()()))())()))()(((()(((())((((()()()(()))())()()))))())()))())((())()())(((((())())((())())))(((())(((())(((((()(((((())(()(()())())(()(())(()))(()((((()))())()))))())))((()(()))))())))(((((())()))())()))))()))))(((()))()))))((()))((()((()(()(())()())))(()()()(())()))()((((())))))))(())(()((()()))(()))(()))(()((()))))))()()((((()()))()())()))))))()()()))(()((())(()))((()()()())()(((()((((())())))()((((()(()))))))())))()()())()))(()))))(()())()))))))((())))))))())()))()((())())))(()((()))()))(())))))(()))()())()()))((()(()))()()()()))))())()()))())(())()()))()))((()))))()()(()())))))()()()))((((()))()))))(()(())))(()())))((())())(()))()))))()())))()())()())))))))))()()))))())))((())((()))))())))(((()())))))))(()))()()))(()))()))))()())))))())((((()())))))))())))()()))))))))()))()))))()))))))(())))))))))())))))))))))))))())())((())))))))))()))((())))()))))))))())()(()))))))())))))()()()())()(()()()(()())(()))()()()(()())))())())))()))))())))))))()()()()())(())())()())()))))(()()()()()))))()))())())))((()())()())))()))()))))(()())))()))))))))(((()))()()))))))))))))))))))))(()))(()((()))())))())(()))(()(()(())))))()(()))()))()()))))))))))))()((()())(())())()(())))))())()())((()()))))(()()))))())()(())()))))))))))))))))))))()))(()(()())))))))()()((()))()))))))((())))()))))))))((()))())()()))())()()))((()))())))))))))))(()())()))(())((()(()()))(()())(())))()())(()(())()()))))()))()(()))))))(()))))))))))(()))())))))))))())))))())))(())))))()))))(())())))))))))()(()))))()())))())(()))()())))))))))))))())()()))))()))))))())))))()))))(())(()()()()((())()))())(()))((())()))())())(())(()()))))()))(())()()((())(())))(())))()))())))))))))()(((((())())))(())()))))(())))((()))()(((((((()))))()()))(())))))()(()))))(()()))()))())))))))(()())()))))))))())))(()))())()))(())()((())())()())())(()(()))))()))))))((()())(())()()(()())))()()))(())(())(()))())))()))(()))()()))((((()))))()))((()()()))))()))()))())))(()))()))))(())))()))())()(()))()())))())))))))())))())))()()))))))(()))())())))()))()()())())))))))))))))())))()))(()()))))())))())()(())))())))))))))))))))))()()())())))))()()()((()(()))()()(())()())()))()))))()()()))))))((()))))))))()(()(()((((((()()((()())))))))))))()))())))))((())())(()))())))())))))())()()())(())))())))()())())(())))))))()()(())))()))())))())())())()))))))))()))(()()()())())())))(())())))))))()()())()))))())))())()(())())))))))()())()))(()()(())())))()(()((()()((()()(((((())(()())()))(())()))(())))(())))))))()))()))((()))()))()))))))))()))))))))((()()())(()))(((()))(())))()))((())(((())))()())))())))))((())))))(())())((((((())())()(()))()(()((()())))((())()(()(()))))(())(()()())(())))())((()(((())())))(((()())())))())()(())())((((()()))))())((()))()()()()(())(((((((()()()((()))())(()())))(())())((((()()(()))))()((())))((())()))()(((()))())))()))((()(()))(())(()((((())((((()()(()()))(((())(()))))((((()(()))(())))))((()))(()))((()(((()(()))(()(()((()(())(()(()(()(()()((()))())(((())(()(()))))(()))()()))(())))(())()(((())(()))()((((()()))))())(()))))((())()((((()(((()))())())(((()))()())((())(())())(())()(())()(()()((((((()()))))()()(((()()))))()())()(((()(()))(()(()())(()(()))))(((((()(((())())))))(((((()((()()((())())((((((()(())(()()((()()()()()()()(()()))()(((()))()))(((((((())(((()((()())()((((())(((()(())))()((()(()()()((())((()())()))()))())))())((((((()))(()(()()()))(()((()(()(()))()((()(((()()()((())(((((())()(()))())())((()(())))(()(()())(())((())())())(((()()()(())))))())(()))))))()))))))())((()()()))((()((((((()))(((()((((()()()(((()))())()(()()(((()((()()()()())()()))()()()(()(())((()))))(()))())))))))()(()()(((((())()(()(((((()((()(()()())(()((((((((()((((((())()((((()()()((()((()((((((()))((())))))))())()))((()(()))()(()()(()((())((()()((((((((((((()())(()()()))((((()((((((())(()))())(()()((()()))()(((((((()((()()((((((()(((())))((())))((((((((()()(((((((())(((((()())(((())((())()((((()(((((((()(()(((()((((((()(((()(((((((((((()()((()()(()))((()()(((()(((())))((((())()(()(((())()(()(((())(((((((((((()))())))((((((())((()()((((()())())((((()()))((())(((((()(()()(()()()((())(()((()()((((()(((((()((()(()((((()())((((((()(((((()()(()(()((((())))(())(())(())((((()(()()((((()((((()()((()((((((())))(((((()))))()))(()((((((((()(((())())(((())))(()(()((())(((()((()()(((((()((()()(((())()(()))(((((((())(()(((((()))((()((()((()))(())())((((()((((())()(()))(((()(((((((((((((((())(((((((((()))(((()(()()()()((((((()((())()((((((((()(())(((((((((((()(()((())()((()()(()(()()((((()()((())(()((()()(()()((((()(((((((())))((((())(())()(((()()((()()((((()((()(((()((())(((()()()((((()((((()()(()(()((((((((())(()(((((())(()())(((((((()())()(()((((()((())(()()())((((()()(((()((((())(())(()()(((((((((()()))()(((())(()(()((((((())(()()())(()))()()(((()(((()((())(()(((((((()(()(()((()(((((()(()((()(()((((((()((((()()((((()(((()((())(()(()((()()((((()()(())()(())(((())(()((((((((()())(((((((((()(())()((((())))()))()()(((((()()((((((())(()()(((()(()(((((((()(()(((((((())(())((((()((()(())))((((()()())(()))((()())((((()(((((()(()(())(()(()()())(((((()(((((()((((()()((((((((()()))(()((((((())((((())()(()(((()()()(((()(()(())(())(((((()(())())((((())(())(()(((()(((((())((((())())((()(((((((()(((())(()(()))(((((((((()((()((()()(()((((())(((()((())((((())(()(((()(((()(()((((()(((())(()(((()(()()(()(()((()()(()())(())())((()(()(((()(((()(((()()(((((((((()(((((((((()()(((()(((()())((((()(()(((()()()((())((((((((((())(()(((()((((()())((((()((()))(((()()()(((((()(((((((())((()())(()((((())((((((((())(()((()((((((((((()()((()((()()))(((()())()())()(((()())()()(()(()(((((((())()))(())()))())()()((())()((()((((()((()((())(((((()((((((()(())))(()))())(((()))((()()(()(((()))((((())()(((()))))()(()(())()(((((())(()(()(())(())()((()()()((((()(())((()())(()(()))(()(()(()()(())()()(()((())()((()))))()))((()(()()()()((()())(()))())()(()(((((((((())())((()((()((((((())()((((())(((())((()(()()()((())(()((())(((()((((()()((()(()(((((())()))()((((((()))((())(((()()))(((())(())()))(((((((())(())())()(())(((((()))()((()))()(()()((()()()()()())(((((((";

	let mut floor = 0;
	let mut first_basement = 0;
	for (i, c) in input.chars().enumerate()
	{
		floor = match c 
		{
			'(' => floor + 1,
			')' => floor - 1,
			_ => floor,
		};
		if floor == -1 && first_basement == 0
		{
			first_basement = i+1;
		}
	}

	println!("Final floor: {}", floor);
	println!("First entered the basement at index {}", first_basement);
}

#[allow(dead_code)]
fn day2()
{
	let mut file = File::open("input/day2.txt").unwrap();
	let mut input = String::new();
	file.read_to_string(&mut input).unwrap();
	let mut paper = 0;
	let mut ribbon = 0;
	for s in input.lines()
	{
		let v:Vec<&str> = s.split("x").collect();
		let l:i32 = v[0].parse().unwrap();
		let w:i32 = v[1].parse().unwrap();
		let h:i32 = v[2].parse().unwrap();
		let s1 = l*w;
		let s2 = w*h;
		let s3 = h*l;
		let min_side = cmp::min(s1, cmp::min(s2, s3));
		paper += 2*s1 + 2*s2 + 2*s3 + min_side;

		let vol = l*w*h;

		let p1 = l+l+w+w;
		let p2 = w+w+h+h;
		let p3 = h+h+l+l;
		let min_perim = cmp::min(p1, cmp::min(p2, p3));
		ribbon += min_perim + vol;
	}

	println!("Total area needed: {}", paper);
	println!("Total ribbon needed: {}", ribbon);
}

#[allow(dead_code)]
fn day3()
{
	let input = get_input("day3.txt");

	#[derive(PartialEq, Eq, Hash, Debug)]
	struct Pos
	{
		x:i32,
		y:i32,
	}

	impl std::fmt::Display for Pos
	{
		fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
		{
			write!(f, "({}, {})", self.x, self.y)
		}
	}

	let mut pos = Pos{x:0, y:0};
	let mut visited:HashSet<Pos> = HashSet::new();
	let mut total = 1;

	for c in input.chars()
	{
		let newpos = match c
		{
			'^' => Pos{x: pos.x, y: pos.y+1},
			'v' => Pos{x: pos.x, y: pos.y-1},
			'>' => Pos{x: pos.x+1, y: pos.y},
			'<' => Pos{x: pos.x-1, y: pos.y},
			_ => panic!("unknown char {}", c),
		};

		if !visited.contains(&newpos)
		{
			total+=1;
		}
		pos.x = newpos.x;
		pos.y = newpos.y;
		visited.insert(newpos);
	}

	println!("Visited {} houses total.", total);

	pos = Pos{x:0, y:0};
	let robopos = Pos{x:0, y:0};
	visited = HashSet::new();
	visited.insert(Pos{x:0, y:0});
	total = 1;
	let mut positions = [pos, robopos];
	let mut which:usize = 0;

	for c in input.chars()
	{
		let newpos = match c
		{
			'^' => Pos{x: positions[which].x, y: positions[which].y+1},
			'v' => Pos{x: positions[which].x, y: positions[which].y-1},
			'>' => Pos{x: positions[which].x+1, y: positions[which].y},
			'<' => Pos{x: positions[which].x-1, y: positions[which].y},
			_ => panic!("unknown char {}", c),
		};

		// Add loc to hashset
		if !visited.contains(&newpos)
		{
			total+=1;
		}

		// Update curr
		positions[which].x = newpos.x;
		positions[which].y = newpos.y;

		if which == 0
		{
			which = 1;
		}
		else
		{
			which = 0;
		}
		visited.insert(newpos);

	}

	println!("Santa and Robo-Santa delivered to {} total houses", total);
}

#[allow(dead_code)]
fn day4()
{
	let mut hasher = Md5::new();
	let key = "bgvyzdsv".as_bytes();

	for i in 0..std::u64::MAX
	{
		hasher.input(key);
		hasher.input(i.to_string().as_bytes());

		let mut output = [0; 16];
		hasher.result(&mut output);

		//let first_x = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
		let first_x = output[0] as i32 + output[1] as i32 + output[2] as i32;
		if first_x == 0
		{
			println!("{}", i);
			break;
		}
		
		hasher.reset();
	}
}

#[allow(dead_code)]
fn is_string_nice(value_str:&str) -> bool
{
	let value = value_str.as_bytes();

	let mut vowels = 0;
	let mut repeat = false;
	// let mut repeat_letter = String::new();
	for i in 0..value.len()
	{
		vowels += match value[i] as char
		{
			'a' | 'e' | 'i' | 'o' | 'u' => 1,
			_ => 0,
		};

		if i < value.len()-1
		{
			let pair = String::from_utf8(value[i..i+2].to_vec()).unwrap();
			if !repeat && value[i] == value[i+1]
			{
				repeat = true;
				// repeat_letter = pair.clone();
			}

			if pair == "ab" || pair == "cd" || pair == "pq" || pair == "xy"
			{
				//println!("{} is naughty - contains a forbidden pair!", value_str);
				return false;
			}
		}
	}

	if vowels >= 3 && repeat
	{
		//println!("{} is nice: {}, {}", value_str, vowels, repeat_letter);
		return true;
	}

	//println!("{} is naughty: {}, {}", value_str, vowels, repeat_letter);
	return false;
}

#[allow(dead_code)]
fn is_string_nice2(value_str:&str) -> bool
{
	//println!("Testing {}", value);
	let mut h = HashMap::new();

	let mut triad = false;
	let mut triad_str = String::new();
	let mut pair = false;
	let mut pair_str = String::new();
	let value = value_str.as_bytes();
	for i in 0..value.len()
	{
		if i < value.len()-2
		{
			if value[i] == value[i+2]
			{
				triad = true;
				triad_str = String::from_utf8(value[i..i+3].to_vec()).unwrap();
			}
		}

		if i < value.len() - 1
		{
			let s : &[u8] = &value[i..i+2];
			if h.contains_key(&s)
			{
				let index = h.get(&s).unwrap();
				println!("Pair {} appeared earlier at index {}", String::from_utf8(s.to_vec()).unwrap(), index);
			}
			if h.contains_key(&s) && i > h.get(&s).unwrap() + 1
			{
				pair = true;
				pair_str = String::from_utf8(s.to_vec()).unwrap();
			}
			
			if !h.contains_key(&s)
			{
				h.insert(s, i);
			}
		}

		if pair && triad
		{
			println!("  {} is nice: {}, {}", value_str, triad_str, pair_str);
			return true;
		}
	}

	if pair || triad
	{
		println!("  {} is naughty: {}, {}", value_str, triad_str, pair_str);
	}
	return false;
}

#[allow(dead_code)]
fn day5()
{
	let input = get_input("day5.txt");
	//let mut stdin = std::io::stdin();

	let mut count = 0;
	let mut count2 = 0;
	for l in input.lines()
	{
		if is_string_nice(l)
		{
			count += 1;
		}
		if is_string_nice2(l)
		{
			count2 += 1;
		}

		// pause for input
		//let _ = stdin.read(&mut [0u8]).unwrap();
	}

	println!("Nice strings: {}, {}", count, count2);
}

#[allow(dead_code)]
fn day6()
{
	let input = get_input("day6.txt");

	let mut grid : [ [u8; 1000]; 1000] = [[0; 1000]; 1000];

	for l in input.lines()
	{
		if l.starts_with("turn on")
		{
			let index = l.find("through").unwrap();
			let first = l.get(8..index).unwrap();
			let second = l.get(index+7..).unwrap();

			let a: Vec<usize> = first.split(',').map(|s| s.trim().parse().unwrap()).collect();
			let b: Vec<usize> = second.split(',').map(|s| s.trim().parse().unwrap()).collect();

			for x in a[0]..b[0]+1
			{
				for y in a[1]..b[1]+1
				{
					grid[x][y] = 1;
				}
			}
		}
		else if l.starts_with("turn off")
		{
			let index = l.find("through").unwrap();
			let first = l.get(9..index).unwrap();
			let second = l.get(index+7..).unwrap();

			let a: Vec<usize> = first.split(',').map(|s| s.trim().parse().unwrap()).collect();
			let b: Vec<usize> = second.split(',').map(|s| s.trim().parse().unwrap()).collect();

			for x in a[0]..b[0]+1
			{
				for y in a[1]..b[1]+1
				{
					grid[x][y] = 0;
				}
			}
		}
		else if l.starts_with("toggle")
		{
			let index = l.find("through").unwrap();
			let first = l.get(7..index).unwrap();
			let second = l.get(index+7..).unwrap();

			let a: Vec<usize> = first.split(',').map(|s| s.trim().parse().unwrap()).collect();
			let b: Vec<usize> = second.split(',').map(|s| s.trim().parse().unwrap()).collect();

			for x in a[0]..b[0]+1
			{
				for y in a[1]..b[1]+1
				{
					grid[x][y] = match grid[x][y]
					{
						0 => 1,
						1 => 0,
						_ => panic!("Whaaaat"),
					};
				}
			}
		}
	}

	let mut count:u32 = 0;
	for &row in grid.iter()
	{
		for &col in row.iter()
		{
			count += col as u32;
		}
	}

	println!("Found {} lit lights.", count);
}

#[allow(dead_code)]
fn day6b()
{
	let input = get_input("day6.txt");

	let mut grid : [ [u8; 1000]; 1000] = [[0; 1000]; 1000];

	for l in input.lines()
	{
		if l.starts_with("turn on")
		{
			let index = l.find("through").unwrap();
			let first = l.get(8..index).unwrap();
			let second = l.get(index+7..).unwrap();

			let a: Vec<usize> = first.split(',').map(|s| s.trim().parse().unwrap()).collect();
			let b: Vec<usize> = second.split(',').map(|s| s.trim().parse().unwrap()).collect();

			for x in a[0]..b[0]+1
			{
				for y in a[1]..b[1]+1
				{
					grid[x][y] = grid[x][y].checked_add(1).expect("Ack");
				}
			}
		}
		else if l.starts_with("turn off")
		{
			let index = l.find("through").unwrap();
			let first = l.get(9..index).unwrap();
			let second = l.get(index+7..).unwrap();

			let a: Vec<usize> = first.split(',').map(|s| s.trim().parse().unwrap()).collect();
			let b: Vec<usize> = second.split(',').map(|s| s.trim().parse().unwrap()).collect();

			for x in a[0]..b[0]+1
			{
				for y in a[1]..b[1]+1
				{
					grid[x][y] = match grid[x][y].checked_sub(1)
					{
						Some(v) => v,
						_ => 0
					}
				}
			}
		}
		else if l.starts_with("toggle")
		{
			let index = l.find("through").unwrap();
			let first = l.get(7..index).unwrap();
			let second = l.get(index+7..).unwrap();

			let a: Vec<usize> = first.split(',').map(|s| s.trim().parse().unwrap()).collect();
			let b: Vec<usize> = second.split(',').map(|s| s.trim().parse().unwrap()).collect();

			for x in a[0]..b[0]+1
			{
				for y in a[1]..b[1]+1
				{
					grid[x][y] = grid[x][y].checked_add(2).expect("Yikes");
				}
			}
		}
	}

	let mut count:u32 = 0;
	for &row in grid.iter()
	{
		for &col in row.iter()
		{
			count += col as u32;
		}
	}

	println!("Found {} total brightness.", count);
}

#[derive(Debug)]
#[allow(dead_code)]
enum Operation<'z>
{
	AssignVar(&'z str),
	NumAnd(u16, &'z str),
	And (&'z str, &'z str),
	Or(&'z str, &'z str),
	LShift(&'z str, u16),
	RShift(&'z str, u16),
	Not(&'z str),
}

// Given an operation and a hashmap of resolved variables and their values, try to resolve this operation
#[allow(dead_code)]
fn resolve<'a>(op: &Operation, h: &'a HashMap<&'a str, u16>) -> Result<u16, &'static str>
{
	match op
	{
		&Operation::AssignVar(src) => 
		{
			if h.contains_key(src)
			{
				let &x = h.get(src).unwrap();
				return Ok(x);
			}
			else
			{
				return Err("Src var not resolved.");
			}
		},
		&Operation::NumAnd(a, b) =>
		{
			if h.contains_key(b)
			{
				let &b = h.get(b).unwrap();
				let x = a & b;
				return Ok(x);
			}
			else
			{
				return Err("One or more src vars not resolved");
			}		
		}
		&Operation::And(a, b) =>
		{
			if h.contains_key(a) && h.contains_key(b)
			{
				let &a = h.get(a).unwrap();
				let &b = h.get(b).unwrap();
				let x = a & b;
				return Ok(x);
			}
			else
			{
				return Err("One or more src vars not resolved");
			}
		},
		&Operation::Or(a, b) =>
		{
			if h.contains_key(a) && h.contains_key(b)
			{
				let &a = h.get(a).unwrap();
				let &b = h.get(b).unwrap();
				let x = a | b;
				return Ok(x);
			}
			else
			{
				return Err("One or more src vars not resolved");
			}
		},
		&Operation::LShift(a, b) =>
		{
			if h.contains_key(a)
			{
				let &a = h.get(a).unwrap();
				let x = a << b;
				return Ok(x);
			}
			else
			{
				return Err("One or more src vars not resolved");
			}
		},
		&Operation::RShift(a, b) =>
		{
			if h.contains_key(a)
			{
				let &a = h.get(a).unwrap();
				let x = a >> b;
				return Ok(x);
			}
			else
			{
				return Err("One or more src vars not resolved");
			}
		},
		&Operation::Not(a) =>
		{
			if h.contains_key(a)
			{
				let &a = h.get(a).unwrap();
				let x = !a;
				return Ok(x);
			}
			else
			{
				return Err("One or more src vars not resolved");
			}
		}		
	}
}

#[allow(dead_code)]
fn day7()
{
	let input = get_input("day7b.txt");
	// let input = get_input("day7_test.txt");

	let mut wires: HashMap< &str, Operation > = HashMap::new();
	let mut values: HashMap< &str, u16 > = HashMap::new();
	
	for l in input.lines()
	{
		println!("{}", l);
		let symbols : Vec<&str> = l.split(' ').collect();

		// Assignments
		if symbols[1] == "->"
		{
			let var = symbols[2];
			let num = symbols[0].parse();

			match num
			{
				// Parsed a number; this is just a value
				Ok(x) => { values.insert(var, x); true },
				// Didn't parse a number; this is a variable
				Err(_) => { wires.insert(var, Operation::AssignVar(symbols[0])); true },
			};
		}
		else
		{
			if symbols[1] == "AND"
			{
				// Cheating: my dataset contains cases of "1 AND x" but no other ones with number literals
				match symbols[0].parse()
				{
					// Parsed a number, add a NumAnd op
					Ok(v) => wires.insert(symbols[4], Operation::NumAnd(v, symbols[2])),
					// Failed to parse, just an And op
					Err(_) => wires.insert(symbols[4], Operation::And(symbols[0], symbols[2])),
				};
			}
			else if symbols[1] == "OR"
			{
				wires.insert(symbols[4], Operation::Or(symbols[0], symbols[2]));
			}
			else if symbols[1] == "LSHIFT"
			{
				// Assume the 2nd param is a number
				let num = symbols[2].parse().expect("LSHIFT parse failed");
				wires.insert(symbols[4], Operation::LShift(symbols[0], num));
			}
			else if symbols[1] == "RSHIFT"
			{
				// Assume the 2nd param is a number
				let num = symbols[2].parse().expect("RSHIFT parse failed");
				wires.insert(symbols[4], Operation::RShift(symbols[0], num));
			}
			else if symbols[0] == "NOT"
			{
				wires.insert(symbols[3], Operation::Not(symbols[1]));
			}
		}
	}

	while wires.len() > 0
	{
		wires.retain(|name, op|
		{
			//println!("Trying to resolve {}: {:?}", name, op);
			match resolve(op, &values)
			{
				Ok(val) => 
				{
					println!("Resolved {}: {}", name, val);
					values.insert(name, val);
					false
				},
				Err(_s) =>
				{
					true
				}
			}
		});
	}

	println!("Value on wire 'a' is: {}", values["a"]);

}

#[derive(Debug)]
struct City
{
	name: String,
	distances: HashMap<String, u16>,
}


fn day9()
{
	let input = get_input("day9.txt");

	let mut cities = HashMap::new();

	// Build the graph
	for l in input.lines()
	{
		let parts:Vec<&str> = l.split(' ').collect();

		let city1 = parts[0].to_string();
		let city2 = parts[2].to_string();
		let dist:u16 = parts[4].parse().expect("Invalid distance");

		if !cities.contains_key(&city1)
		{
			let c = City { name: city1.clone(), distances: HashMap::new() };
			cities.insert(city1.clone(), c);
		}

		if !cities.contains_key(&city2)
		{
			let c = City { name: city2.clone(), distances: HashMap::new() };
			cities.insert(city2.clone(), c);
		}

		if let Some(ref mut x) = cities.get_mut(&city1)
		{
			x.distances.insert(city2.clone(), dist);
		}

		if let Some(ref mut x) = cities.get_mut(&city2)
		{
			x.distances.insert(city1.clone(), dist);
		}
	}

	println!("{:?}", &cities);
}

fn get_input(name:&str) -> String
{
	let prefix = String::from("input/");
	let filename = prefix+name;
	let mut file = File::open(filename).unwrap();
	let mut input = String::new();
	file.read_to_string(&mut input).unwrap();
	return input;
}

fn main() 
{
	//day1();
	//day2();
	//day3();
	//day4();
	//day5();
	//day6b();
	//day7();
	// day 8 was stupid so I skipped it
	day9();
}
