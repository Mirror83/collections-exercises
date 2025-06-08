use std::{
    collections::HashMap,
    io::{self, Write},
};

type EmployeeMap = HashMap<String, Vec<String>>;

const DEPARTMENTS: [&str; 2] = ["Engineering", "Sales"];

fn initialize_employee_map() -> EmployeeMap {
    let mut map = HashMap::new();
    for dept in DEPARTMENTS {
        map.insert(String::from(dept), vec![]);
    }
    return map;
}

fn print_dept_employees(dept: &str, employees: &Vec<String>) {
    println!("{dept}");
    for _ in 0..dept.len() {
        print!("-")
    }
    println!();
    for (i, employee) in employees.iter().enumerate() {
        println!("{}. {}", i + 1, employee);
    }
}

fn list_all_employees_by_dept(map: &EmployeeMap) {
    for (dept, employees) in map {
        print_dept_employees(dept, employees);
    }
}

fn list_all_people_in_dept(map: &mut EmployeeMap, dept: &str) {
    match map.get(dept) {
        Some(employees) => {
            print_dept_employees(dept, employees);
        }
        None => println!("The department {dept} does not exist"),
    }
}

fn add_employee_to_dept(map: &mut EmployeeMap, dept: &str, employee: &str) {
    map.entry(String::from(dept))
        .and_modify(|employees| {
            employees.push(String::from(employee));
            employees.sort()
        })
        .or_insert(vec![String::from(employee)]);
}

/// A text interface to allow a user to add employee names to a department in a company;
/// for example, “Add Sally to Engineering” or “Add Amir to Sales.”
/// Then let the user retrieve a list of all people in a department
/// or all people in the company by department, sorted alphabetically
pub fn company_hr_text_interface() {
    let mut map = initialize_employee_map();

    'interface_loop: loop {
        println!("Company Database");
        println!("Available departments: {DEPARTMENTS:?}");
        println!("Options");
        println!("--------");
        println!("1. Add employee to a department.");
        println!("2. List all people in department.");
        println!("3. List all people in the company by department.");
        println!("4. Quit.");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice. Choice must be a number between 1 and 4 (inclusive).");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Available departments: {DEPARTMENTS:?}");
                println!("To add someone, enter Add Name to Dept");
                println!("e.g. Add John to Sales");
                let mut prompt = String::new();
                io::stdin()
                    .read_line(&mut prompt)
                    .expect("Failed to read prompt");

                let mut employee = String::new();
                let mut dept = String::new();
                for (i, word) in prompt.split_ascii_whitespace().enumerate() {
                    match i {
                        1 => employee.push_str(word),
                        3 => dept.push_str(word.trim()),
                        _ => (),
                    }
                }
                add_employee_to_dept(&mut map, &dept, &employee);
            }
            2 => {
                println!("Available departments: {DEPARTMENTS:?}");

                print!("Enter the name of the department: ");
                io::stdout().flush().unwrap();

                let mut dept = String::new();
                io::stdin()
                    .read_line(&mut dept)
                    .expect("Failed to read department");

                list_all_people_in_dept(&mut map, &(dept.trim()));
            }
            3 => list_all_employees_by_dept(&mut map),
            4 => break 'interface_loop,
            _ => println!("Invalid choice"),
        }
    }
}
