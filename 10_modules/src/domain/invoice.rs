
pub struct Row {
    id: i32,
    pub amount: i32,
    pub text: String,
}

pub struct Head {
    pub id: i32,
    pub rows: Vec<Row>,
}

impl Head {
    pub fn new(amount: i32, text: &str) -> Head {
        let row = Row {
            id: 0,
            amount: amount,
            text: String::from(text),
        };
        Head {
            id: 0,
            rows: vec![row],
        }
    }
}
