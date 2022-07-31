use std::collections::HashMap;


fn main() {
    let arguments: Vec<String> = std::env::args().collect();

    let todolist = TodoList::new();

    match arguments[1].as_str() {
        "add" => todolist.insert(arguments[2].clone()),
        "show" => todolist.print_all(),
        "do" => todolist.update(arguments[2].clone(), true),
        "undo" => todolist.update(arguments[2].clone(), false),
        _ => () // later implement help here
    }
}

struct TodoList {
    map: HashMap<String, bool>,
}

impl TodoList {
    fn new() -> TodoList {
        let mut map = HashMap::new();
        let todolist_content = match std::fs::read_to_string("todolist.txt") {
            Err(_) => String::from(""),
            Ok(result) => result
       };

        for line in todolist_content.lines() {
            let mut chunks = line.splitn(2, '\t');
            let task = chunks.next().expect("no task found");
            let done = match chunks.next().expect("no done found") {
                "true" => true,
                "false" => false,
                _ => false,
            };
            map.insert(task.to_owned(), done);
        }
        
        TodoList { map }
    }

    fn insert(mut self, task: String) {
       self.map.insert(task, false);
    }

    fn print_all(self) {
       let mut contents = String::new();

       contents.push_str("       TODO\n       ~~~~\n");

       for (task, done) in &self.map {
           contents.push('[');
           let check_char = match done { true => 'x', false => ' ' }; 
           contents.push(check_char);
           contents.push_str("] -- ");
           contents.push_str(task);
           contents.push('\n');
       } 

       print!("{}", contents);
    }

    fn update(mut self, task: String, done: bool) {
       let entry = self.map.entry(task).or_insert(false);
       *entry = done;
    }
}

impl Drop for TodoList {
    fn drop(&mut self) {
        let mut contents = String::new(); 

        for (task, done) in &self.map {
            contents.push_str(&task);
            let done_out = match done {
                true => "true", false => "false"
            };
            contents.push('\t');
            contents.push_str(done_out);
            contents.push('\n');
        }
        
        let _ = std::fs::write("todolist.txt", contents);
    }
}


