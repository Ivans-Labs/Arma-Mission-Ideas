mod locations;

use std::io::{self, Write};
use rand::Rng;

trait MissionGenerator {
    fn generate_brief(&self) -> String;
    fn generate_description(&self) -> String;
    fn generate_idea(&self) -> String;
    fn generate_enemy(&self) -> String;
    fn generate_location(&self) -> String;
}

trait GenerateBrief {
    fn generate_brief(&self) -> String;
}

struct BriefGenerator;

impl GenerateBrief for BriefGenerator {
    fn generate_brief(&self) -> String {
        String::from("Generated mission brief using AI")
    }
}

trait GenerateDescription {
    fn generate_description(&self) -> String;
}

struct DescriptionGenerator;

impl GenerateDescription for DescriptionGenerator {
    fn generate_description(&self) -> String {
        String::from("Generated mission description using AI")
    }
}

struct MyMissionGenerator;

impl MissionGenerator for MyMissionGenerator {
    fn generate_brief(&self) -> String {
        let brief_generator = BriefGenerator;
        brief_generator.generate_brief()
    }

    fn generate_description(&self) -> String {
        let description_generator = DescriptionGenerator;
        description_generator.generate_description()
    }

    fn generate_idea(&self) -> String {
        String::from("Generated mission idea")
    }

    fn generate_enemy(&self) -> String {
        let factions = vec!["Opfor", "Blufor", "Independent"];
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..factions.len());
        factions[index].to_string()
    }

    fn generate_location(&self) -> String {
        locations::generate_location()
    }
}

fn main() {
    let mission_generator = MyMissionGenerator;
    let mut input = String::new();

    println!("Mission Generator CLI");
    println!("Available commands: brief, description, idea, enemy, location, all, quit");

    loop {
        print!("Enter command: ");
        io::stdout().flush().unwrap(); // Make sure the prompt is displayed immediately
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "brief" => {
                let brief = mission_generator.generate_brief();
                println!("Mission Brief: {}", brief);
            },
            "description" => {
                let description = mission_generator.generate_description();
                println!("Mission Description: {}", description);
            },
            "idea" => {
                let idea = mission_generator.generate_idea();
                println!("Mission Idea: {}", idea);
            },
            "enemy" => {
                let enemy = mission_generator.generate_enemy();
                println!("Enemy Faction: {}", enemy);
            },
            "location" => {
                let location = mission_generator.generate_location();
                println!("Mission Location: {}", location);
            },
            "all" => {
                let brief = mission_generator.generate_brief();
                let description = mission_generator.generate_description();
                let idea = mission_generator.generate_idea();
                let enemy = mission_generator.generate_enemy();
                let location = mission_generator.generate_location();

                println!("Full Mission Details:");
                println!("Brief: {}", brief);
                println!("Description: {}", description);
                println!("Idea: {}", idea);
                println!("Enemy: {}", enemy);
                println!("Location: {}", location);
            },
            "quit" => {
                println!("Exiting Mission Generator CLI...");
                break;
            }
            _ => println!("Unrecognized command, please try again."),
        }

        input.clear(); // Clear the input for the next command
    }
}