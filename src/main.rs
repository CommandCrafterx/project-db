use dialoguer::Input;
use serde::{Serialize, Deserialize};
use std::path::Path;

// Define Project Struct
#[derive(Serialize, Deserialize)]
struct Project {
    id: u32,
    name: String,
    description: String,
} 

// Define JSON file for the data
const DB_FILE: &str = "data.json";

fn help() {
    println!("Help: 
     help: Show this help menu 
     exit: Exit the program 
     new: Create a new project 
     delete: Delete a project 
     list: List all projects in the Database 
     save: Manually save the database to a file 
     version: Print version number \n");
}

fn version() {
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
}

fn add_project(projects: &Vec<Project>) -> Project {
    
    // Ask user for project Information
    
    let id: u32 = loop {
        
        // Ask user for project ID and check if it is already in use
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
            // End the loop and return the final ID
            break new_id;
        }
    };
    // Ask user for project name and description
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

fn delete_project(projects: &mut Vec<Project>) {
    // Ask user for project ID to delete and ckeck if it exists
    let delete_id: u32 = Input::new()
    .with_prompt("Enter the ID of the Project to delete")
    .interact_text()
    .unwrap();

    let mut found: bool = false;
    let mut delete_index: usize = 0;

    for project in projects.iter() {
        if delete_id == project.id {
            found = true;
            break;
        }
        delete_index +=1;
    }

    if found {
        println!("Deleting Project {delete_id} at index {delete_index}...");
        projects.remove(delete_index);
    } else {
        println!("The Project you are requesting to delete doesent exist.");
    }
    

    
}

fn list_projects(projects: &Vec<Project>) {
    // List all projects from the vector
    println!("All Projects:\n");
    if projects.is_empty() {
        println!("There are currently no projects stored.\n");
    } else {
        for project in projects {
            println!("ID: {}", project.id);
            println!("Name: {}", project.name);
            println!("Description: {} \n", project.description);
        }
        println!("Projects stored: {} \n", projects.len());
    }
}

fn save_to_file(projects: &Vec<Project>) {
    // Convert the project vector to JSON
    let db_json = serde_json::to_string_pretty(projects).unwrap();

    // Write the JSON data to the data.json file
    std::fs::write(DB_FILE, db_json).unwrap();
    println!("Succesfully saved Database to file")
}

fn load_from_file() -> Vec<Project> {
    
    let db_json = std::fs::read_to_string(DB_FILE).unwrap();
    
    // Create the Database from db_json and return the new database
    let projects: Vec<Project> = serde_json::from_str(&db_json).unwrap();
    return projects;
}

fn check_for_db_file() -> bool {
    let db_file_exists: bool = Path::new(DB_FILE).exists();
    return db_file_exists;
}

fn main() {
    // Initialize or load Database
    let mut projects: Vec<Project>;
    if check_for_db_file() {
        println!("Loading database from existing file...\n");
        projects = load_from_file();
    } else {
        println!("No existing Database found creating a new one...\n");
        projects = Vec::new();
    }

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
        } else if input == "delete" {
            delete_project(&mut projects);
        } else if input == "save" {
            save_to_file(&projects);
        } else {
            println!("Unknown Command!");
        }
    }
}
