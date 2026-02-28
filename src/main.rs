mod models;
mod routes;
mod state;

fn print_todos(todos: &[state::TodoJson]) {
    for todo in todos {
        println!(
            "Todo(id={}, title={}, completed={})",
            todo.id, todo.title, todo.completed
        )
    }
}

fn main() {
    let mut state = state::AppState::new();

    for string in [
        "Brush teeth",
        "Have Breakfast",
        "Do the dishes",
        "Have lunch",
    ] {
        let _todo = state.create_todo(string);
    }

    print_todos(&state.get_todos());
}
