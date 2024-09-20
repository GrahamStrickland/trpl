use std::collections::HashMap;

fn main() {
    let employee = String::from("Sally");
    let mut department = String::from("Engineering");
    let mut departments = HashMap::new();

    add_employee(employee, department, &mut departments);

    department = String::from("Engineering");
    println!("Employees in {department}:");
    let employee_list = retrieve_employee_list(department, &mut departments);
    match employee_list {
        Some(employee_list) => print_employee_list(employee_list),
        None => println!("No matching employees found!"),
    }
}

fn add_employee(
    employee: String,
    department: String,
    departments: &mut HashMap<String, Vec<String>>,
) {
    let employee_list = departments.get(&department);

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
    for employee in employees {
        println!("{employee}");
    }
}
