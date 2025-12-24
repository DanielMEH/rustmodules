fn main() {
    //! asignacion de variables
    //!  ```
    //! let s = "Hello".to_string();
    //! asigned value for string
    //! ```
    //!  
    //!
    //!
    /*!
     * ```
     * let s = "Hello".to_string();
     */

    let value: String = String::from("handle daniel");
    let value_str: &str = "handle str"; /* &str */
    let value_char: char = '😊'; /* char 4 bits */
    let (v1, v2): (i32, i32);
    let tuplas_value: (i8, String) = (1, "Daniel".to_string());
    v1 = 12;
    v2 = 10;

    let array_value: [i8; 3] = [1, 4, 3];

    let sumatoria: i32 = array_value.iter().map(|n| *n as i32).sum::<i32>();
    let add_number: Vec<i8> = array_value
        .iter()
        .map(|n| (*n as i8) + 1)
        .collect::<Vec<i8>>();

    let mut value_mut = String::from("Rust");
    value_mut.push_str("click");

    let operation = |x: i32, y: i32| -> i32 { x * y };

    let option_type_shadowing: String = String::from("initialize value");
    println!(
        "shadowing {option_type_shadowing} {} {} {} {} {sumatoria} {:?}",
        tuplas_value.0,
        tuplas_value.1,
        array_value[0],
        operation(2, 5),
        add_number,
    );
    let option_type_shadowing: i32 = 2;

    let min = std::i8::MIN;
    let max = std::i8::MAX;
    println!(
        "Hello, world! {value} {value_str} {value_char} {v1} {v2} {value_mut} {option_type_shadowing} {min} {max}"
    );
}
