use std::collections::HashMap;
use std::io::stdin;
use terminal_menu::{button, menu, mut_menu, run, TerminalMenuItem};

pub struct Employee {
    name: String,
}

pub struct Department {
    employees: Vec<Employee>,
}

pub struct Company {
    name: String,
    departments: HashMap<String, Department>,
}

impl Company {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            departments: HashMap::new(),
        }
    }
}

pub fn add_new_employee(company: &mut Company, employee_name: String, department_name: String) {
    let employee = Employee {
        name: employee_name,
    };

    let department = company
        .departments
        .entry(department_name.clone())
        .or_insert(Department {
            employees: Vec::new(),
        });

    department.employees.push(employee)
}

pub fn list_all_employees(company: &Company) -> Vec<String> {
    let mut employees: Vec<String> = Vec::new();

    for (_department_name, department) in &company.departments {
        for employee in &department.employees {
            employees.push(employee.name.to_string());
        }
    }

    employees.sort();

    employees
}

pub fn company_interface(company: &mut Company) {
    println!(
        "Welcome to {} please let select on the following options:",
        company.name
    );

    let start_menu = menu(vec![
        button("Add new employee"),
        button("List employees from * department"),
        button("List all employees"),
        button("Exit"),
    ]);

    let mut exit = false;

    while !exit {
        run(&start_menu);

        let start_menu_response = mut_menu(&start_menu);

        match start_menu_response.selected_item_index() {
            0 => {
                print!("{esc}c", esc = 27 as char);

                let mut employee_name = String::new();
                let mut department_name = String::new();

                println!("What is the employee's name?");
                stdin()
                    .read_line(&mut employee_name)
                    .expect("Valid employee required!");

                println!("What department is the employee joining?");
                stdin()
                    .read_line(&mut department_name)
                    .expect("Valid employee required!");

                add_new_employee(company, employee_name, department_name);
            }
            1 => {
                print!("{esc}c", esc = 27 as char); 

                let department_names: Vec<TerminalMenuItem> = company
                    .departments
                    .iter()
                    .map(|(department_name, _department)| button(department_name))
                    .collect();

                let department_menu = menu(department_names);

                run(&department_menu);

                let department_menu_response = mut_menu(&department_menu);
                let department_name = department_menu_response.selected_item_name();

                let department = company
                    .departments
                    .get(&department_name.to_string())
                    .expect("No department found with that name!");

                for employee in &department.employees {
                    println!("{}", employee.name);
                }
            }
            2 => {
                print!("{esc}c", esc = 27 as char);
                
                let all_employees = list_all_employees(company);

                for employee in all_employees {
                    println!("{}", employee);
                }
            }
            3 => exit = true,
            _ => panic!("No valid option selected"),
        }
    }
}
