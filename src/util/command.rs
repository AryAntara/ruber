use std::collections::HashMap;

use crate::util::parser::{Rule, ScriptParser};
use fantoccini::Client;
use pest::Parser;
use pest::iterators::Pairs;

mod dom;
mod flow;
mod nav;

#[derive(Debug, Clone)]
pub struct Commander {
    pub flows: HashMap<String, Vec<Command>>,
    pub state: HashMap<String, HashMap<String, String>>,
    pub commands: Vec<Command>,
    pub client: Client,
}

#[derive(Debug, Clone)]
pub enum Command {
    Goto(String),
    Flow(String, Vec<Command>),
    Click(String),
    TriggerEvent(String, String),
    Wait(u64, String),
    Fill(String, String),
    CallFlow(String),
    SelectFirst(String),
    Hangup,
    PressKey(String),
}

impl Commander {
    pub fn new(
        client: Client,
        commands: Vec<Command>,
        flows: HashMap<String, Vec<Command>>,
    ) -> Commander {
        Self {
            flows,
            state: HashMap::new(),
            commands,
            client,
        }
    }

    pub async fn exec(&mut self) {
        Box::pin(async {
            for cmd in self.commands.clone() {
                self.run(cmd).await;
            }
        })
        .await
    }

    pub async fn run(&mut self, cmd: Command) {
        let client = &self.client;
        match cmd {
            Command::Goto(url) => {
                println!("We're going to {url}");
                nav::go(url, &client).await
            }
            Command::Flow(name, cmds) => {
                let exists = self.flows.get(&name);
                if exists.is_some() {
                    println!("Flow \"{name}\" already exists");
                    return;
                }

                println!("Creating new flow \"{name}\"");
                let _ = &self.flows.insert(name, cmds);
            }
            Command::CallFlow(name) => {
                println!("Preparing for flow \"{name}\" and using it");
                let cmds = self.flows.get(&name);
                match cmds {
                    Some(cmds) => {
                        let mut little_commander =
                            Commander::new(client.clone(), cmds.clone(), self.flows.clone());
                        little_commander.exec().await;
                    }
                    None => {
                        println!("Flow \"{name}\" not found");
                    }
                }
            }
            Command::Click(selector) => dom::click(&client, selector).await,
            Command::TriggerEvent(event, selector) => {
                dom::trigger_event(&client, event, selector).await
            }
            Command::Wait(amount, unit) => {
                println!("Waiting for {amount} {unit}");
                flow::wait_for(amount, unit).await
            }
            Command::Fill(selector, value) => dom::fill(&client, selector, value).await,
            Command::SelectFirst(selector) => dom::select_first(&client, selector).await,
            Command::Hangup => {
                flow::hangup().await;
            }
            Command::PressKey(value) => dom::simulate_keyinput(&client, value).await,
        };
    }
}

pub fn parse(cmd: &str) -> Option<Vec<Command>> {
    let parsed = ScriptParser::parse(Rule::script, cmd);
    match parsed {
        Ok(mut pairs) => {
            let commands = parse_command(pairs.next().unwrap().into_inner());
            Some(commands)
        }
        Err(e) => {
            eprintln!("❌ Syntax Error:\n{}", e);
            None
        }
    }
}

pub fn parse_command(pairs: Pairs<'_, Rule>) -> Vec<Command> {
    let mut cmds: Vec<Command> = vec![];
    for pair in pairs {
        if pair.as_rule() == Rule::command {
            let inner_cmd = pair.clone().into_inner().next().unwrap();

            match inner_cmd.as_rule() {
                Rule::CmdGoto => {
                    let url = clean(inner_cmd.into_inner().next().unwrap().as_str());
                    cmds.push(Command::Goto(url.to_string()));
                }
                Rule::CmdClick => {
                    let selector = clean(inner_cmd.into_inner().next().unwrap().as_str());
                    cmds.push(Command::Click(selector.to_string()));
                }
                Rule::CmdFill => {
                    let mut inners = inner_cmd.into_inner();
                    let selector = clean(inners.next().unwrap().as_str());
                    let value = clean(inners.next().unwrap().as_str());
                    cmds.push(Command::Fill(selector.to_string(), value.to_string()));
                }
                Rule::CmdWait => {
                    let mut inners = inner_cmd.into_inner();
                    let amount = clean(inners.next().unwrap().as_str());
                    let unit = inners.next().unwrap().as_str();
                    cmds.push(Command::Wait(amount.parse().unwrap(), unit.to_string()));
                }
                Rule::CmdTriggerEvent => {
                    let mut inners = inner_cmd.into_inner();
                    let event = inners.next().unwrap().as_str();
                    let selector = inners.next().unwrap().as_str();
                    cmds.push(Command::TriggerEvent(
                        event.to_string(),
                        selector.to_string(),
                    ));
                }
                Rule::CmdFlowCreation => {
                    let cmd: Vec<&str> = pair.as_str().split('\n').collect();
                    let name = cmd[0]["create flow".len()..].trim();
                    let blocks = inner_cmd.into_inner();
                    let cmd = parse_command(blocks);
                    cmds.push(Command::Flow(name.to_string(), cmd));
                }
                Rule::CmdCallFlow => {
                    let cmd: Vec<&str> = pair.as_str().split('\n').collect();
                    let name = cmd[0]["using flow".len()..].trim();
                    cmds.push(Command::CallFlow(name.to_string()));
                }
                Rule::CmdSelectFirst => {
                    let mut inners = inner_cmd.into_inner();
                    let selector = clean(inners.next().unwrap().as_str());
                    cmds.push(Command::SelectFirst(selector.to_string()))
                }
                Rule::CmdHangup => {
                    cmds.push(Command::Hangup);
                }
                Rule::CmdPressKey => {
                    let mut inners = inner_cmd.into_inner();
                    let value = clean(inners.next().unwrap().as_str());
                    cmds.push(Command::PressKey(value.to_string()));
                }
                _ => {}
            }
        }
    }
    cmds
}

fn clean(str: &str) -> String {
    str.replace("'", "").replace('"', "")
}
