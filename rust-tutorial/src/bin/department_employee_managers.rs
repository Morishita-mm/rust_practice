use std::{
    collections::HashMap,
    io::{Write, stdin},
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq, Hash, Clone, Copy)]
enum Department {
    Engineerings,
    Sales,
}

impl Department {
    fn department_from_number(num: usize) -> Option<Self> {
        Self::iter().nth(num)
    }
    fn as_str(&self) -> &'static str {
        match self {
            Department::Engineerings => "Engineerings",
            Department::Sales => "Sales",
        }
    }
}

#[derive(Debug, Clone)]
struct Employee {
    name: String,
}

impl Employee {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

fn main() {
    let mut employee_list = HashMap::<Department, Vec<Employee>>::new();
    loop {
        show_menu();
        match select_number() {
            Ok(1) => {
                let name = input_employee_name();
                let dept = select_department();
                employee_list
                    .entry(dept)
                    .or_insert_with(Vec::new)
                    .push(Employee::new(&name));
                println!("Added {} to {}.", name, dept.as_str());
            }
            Ok(2) => {
                let dept = select_department();
                show_department_employees(&dept, &employee_list);
            }
            Ok(3) => {
                show_all_employees(&employee_list);
            }
            _ => {
                println!("Quit");
                break;
            }
        }
    }
}

fn show_menu() {
    println!("================ Menu ================");
    println!("1: Add employee to department");
    println!("2: Show employees in a department");
    println!("3: Show all employees (sorted)");
    println!("Other: Quit");
}

fn select_department() -> Department {
    println!("Select department:");
    for (i, dep) in Department::iter().enumerate() {
        println!("{}: {}", i + 1, dep.as_str());
    }
    loop {
        print!("Enter department number: ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line.");
        if let Ok(idx) = input.trim().parse::<usize>() {
            if let Some(dep) = Department::department_from_number(idx - 1) {
                return dep;
            }
        }
        println!("Please enter a valid number.");
    }
}

fn input_employee_name() -> String {
    print!("Enter employee name: ");
    std::io::stdout().flush().unwrap();
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line.");
    name.trim().to_string()
}

fn select_number() -> Result<u32, std::num::ParseIntError> {
    print!("Enter your choice: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line.");
    input.trim().parse::<u32>()
}

fn show_department_employees(
    dept: &Department,
    employee_list: &HashMap<Department, Vec<Employee>>,
) {
    println!("======================================");
    println!("Employees in {}:", dept.as_str());
    if let Some(vec) = employee_list.get(dept) {
        let mut names: Vec<_> = vec.iter().map(|e| e.name.as_str()).collect();
        names.sort();
        for name in names {
            println!("{}", name);
        }
    } else {
        println!("No employees in this department.");
    }
}

fn show_all_employees(employee_list: &HashMap<Department, Vec<Employee>>) {
    println!("======================================");
    println!("All employees (sorted by department, then name):");
    let mut all: Vec<(&str, &str)> = vec![];
    for (dept, vec) in employee_list {
        for emp in vec {
            all.push((dept.as_str(), emp.name.as_str()));
        }
    }
    all.sort_by(|a, b| a.0.cmp(b.0).then(a.1.cmp(b.1)));
    for (dept, name) in all {
        println!("{}: {}", dept, name);
    }
}
