use std::collections::HashMap;

use fantoccini::Client;

mod dom;
mod flow;
mod nav;

#[derive(Debug)]
pub struct Commander {
    pub modules: HashMap<String, Vec<Command>>,
    pub state: HashMap<String, HashMap<String, String>>,
    pub commands: Vec<Command>,
    pub client: Client,
}

#[derive(Debug)]
pub enum Command {
    Goto(String),
    Module(String, Vec<Command>),
    Click(String),
    TriggerEvent(String, String),
    Wait(u64, String),
    Fill(String, String),
}

impl Commander {
    pub fn new(client: Client) -> Commander {
        Self {
            modules: HashMap::new(),
            state: HashMap::new(),
            commands: Vec::new(),
            client,
        }
    }
}

pub async fn exec(cmd: Command, client: Client) {
    match cmd {
        Command::Goto(url) => nav::go(url, &client).await,
        Command::Module(name, cmds) => println!("module {}", name),
        Command::Click(selector) => dom::click(&client, selector).await,
        Command::TriggerEvent(event, selector) => {
            dom::trigger_event(&client, event, selector).await
        }
        Command::Wait(amount, unit) => flow::wait_for(amount, unit).await,
        Command::Fill(selector, value) => dom::fill(&client, selector, value).await,
    }
}
