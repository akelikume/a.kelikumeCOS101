fn main() {
    // Vector storing tuples of format: (Level, Category, Title)
    let aps_table = vec![
        ("APS 1-2", "Office Admin", "Intern"),
        ("APS 1-2", "Lawyer", "Paralegal"),
        ("APS 1-2", "Teacher", "Placement"),
        ("APS 3-5", "Office Admin", "Administrator"),
        ("APS 3-5", "Academic", "Research Assistant"),
        ("APS 3-5", "Lawyer", "Junior Associate"),
        ("APS 3-5", "Teacher", "Classroom Teacher"),
        ("APS 5-8", "Office Admin", "Senior Administrator"),
        ("APS 5-8", "Academic", "PhD Candidate"),
        ("APS 5-8", "Lawyer", "Associate"),
        ("APS 5-8", "Teacher", "Senior Teacher"),
        ("EL1 8-10", "Office Admin", "Office Manager"),
        ("EL1 8-10", "Academic", "Post-Doc Researcher"),
        ("EL1 8-10", "Lawyer", "Senior Associate 1-2"),
        ("EL1 8-10", "Teacher", "Leading Teacher"),
        ("EL2 10-13", "Office Admin", "Director"),
        ("EL2 10-13", "Academic", "Senior Lecturer"),
        ("EL2 10-13", "Lawyer", "Senior Associate 3-4"),
        ("EL2 10-13", "Teacher", "Deputy Principal"),
        ("SES", "Office Admin", "CEO"),
        ("SES", "Academic", "Dean"),
        ("SES", "Lawyer", "Partner"),
        ("SES", "Teacher", "Principal"),
    ];

    // Example query: Associate Lawyer
    let query_category = "Lawyer";
    let query_title = "Associate";

    let mut result_level = None;

    for (level, category, title) in &aps_table {
        if *category == query_category && *title == query_title {
            result_level = Some(*level);
            break;
        }
    }

    match result_level {
        Some(level) => println!(
            "An {} with the title '{}' holds position {}.",
            query_category, query_title, level
        ),
        None => println!("No matching position found for {} - {}.", query_category, query_title),
    }
}