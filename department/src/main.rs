// Using a hash map and vectors, create a text interface to allow a user
// to add employee names to a department in a company; for example,
// “Add Sally to Engineering” or “Add Amir to Sales.” Then, let the user
// retrieve a list of all people in a department or all people in the company
// by department, sorted alphabetically.

use std::collections::HashMap;

#[derive(PartialEq)]
enum Department
{
    Sales,
    Marketing,
    Technical,
}

struct EmployeeMap(HashMap<String, Department>);

impl EmployeeMap
{
    fn new() -> Self
    {
        return EmployeeMap(HashMap::new());
    }

    fn add_employee(&mut self, name: String, dept: Department)
    {
        self.0.insert(name, dept);
    }

    fn get_employees_by_department(&self, dept: &Department) -> Vec<&String>
    {
        self.0
            .iter()
            .filter(|(_, v)| *v == dept)
            .map(|(k, _)| k)
            .collect()
    }
}

fn main()
{
    let mut employee_map = EmployeeMap::new();

    employee_map.add_employee("Alice".to_string(), Department::Sales);
    employee_map.add_employee("Bob".to_string(), Department::Marketing);
    employee_map.add_employee("Charlie".to_string(), Department::Technical);

    println!(
        "{:?}",
        employee_map.get_employees_by_department(&Department::Technical)
    );
}
