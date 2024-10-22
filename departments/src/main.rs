use std::collections::HashMap;
use std::io;

fn main() {
    let mut departments = HashMap::new();

    loop {
        println!("Please enter a department to which you would like to add an employee.");

        let mut department_line = String::new();
        io::stdin()
            .read_line(&mut department_line)
            .expect("Failed to read line");

        let department = String::from(department_line.trim_ascii());
        println!("Please enter an employee to add to {department}.");

        let mut employee_line = String::new();
        io::stdin()
            .read_line(&mut employee_line)
            .expect("Failed to read line");

        let employee = String::from(employee_line.trim_ascii());
        add_employee(employee, department, &mut departments);

        println!("Add another employee (Y/N)?");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        match answer.as_str().trim_ascii() {
            "Y" | "y" => continue,
            "N" | "n" => break,
            _ => {
                println!("Invalid response, please enter 'Y' for yes or 'N' for no.");
                continue;
            }
        }
    }

    loop {
        println!("Please enter a department for which you would like to view the employees.");

        let mut department_line = String::new();
        io::stdin()
            .read_line(&mut department_line)
            .expect("Failed to read line");

        let department = String::from(department_line.trim_ascii());

        println!("Employees in {department}:");
        let employee_list = retrieve_employee_list(department, &mut departments);
        match employee_list {
            Some(employee_list) => print_employee_list(employee_list),
            None => println!("No matching employees found!"),
        }

        println!("Print another department (Y/N)?");
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        match answer.as_str().trim_ascii() {
            "Y" | "y" => continue,
            "N" | "n" => break,
            _ => {
                println!("Invalid response, please enter 'Y' for yes or 'N' for no.");
                continue;
            }
        }
    }
}

fn add_employee(
    employee: String,
    department: String,
    departments: &mut HashMap<String, Vec<String>>,
) {
    let employee_list = departments.get_mut(&department);

    match employee_list {
        Some(employee_list) => employee_list.push(employee),
        None => {
            let new_list = vec![employee];
            departments.insert(department, new_list);
        }
    }
}

fn retrieve_employee_list(
    department: String,
    departments: &mut HashMap<String, Vec<String>>,
) -> Option<Vec<String>> {
    {
        let this = departments.get(&department);
        match this {
            Some(t) => Some(t.clone()),
            None => None,
        }
    }
}

fn print_employee_list(employees: Vec<String>) {
    let mut sorted_employees = employees.clone();
    sorted_employees.sort();

    for employee in sorted_employees {
        println!("{employee}");
    }
}
