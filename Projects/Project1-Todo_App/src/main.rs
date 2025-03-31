use structopt::StructOpt;
mod tasks;

#[derive(StructOpt)]
#[structopt(name="cli_todo",about="A simple CLI todo app")]
enum Command {
    #[structopt(about="Add a new task")]
    Add {description:String},

    #[structopt(about="List all tasks")]
    List,

    #[structopt(about="Mark a task as done")]
    Done {id:usize},
}

fn main() {
   let args=Command::from_args();
   match args {
       Command::Add { description }=>tasks::add_task(description),
       Command::Done { id }=>tasks::mark_as_done(id),
       Command::List=>tasks::list_tasks(),
   } 
}
