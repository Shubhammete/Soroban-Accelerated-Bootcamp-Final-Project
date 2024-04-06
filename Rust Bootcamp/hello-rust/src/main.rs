fn main() {
    println!("Hello, world!");

    // data types

    // int 
    // i8 i16 i32 i64 u8 u16 u32 u64 
    let num : i32 = 42;

    // bool
    let isTrue : bool = true;

    // char
    let c: char = 'a';

    // float
    //f32 f64
    let float_no:f32 = 23.4;

    // tuple
    let new_tuple:(i32,char,bool) = (33,'a',false);

    //array
    let new_Arr : [i32;3] = [2,3,4];

    fn add(a:i32, b:i32)-> i32{
        return a+b
    }

    let result : i32 = add(2,3);
    print!("The sum of {} and {} is {}\n",2,3,result);

    // enum is used for defining a type that has a set of named values or variants
   

    enum Direction {
        North,
        South,
        East,
        West,
    }
    
    fn movePlayer(dir:Direction) {
        // Using the enum variants
        let player_direction = dir;
    
        // Match expression to handle different enum variants
        match player_direction {
            Direction::North => println!("Player is facing North"),
            Direction::South => println!("Player is facing South"),
            Direction::East => println!("Player is facing East"),
            Direction::West => println!("Player is facing West"),
        }
    }

    movePlayer(Direction::North);


    // struct

    struct Person{
        name: String,
        age:u32,
    }

    let person : Person = Person{
        name: String::from("Alice"),
        age:30,
    };

    println!("Hi! I am {} and I am {} years old",person.name,person.age);

    //trait is defines behaviour hat types can implement

    trait Printable{
        fn print(&self);
    }

    struct Book{
        title:String,
    }
// implement trait on Book type 
    impl Printable for Book{
        fn print(&self){
            println!("Book Title :{}",self.title);
        }
    }

    let book = Book{
        title:String::from("The Alchemist"),
    };
    book.print();

}
