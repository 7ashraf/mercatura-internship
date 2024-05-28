#[macro_use]
extern crate yew;

use yew::prelude::*;
type Context = ();
struct Model {
    input : String,
    todos: Vec<String>,
}

enum Msg{
    Add,
    Update(String),
    Remove(usize),
    RemoveAll,
    Nothing,
}

fn update(_: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Add => {
            model.todos.push(model.input.clone());
            model.input = "".to_string();
        }
        Msg::Update(value) => {
            model.input = value;
        }
        Msg::Remove(index) => {
            model.todos.remove(index);
        }
        Msg::RemoveAll => {
            model.todos.clear();
        }
        Msg::Nothing => {}
    }
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            <h1>{ "Todo App" }</h1>
            <input
                value=&model.input,
                oninput=|e| Msg::Update(e.value),
                onkeypress=|e| {
                    if e.key() == "Enter" { Msg::Add } else { Msg::Nothing }
                },
            />
            <button onclick=|_| Msg::Add,>{ "Add" }</button>
            <button onclick=|_| Msg::RemoveAll,>{ "Remove All" }</button>
            <ul>
                { for model.todos.iter().enumerate().map(view_todo) }
            </ul>
        </div>
    }
}

fn main() {
    let model = Model {
        todos: vec![],
        input: "".to_string(),
    };

    program(model, update, view);
    println!("Hello, world!");
}
