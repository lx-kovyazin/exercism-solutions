use std::collections::HashMap;

pub struct School(HashMap<u32, Vec<String>>);

impl School {
    pub fn new() -> School {
        Self(HashMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.0.entry(grade).or_insert(Vec::new()).push(student.to_owned());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.0.keys().cloned().collect();
        grades.sort_unstable();
        grades
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.0.get(&grade).cloned() {
            None => Vec::new(),
            Some(mut students) => {
                students.sort_unstable();
                students
            }
        }
        
    }
}
