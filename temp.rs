//https://doc.rust-lang.ru/rust-cookbook/encoding/csv.html


fn main() {
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);

    let args: Vec<String> = env::args().collect();

    //let query = &args[1];
    //let filename = &args[2];
    //let config = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
        /*Config {
            query: String::from("ertet"),
            filename: String::from("gjkg"),
        }*/
    });

    println!("Searching for {}", config.query); //map_or(|| 1, |v| v)
    println!("In file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

fn addr_of(s: &str) -> usize {
    s.as_ptr() as usize
}

fn split_whitespace_indices(s: &str) -> impl Iterator<Item = (usize, &str)> {
    s.split_whitespace()
        .map(move |sub| (addr_of(sub) - addr_of(s), sub))
}

fn main() {
    let mut iter = split_whitespace_indices(" Hello world");

    assert_eq!(Some((1, "Hello")), iter.next());
    assert_eq!(Some((7, "world")), iter.next());
}



There is a special method split for struct String:

fn split<'a, P>(&'a self, pat: P) -> Split<'a, P> where P: Pattern<'a>

Split by char:

let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);

Split by string:

let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
assert_eq!(v, ["lion", "tiger", "leopard"]);

Split by closure:

let v: Vec<&str> = "abc1def2ghi".split(|c: char| c.is_numeric()).collect();
assert_eq!(v, ["abc", "def", "ghi"]);
