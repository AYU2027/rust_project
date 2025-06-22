use genpdf::{Document, elements};
use std::io;

fn calculate_average(total_marks: f32, num_subjects: u32) -> f32 {
    total_marks / num_subjects as f32
}

fn assign_grade(average: f32) -> char {
    if average >= 90.0 {
        'A'
    } else if average >= 75.0 {
        'B'
    } else if average >= 60.0 {
        'C'
    } else {
        'D'
    }
}

fn generate_report_pdf(
    name: &str,
    total_marks: f32,
    num_subjects: u32,
    average: f32,
    grade: char,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load font family from local folder "fonts" (make sure you have ttf files here)
    let font_family = genpdf::fonts::from_files("./fonts", "LiberationSans", None)?;
    let mut doc = Document::new(font_family);

    doc.set_title("Report Card");
    doc.push(elements::Paragraph::new("Report Card").aligned(genpdf::Alignment::Center));
    doc.push(elements::Break::new(1));

    // Create a simple text block instead of a table to avoid complex decorators
    let info = format!(
        "Student Name: {}\nTotal Marks: {:.2}\nNumber of Subjects: {}\nAverage: {:.2}\nGrade: {}",
        name, total_marks, num_subjects, average, grade
    );
    doc.push(elements::Paragraph::new(info));

    doc.render_to_file("report_card.pdf")?;
    println!("Report card saved as 'report_card.pdf'");

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut name = String::new();
    let mut total_marks_input = String::new();
    let mut num_subjects_input = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name)?;

    println!("Enter total marks:");
    io::stdin().read_line(&mut total_marks_input)?;

    println!("Enter number of subjects:");
    io::stdin().read_line(&mut num_subjects_input)?;

    let total_marks: f32 = total_marks_input.trim().parse()?;
    let num_subjects: u32 = num_subjects_input.trim().parse()?;

    let average = calculate_average(total_marks, num_subjects);
    let grade = assign_grade(average);

    println!("\nStudent: {}", name.trim());
    println!("Average Marks: {:.2}", average);
    println!("Grade: {}", grade);

    generate_report_pdf(name.trim(), total_marks, num_subjects, average, grade)?;

    Ok(())
}
