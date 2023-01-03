use std::collections::HashMap;
use std::io;

pub struct Student {
    id: u32,
    name: String,
    age: u32
}

impl Student {
    fn show_list_students(&self) {
        print!("Infor's Student: \nMSSV = {} \nName = {} \nAge = {}\n\n\n", self.id, self.name, self.age);
    }
}

pub struct Students {
    list: HashMap<u32, Student>,
}

impl Students {
    // initialize new hashmap 
    fn new() -> Self {
        return Self { 
            list: HashMap::new(),
        }
    }

    // adding student
    pub fn add(&mut self, student: Student) {
        self.list.insert(student.id, student);
    }   

    // viewing student
    #[allow(unused_variables)]
    pub fn view(&self) {
        #[allow(non_snake_case)]
        for (MSSV, info) in self.list.iter() {
            info.show_list_students();
        }
    }

    // editting student
    pub fn edit(&mut self, id: &u32, name: String, age: u32) -> bool {
        match self.list.get_mut(id) {
            Some(student) => {
                student.name = name;
                student.age = age;
                return true
            },
            None => return false,
        }
    }

    // deleting student
    pub fn delete(&mut self, id: &u32) -> bool {
        // self.list.remove(id);
        // return true;
        return self.list.remove(id).is_some();
    }
}

mod manager {
    use crate::*;

    // create new student function
    pub fn add_students(students: &mut Students) {
        
        println!("Enter MSSV:");
        let id: u32 = match get_input() {
            Some(input) => input.parse::<u32>().unwrap(),
            None => return,
        };

        println!("Enter Name's student:");
        let name: String = match get_input() {
            Some(str) => str,
            None => return,
        };

        println!("Enter age's student:");
        let age: u32 = match get_input() {
            Some(num) => num.parse::<u32>().unwrap(),
            None => return,
        };

        let student: Student = Student { 
            id: id, 
            name: name, 
            age: age,
        };

        Students::add(students, student);
    }

    // view student's info function
    pub fn view_students(students: &Students) {
        Students::view(students);
    }

    // view student's info is pointed
    // pub fn view_pointed_student()

    // edit student's info function
    pub fn edit_students(students: &mut Students) {

        println!("Enter MSSV (You want to edit):");
        let id: u32 = match get_input() {
            Some(input) => input.parse::<u32>().unwrap(),
            None => return,
        };

        println!("Enter new name's student:");
        let name: String = match get_input() {
            Some(str) => str,
            None => return,
        };

        println!("Enter new age's student:");
        let age: u32 = match get_input() {
            Some(num) => num.parse::<u32>().unwrap(),
            None => return,
        };

        if students.edit(&id, name, age) {
            println!("Student's information is changed!");
        } else {
            println!("Oops. Can not find the student!!!");
        }

    }

    // delete student's info function
    pub fn delete_students(students: &mut Students) {
        println!("Enter MSSV (You want to delete):");
        let id: u32 = match get_input() {
            Some(input) => input.parse::<u32>().unwrap(),
            None => return,
        };

        if students.delete(&id) {
            println!("The student's information is deleted.");
        } else {
            println!("Oops. Can not find the student!!!");
        }

    }
}


enum Manager {
    AddStudent,
    ViewStudent,
    EditStudent,
    DeleteStudent,
}

impl Manager {
    pub fn show() {
        println!("");
        println!("== Manager Panel ==");
        println!("");
        println!("1. Add Student");
        println!("2. View Students");
        println!("3. Edit Student");
        println!("4. Delete Student");
        println!("5. Exit Program!");
        println!("");
        println!("Please Enter Your Choice:");
        println!("");
    }

    pub fn choice(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::AddStudent),
            "2" => Some(Manager::ViewStudent),
            "3" => Some(Manager::EditStudent),
            "4" => Some(Manager::DeleteStudent),
            _ => None,
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap();

    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

pub fn run_app() {
    let mut students: Students = Students::new();
    loop {
        Manager::show();
        let input: Option<String> = get_input();
        match Manager::choice(input.unwrap().as_str()) {
            Some(Manager::AddStudent) => manager::add_students(&mut students),
            Some(Manager::ViewStudent) => manager::view_students(&students),
            Some(Manager::EditStudent) => manager::edit_students(&mut students),
            Some(Manager::DeleteStudent) => manager::delete_students(&mut students),
            None => break,
        }
    }
}

fn main() {
    run_app();

    println!("The program is ended!!!");
}