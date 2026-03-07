use std::{collections::HashMap, env};

use crate::util::{
    command,
    command::{Command, Commander},
    fs,
    parser::{self, ScriptParser},
};
use fantoccini::{Client, ClientBuilder};
use parser::Rule;
use pest::{Parser, iterators::Pairs};
use serde_json::json;

mod util;

#[tokio::main]
async fn main() {
    let mut caps = serde_json::Map::new();
    let current_dir = env::current_dir().expect("Gagal mendapatkan direktori saat ini");
    let chrome_path = current_dir.join("./browsers/chrome-linux64/chrome");
    let chrome_path_str = chrome_path.to_str().expect("Path tidak valid");
    caps.insert(
        "goog:chromeOptions".to_string(),
        json!({
            "binary": chrome_path_str
        }),
    );

    let client = ClientBuilder::native()
        .capabilities(caps)
        .connect("http://localhost:44877")
        .await
        .expect("Cannot connect to the client.");
    let mut commander = Commander::new(client.clone());
    parse(
        &mut commander,
        "go to http://localhost
fill element '[name=\"email\"]' with superadmin@neomr.id
fill element '[name=\"password\"]' with 123456
click element 'button.kt-btn.kt-btn-primary'
wait for '100' seconds

define login
go to 'youtube.com'
wait for '5' seconds
end of module",
    );

    for cmd in commander.commands {
        command::exec(cmd, client.clone()).await;
    }

    let _ = client.close().await;
}

fn parse(commander: &mut Commander, cmd: &str) -> Option<Vec<Command>> {
    let parsed = ScriptParser::parse(parser::Rule::script, cmd);
    match parsed {
        Ok(mut pairs) => {
            commander.commands = parse_command(pairs.next().unwrap().into_inner());
            print!("{:?}", commander);
            None
        }
        Err(e) => {
            eprintln!("❌ Syntax Error:\n{}", e);
            None
        }
    }
}

fn parse_command(pairs: Pairs<'_, Rule>) -> Vec<Command> {
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
                Rule::CmdModuleCreation => {
                    let cmd: Vec<&str> = pair.as_str().split('\n').collect();
                    let name = cmd[0][7..].trim();
                    let blocks = inner_cmd.into_inner();
                    let cmd = parse_command(blocks);
                    cmds.push(Command::Module(name.to_string(), cmd));
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
