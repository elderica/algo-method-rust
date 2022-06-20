use core::cmp::Eq;
use core::fmt;
use std::cmp::Ordering;

#[derive(Eq)]
struct Student {
    pub name: String,
    pub math: usize,
    pub english: usize,
}

impl Student {
    fn new(name: String, math: usize, english: usize) -> Self {
        Self {
            name,
            math,
            english,
        }
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.name, self.math, self.english)
    }
}

impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.math == other.math && self.english == other.english
    }
}

impl Ord for Student {
    #[allow(clippy::if_same_then_else)]
    fn cmp(&self, other: &Self) -> Ordering {
        let selfsum = self.math + self.english;
        let othersum = other.math + other.english;
        if self.math > other.math {
            Ordering::Greater
        } else if self.math == other.math && selfsum < othersum {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut students: Vec<Student> = Vec::new();
    let n = scan_line();
    let n: usize = n[0];
    for _ in 0..n {
        let mut line = read_words();
        let english: usize = line
            .pop()
            .unwrap_or_else(|| String::from("0"))
            .parse()
            .unwrap_or(0);
        let math: usize = line
            .pop()
            .unwrap_or_else(|| String::from("0"))
            .parse()
            .unwrap_or(0);
        let name: String = line.pop().unwrap_or_else(|| String::from("0"));
        let s = Student::new(name, math, english);
        students.push(s);
    }
    students.sort();
    for student in students.iter().rev() {
        println!("{}", student);
    }
}

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    String::from(line.trim())
}

#[allow(dead_code)]
fn read_words() -> Vec<String> {
    read_line().split_whitespace().map(String::from).collect()
}

#[allow(dead_code)]
fn scan_line<F>() -> Vec<F>
where
    F: std::str::FromStr,
{
    read_line()
        .split_whitespace()
        .flat_map(|s| s.parse::<F>())
        .collect()
}
