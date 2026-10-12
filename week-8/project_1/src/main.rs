#[derive(Debug)]
struct PublicServiceRecord {
    level: &'static str,
    office_admin: &'static str,
    academic: &'static str,
    lawyer: &'static str,
    teacher: &'static str,
}

struct PsLevelChecker {
    records: Vec<PublicServiceRecord>,
}

impl PsLevelChecker {
    fn new() -> Self {
        let records = vec![
            PublicServiceRecord {
                level: "APS 1-2",
                office_admin: "Intern",
                academic: "-",
                lawyer: "Paralegal",
                teacher: "Placement",
            },
            PublicServiceRecord {
                level: "APS 3-5",
                office_admin: "Administrator",
                academic: "Research Assistant",
                lawyer: "Junior Associate",
                teacher: "Classroom Teacher",
            },
            PublicServiceRecord {
                level: "APS 5-8",
                office_admin: "Senior Administrator",
                academic: "PhD Candidate",
                lawyer: "Associate",
                teacher: "Senior Teacher",
            },
            PublicServiceRecord {
                level: "EL1 8-10",
                office_admin: "Office Manager",
                academic: "Post-Doc Researcher",
                lawyer: "Senior Associate 1-2",
                teacher: "Leading Teacher",
            },
            PublicServiceRecord {
                level: "EL2 10-13",
                office_admin: "Director",
                academic: "Senior Lecturer",
                lawyer: "Senior Associate 3-4",
                teacher: "Deputy Principal",
            },
            PublicServiceRecord {
                level: "SES",
                office_admin: "CEO",
                academic: "Dean",
                lawyer: "Partner",
                teacher: "Principal",
            },
        ];

        Self { records }
    }

    // Validates and returns the APS/SES level given a profession category and job title
    fn check_level(&self, category: &str, title: &str) -> Option<&str> {
        for record in &self.records {
            let matched = match category.to_lowercase().as_str() {
                "office admin" => record.office_admin == title,
                "academic" => record.academic == title,
                "lawyer" => record.lawyer == title,
                "teacher" => record.teacher == title,
                _ => false,
            };

            if matched {
                return Some(record.level);
            }
        }
        None
    }
}

fn main() {
    let checker = PsLevelChecker::new();

    // Example test case: Associate Lawyer with 5 to 8 years of experience
    let category = "Lawyer";
    let title = "Associate";

    match checker.check_level(category, title) {
        Some(level) => println!("An {} holds position {}.", title, level),
        None => println!("No matching record found for {} in {}.", title, category),
    }
}