pub fn print() {
    println!("Hello, world!");

    println!("my name is {} and I'm {} years old", "miald", 27);

    let person = ("milad", 27);

    println!("my name is {} and I'm {} years old", person.0, person.1);

    let numbers: [i32; 2] = [1, 2];

    println!("my name is {} and I'm {} years old", numbers[0], numbers[1]);

    let (name, age, isProgrammer) = ("milad", 27, true);
    println!("my name is {} and I'm {} years old and it is {}", name, age, isProgrammer);

}