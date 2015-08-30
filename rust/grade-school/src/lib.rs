use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct School(HashMap<u32,Vec<String>>);

impl School {
   pub fn new() -> School {
       School(HashMap::new())
   }

   pub fn add(&mut self, grade: u32, student: &str) -> (){
       self.0.entry(grade).or_insert(Vec::new()).push(student.to_string());
   }

   /// Sorted list of grades
   pub fn grades(&self) -> Vec<u32> {
       let mut grades: Vec<u32> = self.0.keys().map(|k| *k).collect();
       grades.sort();
       grades
   }

   // TODO: sort alphabetically
   /// Sorted list of students in a given grade
   /// We cannot save the sorted list back to grade if called with an immutable self.
   pub fn grade(& mut self, grade: u32) -> Option<&Vec<String>> {
       match self.0.entry(grade) {
           Entry::Vacant(_) => None,
           Entry::Occupied(entry) => { let mut students = entry.into_mut(); students.sort(); Some(students) }
       }
   }
}
