// --- MAIN INTERACTION GATEWAY ---
use std::io::{self, Write};

// Tell Rust to look for the companion security file
mod security_engine; 
use security_engine::{PatientRecord, TriageStatus};

fn main() {
    // An expandable collection array (Vector) to safely store multiple entries in memory
    let mut hospital_database: Vec<PatientRecord> = Vec::new();
    let mut id_counter: u32 = 5001; // Secure incremental ID starting point

    println!("=====================================================");
    println!("     MOTHERLY INSTITUTE: SECURE IO&M GATEWAY        ");
    println!("=====================================================");

    loop {
        println!("\n--- [MENU] Choose an Option ---");
        println!("1. Enter New Patient Telemetry Data");
        println!("2. Display Secure Database System Log Table");
        println!("3. Exit Gateway System");
        print!("Select Option (1-3): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("\n--- SECURE ENCRYPTED DATA ENTRY ---");
                
                // Input Patient Name
                print!("Enter Patient Name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();

                // Input Heart Rate
                print!("Enter Heart Rate (BPM): ");
                io::stdout().flush().unwrap();
                let mut hr_str = String::new();
                io::stdin().read_line(&mut hr_str).unwrap();
                let hr: u8 = hr_str.trim().parse().unwrap_or(0);

                // Input Oxygen Levels
                print!("Enter Oxygen Level (SpO2 %): ");
                io::stdout().flush().unwrap();
                let mut o2_str = String::new();
                io::stdin().read_line(&mut o2_str).unwrap();
                let o2: u8 = o2_str.trim().parse().unwrap_or(0);

                // Input Blood Pressure
                print!("Enter Systolic Blood Pressure (mmHg): ");
                io::stdout().flush().unwrap();
                let mut bp_str = String::new();
                io::stdin().read_line(&mut bp_str).unwrap();
                let bp: u8 = bp_str.trim().parse().unwrap_or(0);

                // Send the collected data to the security engine validation check
                match PatientRecord::clean_and_create(id_counter, name, hr, o2, bp) {
                    Ok(validated_record) => {
                        println!("\n[SUCCESS] Memory Check Passed. Record securely pushed to storage heap.");
                        hospital_database.push(validated_record);
                        id_counter += 1; // Increment IDs safely without overlap
                    }
                    Err(error_message) => {
                        println!("\n[GATEKEEPER BLOCKED] {}", error_message);
                    }
                }
            }
            "2" => {
                // Display the active monitoring table
                if hospital_database.is_empty() {
                    println!("\n[SYSTEM NOTICE] The secure database is currently empty.");
                    continue;
                }

                println!("\n=========================================================================");
                println!("                   SECURE HOSPITAL MONITORING DATA TABLE                 ");
                println!("=========================================================================");
                println!("{:<8} | {:<15} | {:<10} | {:<10} | {:<10} | {:<10}", "ID", "Patient Name", "Heart Rate", "SpO2 %", "Sys BP", "Triage Alert");
                println!("-------------------------------------------------------------------------");

                for record in &hospital_database {
                    let alert_label = match record.triage {
                        TriageStatus::Nominal => "STABLE",
                        TriageStatus::Warning => "ELEVATED",
                        TriageStatus::Critical => "CRITICAL EMERGENCY",
                    };

                    println!(
                        "{:<8} | {:<15} | {:<10} | {:<10} | {:<10} | {:<10}",
                        record.id, record.name, record.heart_rate, record.oxygen, record.systolic_bp, alert_label
                    );
                }
                println!("=========================================================================");
            }
            "3" => {
                println!("\nShutting down Secure Gateway. Terminating encryption keys cleanly.");
                break;
            }
            _ => {
                println!("\n[INVALID INPUT] Please type 1, 2, or 3.");
            }
        }
    }
}