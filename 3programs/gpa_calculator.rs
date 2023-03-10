// Fancy GPA Calculator
// 3 Programs Assignment
// ITCS4102
//
// Authors: Walker Clem | Sam Edwards | Jordan Smiley | Max Ezzell
//
// 1:   4 data types & 2 built in methods for each:
//          float32 -   sqrt(), powf()
//          String -    trim(), to_uppercase()
//          bool -      not(), then()
//          u32 -       parse(), clamp()
// 2:   2 major data structures:
//          vector{}, struct{}
//      2 major control structures
//          match{}, loop{}, if{}, for{}
// 3 -  Exception handling or Concurrency
//          expect(), panic() - mean exception handling, will panic and quit if error occurs

use std::io;
use std::ops::Not;

struct Course {
    name: String,
    grade: String,
    credits: u32,
    gp: f32,
}

fn grade_to_gpa(grade: &str) -> f32 {
    match grade {
        "A" => 4.0, "A-" => 3.7,
        "B+" => 3.3, "B" => 3.0, "B-" => 2.7,
        "C+" => 2.3, "C" => 2.0, "C-" => 1.7,
        "D+" => 1.3, "D" => 1.0,
        "F" => 0.0,
        _ => {
            panic!("Invalid grade entered.");
        }
    }
}

fn chancellors_list(gpa: &f32, credits: &u32) -> bool {
    let award: bool = true;
    if gpa >= &3.8 && credits >= &12 {
        return award;
    }
    return award.not()
}

fn deans_list(gpa: &f32, credits: &u32) -> bool {
    let award: bool = true;
    if gpa >= &3.4 && gpa < &3.8 && credits >= &12 {
        return award;
    }
    return award.not();
}

fn calculate_gpa(courses: &[Course]) -> f32 {
    let mut total_points = 0.0;
    let mut total_credits = 0;

    for course in courses {
        total_points += course.gp * course.credits as f32;
        total_credits += course.credits;
    }

    if total_credits == 0 {
        return 0.0; // save from /0 error
    }

    let gpa = total_points / total_credits as f32;

    chancellors_list(&gpa, &total_credits).then(|| { println!("Congratulations! You made the Chancellors list!"); });
    deans_list(&gpa, &total_credits).then(|| { println!("Congratulations! You made the Deans list!"); });

    return gpa;
}

fn calculate_std_deviation(courses: &[Course]) -> f32 {
    let gpas: Vec<f32> = courses.iter().map(|course| course.gp).collect();
    let mean = gpas.iter().sum::<f32>() / gpas.len() as f32;
    let squared_diffs = gpas.iter().map(|gpa| (gpa - mean).powf(2.0));
    let mean_squared_diff = squared_diffs.sum::<f32>() / gpas.len() as f32;
    return mean_squared_diff.sqrt()
}

fn print_transcript(courses: &[Course]) {
    println!("\nTranscript:");
    println!("{:<15} {:<10} {}", "Course", "Grade", "Credits");

    for course in courses {
        println!("{:<15} {:<10} {}", course.name, course.grade, course.credits);
    }

    println!("\nTotal credits: {}", courses.iter().map(|course| course.credits).sum::<u32>());
    println!("GPA: {:.2}", calculate_gpa(courses));
    println!("The standard deviation of your GPA is: {:.2}", calculate_std_deviation(courses));
}

fn main() {
    let mut courses = Vec::new();

    loop {
        let mut name = String::new();
        let mut grade = String::new();
        let mut credits = String::new();

        println!("Course name (or 'done' to exit):");
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read course name.");
        let name = name.trim().to_uppercase();

        if name == "DONE" {
            break;
        }

        println!("Credit hours between 1 and 5:");
        io::stdin()
            .read_line(&mut credits)
            .expect("Failed to read credit hours.");
        let credits: u32 = credits
            .trim()
            .parse()
            .expect("Invalid credit hours entered.");
        // 'clamps' credit hours between 1 and 5
        let credits = credits.clamp(1, 5);

        println!("Enter the Grade (A, A-, B+, B, B-, C+, C, C-, D+, D, or F):");
        io::stdin()
            .read_line(&mut grade)
            .expect("Failed to read grade.");
        let grade = grade.trim().to_uppercase();
        let gp = grade_to_gpa(&grade);

        let course = Course {
            name,
            grade,
            credits,
            gp,
        };
        courses.push(course);
    }

    print_transcript(&courses);
}
