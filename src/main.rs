use std:: sync::atomic::{AtomicU8, Ordering};
use clap::{Parser, Subcommand};
use chrono ::{ DateTime, Utc};

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
        println!("adding task ");
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
            for todo in &self.todo_storage {
                if val || !todo.done {
                print!("{} {} :  {}  {} " ,todo.id,  todo.name, todo.created_at, todo.done);
                if val && todo.done { 
                    if let Some(completed_at) = todo.completed_at {
                        print!("{}" , completed_at);
                    } 
                }
                println!();
            }
        }  
    }

    pub fn mark( &mut self , id : u8) {
        if let Some(pos) = self.todo_storage.iter().position(|todo: &Todo| todo.id == id){
            if let Some(todo) = self.todo_storage.get_mut(pos){
                if todo.done == false  {
                    todo.completed_at = Some(Utc::now());
                    todo.done = true;
                    println!("todo with id {} marked done" , id);
                }else {
                    println!("todo is already marked true");
                }
            }
            
        }else{
            println!("Todo item with id {} not found" , id);
        }
    }
}

#[derive(Parser)]
#[command(name = "todo-cli")]
#[command(author = "dnotnice", version = "1.0", about = "Your cli to-do application", long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Commands>
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {

    ///Adds a new todo (provide a sentence to save as a todo)
    Add {
        /// this is the todo that you want to save
        #[arg(short , long)]
        name : String
    },

    ///Lists all the toods (use -all || -a to view previously completed todo's) 
    List { 
        /// the  -all flag 
        #[arg(short , long)]
        all : bool
    },
    ///Removes a todo 
    Remove {
        ///The id of the todo to remove
        #[arg(short , long)]
        id : u8
    } ,
    ///mark a todo as completed 
    Done {
        ///the id of the todo to mark done
        #[arg(short ,long)]
        id : u8
    },
    ///exit the application
    End
     
}

fn main() {
    println!("Welcome to todo-cli ❤️ , a terminal based todo list");
    let mut todo_storage = TodoManager::new();
    
    loop{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "quit" {
            println!("Exiting");
            break;
        }

        let args: Vec<String> = input
            .trim()
            .split_whitespace()
            .map(String::from)
            .collect();

        let args = Args::parse_from(std::iter::once("program".to_string()).chain(args));

         match args.cmd{
            Some(Commands::Add {ref name}) => todo_storage.add( Todo::new(name.clone())),
            Some(Commands::List {all }) => todo_storage.list(all),
            Some(Commands::Remove {id}) => todo_storage.delete(id),
            Some(Commands::Done { id }) => todo_storage.mark(id),
            Some(Commands::End) => {
                                println!("exiting the program");
                                break;  }
            None => println!("use --help or -h "),
        }
    }
}

