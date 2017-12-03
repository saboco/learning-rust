fn main() {
    println!("******************* Vectores ********************");
    vectores();
    println!("******************* Strings *********************");
    strings();
    println!("****************** Hash Maps ********************");
    hash_maps();
}

fn vectores() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = Vec::new();
    vec2.push(1);
    vec2.push(5);
    vec2.push(3);

    let vec3 = vec![1, 2, 5];
    let mut vec4 = vec![4, 2, 9];
    vec4.push(1);
    vec4.push(3);

    println!("vec1 is {:?}", vec1);
    println!("vec2 is {:?}", vec2);
    println!("vec3 is {:?}", vec3);
    println!("vec4 is {:?}", vec4);
}

fn strings() {
    let data = "initial contents";
    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    println!("s is {}", s);

    let hello = String::from("السلام عليكم");
    println!("hello is {}", hello);
    let hello = String::from("Dobrý den");
    println!("hello is {}", hello);
    let hello = String::from("Hello");
    println!("hello is {}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("hello is {}", hello);
    let hello = String::from("नमस्ते");
    println!("hello is {}", hello);
    let hello = String::from("こんにちは");
    println!("hello is {}", hello);
    let hello = String::from("안녕하세요");
    println!("hello is {}", hello);
    let hello = String::from("你好");
    println!("hello is {}", hello);
    let hello = String::from("Olá");
    println!("hello is {}", hello);
    let hello = String::from("Здравствуйте");
    println!("hello is {}", hello);
    let hello = String::from("Hola");
    println!("hello is {}", hello);

    let mut s = String::from("foo");
    println!("s is {}", s);
    s.push_str("bar");
    println!("s is {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    println!("s1 is {}", s1);
    let s2 = String::from("world!");
    println!("s2 is {}", s2);
    let s3 = s1 + &s2; // Note that s1 has been moved here and can no longer be used
    println!("s3 is {}", s3);
    println!("s2 is {}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {}", s);

    let s1 = String::from("tic");
    //   let s2 = String::from("tac"); // this wasn't moved
    //   let s3 = String::from("toe"); // this wasn't moved

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
fn hash_maps() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores is {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("scores is {:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    //let s = field_name + "hola"; // field_name was moved
    println!("map is {:?}", map);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("blue team score is {:?}", score);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
