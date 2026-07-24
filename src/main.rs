use dialoguer::Input;
use serde::{Serialize, Deserialize};
use std::path::Path;
use clap::Parser;
use clap::Subcommand;

// Define the commands for project-db
#[derive(Subcommand)]
enum Commands {
    /// Create a new Project
    New,
    /// Delete a Project
    Delete,
    /// List all Projects
    List,
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

// Define Project Struct
#[derive(Serialize, Deserialize)]
struct Project {
    name: String,
    description: String,
} 

fn add_project(projects: &Vec<Project>) -> Project {
    
    let name = loop {
        // Ask user for a project name and check if its already in use
        let new_name: String = Input::new()
        .with_prompt("Enter a name for the project")
        .interact_text()
        .unwrap();
        
        let new_name = new_name.trim().to_string().to_lowercase();

        let mut name_already_in_use: bool = false;
        for project in projects {
            if new_name == project.name {
                name_already_in_use = true;
            }
        }
        if name_already_in_use {
            println!("The Name is already in use! Please try again");
        } else {
            // End the loop and return the name for the project
            break new_name;
        }
    };
    // Ask user for a project description
    let mut description: String = Input::new()
        .with_prompt("Enter a project description (Optional)")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    if description == "" {
        description = String::from("None");
    }

    // Create a Project using the data
     let project = Project {
        name,
        description,
    };
    // Return the project
    return project;
    
}

fn delete_project(projects: &mut Vec<Project>) {
    // Ask user for the Project to delete
    let delete_name: String = Input::new()
    .with_prompt("Enter the Name of the Project to delete")
    .interact_text()
    .unwrap();

    let delete_name = delete_name.trim().to_lowercase();

    let mut found: bool = false;
    let mut delete_index: usize = 0;

    // Check if the project exists and find out its index in the vector
    for project in projects.iter() {
        if delete_name == project.name {
            found = true;
            break;
        }
        delete_index +=1;
    }

    if found {
        // Remove the project via its index
        println!("Deleting Project {delete_name} at index {delete_index}...");
        projects.remove(delete_index);
    } else {
        println!("The Project you are requesting to delete doesent exist.");
    }
    

    
}

fn list_projects(projects: &Vec<Project>) {
    // List all projects from the vector
    println!("All Projects:\n");
    if projects.is_empty() {
        println!("There are currently no projects stored.\n"); // Display a custom message when there are no projects in the vector
    } else {
        for project in projects {
            println!("Name: {}", project.name);
            
            if project.description != "None" {
                println!("Description: {} \n", project.description);
            } else {
                println!(""); // Prevent inconsistent formatting when the description isnt printed
            }
            
        }
        println!("Total projects stored: {}\n", projects.len());
    }
}

// Get the paths to the data files of project-db relative to the users home directory
fn get_data_dir() -> String {
    let home = std::env::var("HOME").unwrap();
    let db_dir = format!("{home}/.local/share/project-db");
    return db_dir; 
}
fn get_db_path() -> String {
    let db_path = format!("{}/data.json", get_data_dir());
    return db_path; 
}

fn save_to_file(projects: &Vec<Project>) {
    // Convert the project vector to JSON
    let db_json = serde_json::to_string_pretty(projects).unwrap();

    // Create the data directory if it doesent exist
    let db_dir_exists: bool = Path::new(&get_data_dir()).exists();
    if !db_dir_exists {
        std::fs::create_dir_all(get_data_dir()).unwrap();
    }
    // Write the JSON data to the data.json file
    std::fs::write(get_db_path(), db_json).unwrap();
    println!("Succesfully saved Database to file")
}

fn load_from_file() -> Vec<Project> {
    
    let db_json = std::fs::read_to_string(get_db_path()).unwrap();
    
    // Create the Database from db_json and return the new database
    let projects: Vec<Project> = serde_json::from_str(&db_json).unwrap();
    return projects;
}

fn check_for_db_file() -> bool {
    // Check if the data.json file exists using Path::new
    let db_file_exists: bool = Path::new(&get_db_path()).exists();
    return db_file_exists;
}

fn main() {
    // Initialize or load Database
    let mut projects: Vec<Project>;
    if check_for_db_file() {
        projects = load_from_file();
    } else {
        projects = Vec::new();
    }

    // Main CLI
    let args = Args::parse();
    match args.command {
        Commands::New => {
            let project = add_project(&projects);
            projects.push(project);
            println!("Your Project was added succesfully");
            save_to_file(&projects);
        }
        Commands::Delete => {
            delete_project(&mut projects);
            save_to_file(&projects);
        }
        Commands::List => {
            list_projects(&projects);
        }
    }
}
