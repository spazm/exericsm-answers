use std::collections::HashMap;

pub struct School(HashMap<u32,Vec<String>>);

impl School {
   pub fn new() -> School {
       School(HashMap::new())
   }

   /// Add student to sorted list of students in grade.
   pub fn add(&mut self, grade: u32, student: &str) -> (){
       let mut students = self.0.entry(grade).or_insert(Vec::new());
       students.push(student.to_string());
       students.sort()
   }

   /// Sorted list of grades
   pub fn grades(&self) -> Vec<u32> {
       let mut grades: Vec<u32> = self.0.keys().map(|k| *k).collect();
       grades.sort();
       grades
   }

   /// Sorted list of students in a given grade
   pub fn grade(&self, grade: u32) -> Option<&Vec<String>> {
       self.0.get(&grade)
   }
}
