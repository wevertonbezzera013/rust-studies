#[derive(Debug, Copy, Clone)]
enum Position {
    Manager,
    Supervisor,
    Worker
}

#[derive(Debug, Copy, Clone)]
struct Employee {
    position: Position
    work_hours: i64
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40
    };
    /* match me.position {
        Position::Manager => println!("manager"),
        Position::Supervisor => println!("supervisor"),
        Position::Worker => println!("worker"),
    } */
    print_employee(me);
    print_employee(me);
}