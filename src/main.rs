use std::{process,  sync::atomic::{AtomicU8, Ordering}};
use clap::{Parser, Subcommand};
use chrono ::{ DateTime, Utc};

#[derive(Parser)]
#[command(name = "todo-cli")]
#[command(author = "dnotnice", version = "1.0", about = "Your cli to-do application", long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Commands>
}
//global counter for id
static COUNTER: AtomicU8= AtomicU8::new(1);

struct Todo {
    id : u8,
    name : String ,
    created_at : DateTime<Utc>,
    completed_at : Option<DateTime<Utc>>,
    done : bool,

}
impl Todo {
    pub fn new(name: String) -> Self {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        Todo {
            id,
            name,
            created_at: Utc::now(),
            completed_at : None,
            done: false,
        }
    }
}
    
struct TodoManager {
    todo_storage: Vec<Todo>
}
impl TodoManager {
    pub fn new()->Self {
        TodoManager{todo_storage : Vec::new()}
    }
    pub fn add(&mut self ,todo : Todo){
        self.todo_storage.push(todo);
    } 


    pub fn delete(&mut self, id: u8) {
        if let Some(pos) = self.todo_storage.iter().position(|todo: &Todo| todo.id == id) {
            self.todo_storage.remove(pos);
            println!("Todo item with id {} has been removed.", id);
        } else {
            println!("Todo item with id {} not found.", id);
        }
    }
    
    pub fn list(&mut self ,val : bool){
        if val {
            for todo in &self.todo_storage {
                println!("Todo {} : {} {} {}" , todo.id , todo.name , todo.created_at , todo.done);
            }
        }else{
            for todo in &self.todo_storage {
               if todo.done == false {
                println!("Todo {} : {} {} {}" , todo.id , todo.name , todo.created_at , todo.done);
                }
            }
        }
    }

}

#[derive(Subcommand, Debug, Clone)]
enum Commands {

    ///Adds a new todo (provide a sentence to save as a todo)
    Add {
        /// this is the todo that you want to save
        #[arg(short , long)]
        name : Option<String>
    },

    ///Lists all the toods (use -all || -a to view previously completed todo's) 
    List { 
        /// the  -all flag 
        #[arg(short , long)]
        all : Option<bool>
    },
    ///Removes a todo 
    Remove {
        ///The id of the todo to remove
        #[arg(short , long)]
        id : Option<u8>
    } ,
     
}

fn main() {
    let args = Args::parse();
    let mut todo_storage = TodoManager::new();

    match &args.cmd{
        
        Some(Commands::Add {name }) => add_todo( name , &mut todo_storage),
        Some(Commands::List {all }) => show_todos(all),
        Some(Commands::Remove {id}) => remove_todo(id,),
        _=> println!("use --help for assistance"),
        
    }


}
fn add_todo(name: &Option<String>, todo_storage: &mut TodoManager) {
    match name {
        Some(name) => {
                todo_storage.add(Todo::new(name.clone()));
                println!("Todo added successfully.");
            
        }
        None => println!("Please provide a title to add."),
    }
}

fn show_todos(all : &Option<bool> ) {
    //  match all {
    //     Some(true) => list_all_todos(),
    //     _=> list_pending_todos()
    //  }

}

fn remove_todo(id : &Option<u8>) {
    println!("todo with {:?} removed successfully" , id);
}

