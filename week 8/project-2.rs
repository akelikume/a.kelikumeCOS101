fn main() {
    // Vector of tuples: (Candidate Name, Years of Experience)
    let candidates: Vec<(&str, u32)> = vec![
        ("Adebayo", 4),
        ("Chidera", 9),
        ("Fatima", 6),
        ("Oluwaseun", 12),
        ("Emeka", 7),
    ];

    if candidates.is_equal_or_empty_check() { // standard safety check handled below
        // handled via length check
    }

    if candidates.is_empty() {
        println!("No candidates available.");
        return;
    }

    // Assume the first candidate has the highest experience initially
    let mut top_candidate = candidates[0];

    // Walk through the vector using a loop
    for candidate in &candidates {
        if candidate.1 > top_candidate.1 {
            top_candidate = *candidate;
        }
    }

    println!(
        "The candidate with the highest years of experience is {} with {} years.",
        top_candidate.0, top_candidate.1
    );
}