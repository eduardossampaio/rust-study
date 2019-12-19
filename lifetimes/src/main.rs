struct ImportantExcerpt<'a> {
    part: &'a str
}
fn main() {

    println!("Hello, world!");
    let r;
    // {
        let x = 5;
        r = &x;
    // }
    println!("R is: {}", r);

    let string1 = String::from("abcd");
    // {
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("str1 is: {}, str2 is: {}", string1,string2);
        println!("The longest string is {}", result);
    // }
    let first_sentence =  "something interesting";
    let i = ImportantExcerpt{
        part: first_sentence
    };
    println!("{}", first_sentence);
    println!("{}", i.part);

}

fn longest<'a>(str1: &'a str, str2: &'a str ) -> &'a str{
    if str1.len() > str2.len() {
        str1
    }else{
        str2
    }
}
fn longest_2<'a>(_x: &str, _y: &str) -> String {
    let result = String::from("really long string");
    result
}

// fn longest2(str)
