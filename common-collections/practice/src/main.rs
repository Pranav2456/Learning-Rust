// Create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
// Implement the following:
use std::collections::HashMap;
use std::io;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if input == "exit" {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() != 4 {
            println!("Invalid input");
            continue;
        }

        let name = parts[1];
        let department = parts[3];
        let employees = company.entry(department.to_string()).or_insert(Vec::new());
        employees.push(name.to_string());
        employees.sort();

        println!("{:?}", company);
    }
} // Simple and direct implementation

// Optimized version given by Claude

// Define all possible commands as enum variants
// Each variant can hold data specific to that command
enum Command {
    Add(String, String),         // Holds (name, department)
    ListDepartment(String),      // Holds department name
    ListAll,                     // No associated data needed
    Exit,                        // No associated data needed
    Invalid,                     // No associated data needed
}

// Main struct to hold company data
// Using HashMap where:
// - Key (String): department name
// - Value (Vec<String>): list of employee names in that department
struct Company {
    departments: HashMap<String, Vec<String>>,
}

// Implementation block for Company struct
// This is where we define all methods associated with Company
impl Company {
    // Associated function (like a static method) to create a new Company
    // Returns a Company instance with an empty HashMap
    fn new() -> Self {  // Self is an alias for the Company type
        Self {
            departments: HashMap::new(),  // Create new, empty HashMap
        }
    }

    // Method to add an employee to a department
    // Takes mutable reference to self because we're modifying the HashMap
    fn add_employee(&mut self, name: String, department: String) {
        // entry() returns Entry enum which provides in-place update capabilities
        // or_insert_with() calls the provided closure if key doesn't exist
        // This is more efficient than using or_insert(Vec::new())
        let employees = self.departments
            .entry(department.clone())  // Get entry for department
            .or_insert_with(Vec::new());  // If doesn't exist, insert new Vec
        
        // Check if employee already exists in department
        if !employees.contains(&name) {
            employees.push(name);        // Add name to vector
            employees.sort();            // Sort names alphabetically
            println!("Added {} to {}", name, department);
        } else {
            println!("{} is already in {}", name, department);
        }
    }

    // Method to list all employees in a specific department
    // Takes reference to self because we're only reading
    fn list_department(&self, department: &str) {
        // Using match with Option returned by HashMap::get
        match self.departments.get(department) {
            // If department exists and has employees
            Some(employees) if !employees.is_empty() => {
                println!("\n{} department:", department);
                // Iterate over employee names in the Vec
                for employee in employees {
                    println!("- {}", employee);
                }
            }
            // If department doesn't exist or is empty
            _ => println!("No employees found in {}", department),
        }
    }

    // Method to list all departments and their employees
    fn list_all(&self) {
        if self.departments.is_empty() {
            println!("No employees in the company");
            return;
        }

        println!("\nAll departments:");
        // Collect keys (department names) into a Vec for sorting
        let mut departments: Vec<_> = self.departments.keys().collect();
        departments.sort();  // Sort department names alphabetically

        // Iterate over sorted department names
        for dept in departments {
            self.list_department(dept);
        }
    }
}

// Function to parse user input into a Command enum
fn parse_command(input: &str) -> Command {
    // Split input string into Vec of word strings
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    // Match on first word of input (if it exists)
    match parts.get(0).map(|s| *s) {
        Some("add") | Some("Add") => {
            // Check for correct "add name to department" format
            if parts.len() == 4 && (parts[2] == "to" || parts[2] == "TO") {
                Command::Add(parts[1].to_string(), parts[3].to_string())
            } else {
                Command::Invalid
            }
        }
        Some("list") | Some("List") => {
            // Check what we're listing (all or specific department)
            match parts.get(1) {
                Some(&"all") => Command::ListAll,
                Some(department) => Command::ListDepartment(department.to_string()),
                None => Command::Invalid,
            }
        }
        Some("exit") | Some("quit") => Command::Exit,
        _ => Command::Invalid,
    }
}

// Helper function to print available commands
fn print_help() {
    println!("\nAvailable commands:");
    println!("- Add <name> to <department>");
    println!("- List <department>");
    println!("- List all");
    println!("- Exit");
}

/*
fn main() {
    // Create new Company instance
    let mut company = Company::new();
    println!("Welcome to Company Directory!");
    print_help();

    // Main program loop
    loop {
        print!("\nEnter command: ");
        Flush stdout to ensure prompt is displayed
        io::stdout().flush().unwrap();
        
        Create new String to store user input
        let mut input = String::new();
        Read line from stdin into input String
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        Match on the parsed command
        match parse_command(input.trim()) {
            Command::Add(name, department) => {
                company.add_employee(name, department);
            }
            Command::ListDepartment(department) => {
                company.list_department(&department);
            }
            Command::ListAll => {
                company.list_all();
            }
            Command::Exit => {
                println!("Goodbye!");
                break;
            }
            Command::Invalid => {
                println!("Invalid command!");
                print_help();
            }
        }
    }
*/
