use clap::{Parser, Subcommand};

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
        /// this is the actual todo
        #[arg(short , long)]
        name : Option<String>
    },

    ///Lists all the toods (you -all || -a to view previously completed todo's) 
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
    

    match &args.cmd{
        
        Some(Commands::Add {name }) => add_todo(name),
        Some(Commands::List {all }) => show_todos(all),
        Some(Commands::Remove {id}) => remove_todo(id),
        None=> println!("use the todo --help command to know how to use this application"),
        
    }


    fn add_todo(name : &Option<String>){
        println!("add a todo ,{:?}" , name);
    }

    fn show_todos(all : &Option<bool> ) {
         match  all {
            Some(true) => list_all_todos(),
            _=> list_pending_todos()
         }

    }
    
    fn remove_todo(id : &Option<u8>) {
        println!("todo with {:?} removed successfully" , id);
    }
    
    fn list_all_todos(){

    }
    fn list_pending_todos(){

    }
    
    

}

