use rusqlite::{Connection, Result, params};
use std::env;
use std::fs;
use home::home_dir;
fn main() -> Result<()> {
    //setup connection
    let home = home_dir().expect("Failed to get home directory...");
    let db_path = home.join(".todo");
    let conn = Connection::open(db_path)?;
    conn.execute(
        "create table if not exists todo_list (
        id integer primary key,
        task text not null, 
        completed integer not null)", 
        [],
    )?;
    //args handling
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || (args.len() > 1 && &args[1] == "help") {
        println!("Usage: {} <action> <task|id>", args[0]);
        println!("Actions: add, done, list, wipe");
        return Ok(());
    }

    let action = &args[1];

    if action == "list" {
        let mut stmt = conn.prepare(
            "SELECT id, task from todo_list WHERE completed = 0"
        )?;
        let task_iter = stmt.query_map([], |row| {
            let id: i32 = row.get(0)?;
            let task: String = row.get(1)?;
            Ok((id, task))
        })?;

        for task in task_iter {
            match task {
                Ok((id, task)) => println!("{} - {}", id, task),
                Err(e) => eprintln!("Error printing task!"),
            };
        }

        std::process::exit(0);
    }
    
    if action == "add" {
        let item = args[2..].join(" ");
        conn.execute(
            "INSERT INTO todo_list (task, completed) values ($1, 0)", 
            params![item],
        )?;
    }
    if action == "done" {
        //update value of completed to 1
        for id in &args[2..] {
            conn.execute(
                "UPDATE todo_list SET completed = 1 WHERE id = ?1",
                params!(id),
            )?;
        }
    }
    Ok(())
}
