struct Unko{
    name: String,
    age: u32,
}

impl Unko{
    fn greeting(&self){
        println!("Hello!! I'm {}!! My age is {}.", self.name, self.age);
    }
}

fn main() {
    let unkoman = Unko{name: String::from("Mike"), age: 32};
    unkoman.greeting();
}
