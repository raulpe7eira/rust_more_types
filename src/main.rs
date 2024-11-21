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

fn enumerations() {
    println!("É fim de semana? {}", weekend(Weekday::Monday))
}

fn main() {
    array();
    matrix();
    enumerations();
}
