#[derive(Default)]
pub enum Multiplicity {
    #[default]
    Maybe,
    Single,
    Bag,
    List,
}

#[derive(Default)]
pub enum Value {
    #[default]
    Bool,
    Integer,
    Float,
    Text,
    Symbol(String),
    Kind(String),
    Node(String),
}

#[derive(Default)]
pub struct Specification {
    roots: Vec<Root>,
    kinds: Vec<Kind>,
    nodes: Vec<Node>,
}

pub struct Root {
    pub name: String,
    pub multiplicity: Multiplicity,
    pub inhabitant: Value,
}

#[derive(Default)]
pub struct Kind {
    pub name: String,
    pub color: Option<(f32, f32, f32)>,
    pub inhabitants: Vec<Value>,
}

#[derive(Default)]
pub struct Node {
    pub name: String,
    pub color: Option<(f32, f32, f32)>,
    pub sockets: Vec<Socket>,
    pub outputs: Vec<Output>,
}

pub enum Direction {
    Above,
    Below,
    Before,
    After,
}

#[derive(Default)]
pub struct Socket {
    pub name: String,
    pub multiplicity: Multiplicity,
    pub inhabitant: Value,
    pub direction: Option<Direction>,
}

#[derive(Default)]
pub struct Output {
    pub name: String,
    pub multiplicity: Multiplicity,
    pub inhabitant: Value,
    pub direction: Option<Direction>,
}

fn ngs_ngs() -> Specification {
    Specification {
        kinds: vec![
            Kind {
                name: "Multiplicity".into(),
                inhabitants: vec![
                    Value::Symbol("Maybe".into()),
                    Value::Symbol("Single".into()),
                    Value::Symbol("Bag".into()),
                    Value::Symbol("List".into()),
                ],
                ..Default::default()
            },
            Kind {
                name: "Value".into(),
                inhabitants: vec![
                    Value::Symbol("Bool".into()),
                    Value::Symbol("Integer".into()),
                    Value::Symbol("Float".into()),
                    Value::Symbol("Text".into()),
                ],
                ..Default::default()
            },
        ],
        nodes: vec![],
        roots: vec![],
    }
}

struct Todo {
    pub text: String,
    pub done: bool,
    pub dependencies: Vec<Todo>,
}

type TodoFile = Vec<Todo>;

fn todo_example() -> TodoFile {
    vec![Todo {
        text: "Finish Specification Language".into(),
        done: false,
        dependencies: vec![
            Todo {
                text: "Write Example Todo".into(),
                done: true,
                dependencies: vec![],
            },
            Todo {
                text: "Write Desired Specification".into(),
                done: true,
                dependencies: vec![],
            },
            Todo {
                text: "Generalize Specification".into(),
                done: false,
                dependencies: vec![],
            },
        ],
    }]
}

// fn todo_ngs() -> Specification {
//     Specification {
//         roots: vec![(Value::Node("Todo".into()), Multiplicity::Bag)],
//         nodes: vec![Node {
//             name: "Todo".into(),
//             sockets: vec![
//                 Socket {
//                     name: "text".into(),
//                     inhabitant: Value::Text,
//                     ..Default::default()
//                 },
//                 Socket {
//                     name: "done".into(),
//                     inhabitant: Value::Bool,
//                     ..Default::default()
//                 },
//                 Socket {
//                     name: "dependencies".into(),
//                     inhabitant: Value::Node("Todo".into()),
//                     multiplicity: Multiplicity::List,
//                     ..Default::default()
//                 },
//             ],
//             ..Default::default()
//         }],
//     }
// }
