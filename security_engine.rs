// --- SECURITY ENGINE MODULE ---

// 1. Definition Table for Device Status
#[derive(Debug, Clone)]
pub enum TriageStatus {
    Nominal,
    Warning,
    Critical,
}

// 2. Main Data Structure Table for Patient Vitals
#[derive(Debug, Clone)]
pub struct PatientRecord {
    pub id: u32,
    pub name: String,
    pub heart_rate: u8,
    pub oxygen: u8,
    pub systolic_bp: u8,
    pub triage: TriageStatus,
}

impl PatientRecord {
    // Constructor method that validates incoming data bounds before allowing memory allocation
    pub fn clean_and_create(id: u32, name: String, hr: u8, o2: u8, bp: u8) -> Result<Self, &'static str> {
        // Strict Security Constraints Check
        if name.trim().is_empty() {
            return Err("REJECTED: Patient name cannot be blank.");
        }
        if hr == 0 || hr > 240 {
            return Err("SECURITY ALERT: Out-of-bounds Heart Rate detected. Potential sensor tamper.");
        }
        if o2 > 100 {
            return Err("SECURITY ALERT: Impossible Oxygen Saturation (>100%). Packet dropped.");
        }
        if bp < 40 || bp > 230 {
            return Err("REJECTED: Blood pressure telemetry signature corrupt or highly critical.");
        }

        // Automatic Triage Classification Matrix
        let assigned_triage = if o2 < 90 || hr > 130 {
            TriageStatus::Critical
        } else if o2 < 94 || hr > 100 {
            TriageStatus::Warning
        } else {
            TriageStatus::Nominal
        };

        // Data is validated safely. Return the finalized Record object.
        Ok(PatientRecord {
            id,
            name: name.trim().to_string(),
            heart_rate: hr,
            oxygen: o2,
            systolic_bp: bp,
            triage: assigned_triage,
        })
    }
}