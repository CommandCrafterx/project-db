// Import dialoguer Library
use dialoguer::Input;

// Define Project Struct
struct Project {
    id: u32,
    name: String,
    description: String,
} 

fn help() {
    println!("Help:\n help: Show this help menu \n exit: Exit the program \n new: Create a new project \n delete: Delete a project \n list: List all projects in the Database \n version: Print version number \n");
}

fn version() {
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
}

fn add_project(projects: &Vec<Project>) -> Project {
    
    // Ask user for project Information
    let id: u32 = loop {
        
        let new_id: u32 = Input::new()
        .with_prompt("Enter a Project ID")
        .interact_text()
        .unwrap();
        
        let mut id_already_in_use = false;

        for project in projects {
            if new_id == project.id {
                id_already_in_use = true;
            }
        }
        if id_already_in_use {
            println!("The ID is already in use! Please try again");
        } else {
            break new_id;
        }
    };
    
    let name: String = Input::new()
        .with_prompt("Enter a name for the project")
        .interact_text()
        .unwrap();

    let description: String = Input::new()
        .with_prompt("Enter a project description")
        .interact_text()
        .unwrap();

    // Create a Project using the data
     let project = Project {
        id,
        name,
        description,
    };
    // Return the project
    return project;
    
}

fn list_projects(projects: &Vec<Project>) {
    // List all projects from the vector
    println!("All Projects:\n");
    for project in projects {
        println!("ID: {}", project.id);
        println!("Name: {}", project.name);
        println!("Description: {} \n", project.description);
    }
}

fn main() {
    // Initialize Database
    let mut projects: Vec<Project> = Vec::new();
    // Welcome text
    println!("Welcome to ProjectDB!\n");
    help();

    // Main CLI
    loop {
        let input: String = Input::new()
        .with_prompt("Enter a command")
        .interact_text()
        .unwrap();

        if input == "help" {
            help();
        } else if input == "exit" {
            break;
        } else if input == "new" {
            let project = add_project(&projects);
            projects.push(project);
            println!("Your Project was added succesfully!")
        } else if input == "list" {
            list_projects(&projects);
        } else if input == "version" {
            version();
        } 
        else {
            println!("Unknown Command!");
        }
    }
}