#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, symbol_short, Symbol};

// Symbol for storage
const PROJECT_INFO: Symbol = symbol_short!("PROJ_INFO");

#[contracttype]
#[derive(Clone)]
pub struct ProjectDetails {
    pub title: String,
    pub description: String,
    pub deployed_by: String,
}

#[contract]
pub struct HydroponicContract;

#[contractimpl]
impl HydroponicContract {
    // Set project info
    pub fn set_project_info(env: Env, title: String, description: String, deployed_by: String) {
        let details = ProjectDetails {
            title,
            description,
            deployed_by,
        };
        env.storage().instance().set(&PROJECT_INFO, &details);
    }

    // Get project info
    pub fn get_project_info(env: Env) -> ProjectDetails {
        env.storage().instance().get(&PROJECT_INFO).unwrap_or(ProjectDetails {
            title: String::from_str(&env, "Not Set"),
            description: String::from_str(&env, "Not Set"),
            deployed_by: String::from_str(&env, "Unknown"),
        })
    }

    // Update description only
    pub fn update_description(env: Env, new_description: String) {
        let mut details = Self::get_project_info(env.clone());
        details.description = new_description;
        env.storage().instance().set(&PROJECT_INFO, &details);
    }
}
