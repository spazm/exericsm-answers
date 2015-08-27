use std::collections::BTreeMap;

pub struct School(BTreeMap<u32,Vec<String>>);

impl School {
   pub fn new() -> School {
       School(BTreeMap::new())
   }

   pub fn add(&mut self, grade: u32, student: &str) -> (){
       self.0.entry(grade).or_insert(Vec::new()).push(student.to_string());
   }

   // TODO: sort numerically
   pub fn grades(&self) -> Vec<u32> {
       self.0.keys().map(|k| *k).collect()
   }

   // TODO: sort alphabetically
   pub fn grade(&self, grade: u32) -> Option<&Vec<String>> {
       self.0.get(&grade)
   }
}
