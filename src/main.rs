fn array() {
    let grades: [f32; 4] = [10f32, 8f32, 9.5, 6.0];
    for grade in grades {
        println!("A nota é = {}", grade);
    }

    let other_grades = [3.5; 5];
    for other_grade in other_grades {
        println!("A outra nota é = {}", other_grade);
    }

    let index: usize = 1;
    println!("{}", grades[index]);
}

fn matrix() {
    let matrix = [[0.0, 1.2, 0.1], [1.3, 0.3, 1.4]];

    for row in matrix {
        for item in row {
            println!("Item = {}", item);
        }
    }
}

#[allow(dead_code)]
enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

fn weekend(weekday: Weekday) -> bool {
    match weekday {
        Weekday::Sunday | Weekday::Saturday => true,
        _ => false,
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

fn colors() {
    let color = Color::Green;

    println!(
        "Cor = {}",
        match color {
            Color::Red => "vermelho",
            Color::Green => "verde",
            Color::Blue => "azul",
            Color::RgbColor(0, 0, 0)
            | Color::CymkColor {
                cyan: 0,
                magenta: 0,
                yellow: 0,
                black: 0,
            } => "preto",
            Color::RgbColor(_, _, _) => "RGB desconhecido",
            Color::CymkColor {
                cyan: _,
                magenta: _,
                yellow: _,
                black: _,
            } => "CYMK desconhecido",
        }
    );
}

fn enumerations() {
    println!("É fim de semana? {}", weekend(Weekday::Monday));

    colors();
}

fn optional() {
    let content_file = read_file(String::from("oi"));

    // first option to get optional
    match &content_file {
        Some(value) => println!("{}", value),
        None => println!("Arquivo não existe"),
    }

    // second option to get optional
    println!("{:?}", content_file);

    // third option to get optional
    if let Some(value) = content_file {
        println!("Agora, tenho certeza de ter um valor {}", value);
    }
}

fn read_file(path: String) -> Option<String> {
    Some(String::from(path))
}

fn main() {
    array();
    matrix();
    enumerations();
    optional();
}
