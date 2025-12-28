fn main() {
    let number: i8 = 127;

    if number == std::i8::MAX {
        println!("no se pueden agregr mas valores :{}", std::i8::MAX)
    } else {
        println!("se puede invluir mas valores:{}", std::i8::MAX)
    }

    let asigned = 1;

    let value = match asigned {
        1 => 1,
        _ => 0,
    };

    let mut counter = 0;
    let maxcounter = 10;

    while counter < maxcounter {
        println!("counter: {counter}");
        counter += 1
    }

    let mut value1 = 0;

    loop {
        println!(" value {value1}");
        value1 += 1;

        if value1 == 10 {
            break;
        }
    }

    for i in 1..3 {
        println!("{i}")
    }

    println!("Hello, world! {value} {value1}");
}
