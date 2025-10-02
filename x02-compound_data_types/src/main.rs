fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("Array: {:?}", a);
    let fruit = ["Apple", "Banana", "Orange"];
    println!("Array of strings: {:?}", fruit);
    println!("First fruit: {}", fruit[0]);

    let human : (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Tuple: {:?}", human);
    let my_mix_tuple = ("Bob", 25, true, [1, 2, 3]);
    println!("My mix tuple is {:?}", my_mix_tuple);

    //slices: [1, 2, 3, 4, 5]
    let number_slices:&[i32] = &[1, 2, 3, 4, 5];
    println!("My number slices {:?}", number_slices);

    let animal_slices:&[&String] = &[&"Lion".to_string(), &"Crocodile".to_string()];
    println!("My number slices {:?}", animal_slices);

    //Strings Vs String Slices (&str)
    //Strings [ growable, mutable, owned string type]
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone cold Says: {}", stone_cold);

    //B- &str (String Slice)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string;
    println!("Slice Value: {}", slice);
}
