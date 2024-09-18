use std::collections::HashMap;

fn main() {
    let employee = String::from("Sally");
    let department = String::from("Engineering");
    let departments = HashMap::new();

    add_employee(employee, department, &mut departments);
}

fn add_employee(employee: String, department: String, departments: &mut HashMap<String, String>) {
    departments.entry(department).or_insert(employee);
}
