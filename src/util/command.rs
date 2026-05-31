use crate::util::parser::{Rule, ScriptParser};
use fantoccini::Client;
use pest::iterators::Pairs;
use pest::Parser;
use std::collections::HashMap;

mod dom;
mod flow;
mod nav;

#[derive(Debug, Clone)]
pub struct Commander {
    pub flows: HashMap<String, Vec<Command>>,
    pub commands: Vec<Command>,
    pub client: Client,
}

#[derive(Debug, Clone)]
pub enum Command {
    Goto(String),
    Refresh,
    Back,
    Flow(String, Vec<Command>),
    Click(String),
    TriggerEvent(String, String),
    Wait(u64, String),
    WaitElement(String),
    Fill(String, String),
    Clear(String),
    Hover(String),
    ScrollTo(String),
    CallFlow(String),
    SelectFirst(String),
    Hangup,
    PressKey(String),
    Screenshot(String),
    AssertText(String, String),
}

impl Commander {
    pub fn new(
        client: Client,
        commands: Vec<Command>,
        flows: HashMap<String, Vec<Command>>,
    ) -> Commander {
        Self {
            flows,
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
                println!("🚀 Navigating to: {url}");
                nav::go(url, client).await
            }
            Command::Refresh => {
                println!("🔄 Refreshing page");
                let _ = client.refresh().await;
            }
            Command::Back => {
                println!("⬅️ Going back");
                let _ = client.back().await;
            }
            Command::Flow(name, cmds) => {
                println!("🏗️ Defining flow: {name}");
                self.flows.entry(name).or_insert(cmds);
            }
            Command::CallFlow(name) => {
                if let Some(cmds) = self.flows.get(&name).cloned() {
                    println!("🔄 Calling flow: {name}");
                    let mut runner = Commander::new(client.clone(), cmds, self.flows.clone());
                    runner.exec().await;
                } else {
                    println!("⚠️ Flow not found: {name}");
                }
            }
            Command::Click(sel) => dom::click(client, sel).await,
            Command::Fill(sel, val) => dom::fill(client, sel, val).await,
            Command::Clear(sel) => dom::clear(client, sel).await,
            Command::Hover(sel) => dom::hover(client, sel).await,
            Command::ScrollTo(sel) => dom::scroll_to(client, sel).await,
            Command::Wait(amt, unit) => {
                println!("⏱️ Waiting: {amt} {unit}");
                flow::wait_for(amt, unit).await
            }
            Command::WaitElement(sel) => {
                println!("🔍 Waiting for element: {sel}");
                dom::wait_for_element(client, sel).await;
            }
            Command::TriggerEvent(ev, sel) => dom::trigger_event(client, ev, sel).await,
            Command::SelectFirst(sel) => dom::select_first(client, sel).await,
            Command::PressKey(val) => dom::simulate_keyinput(client, val).await,
            Command::Screenshot(name) => dom::screenshot(client, name).await,
            Command::AssertText(text, sel) => dom::assert_text(client, text, sel).await,
            Command::Hangup => flow::hangup().await,
        };
    }
}

pub fn parse(cmd: &str) -> Option<Vec<Command>> {
    ScriptParser::parse(Rule::script, cmd)
        .map(|mut pairs| parse_command(pairs.next().unwrap().into_inner()))
        .map_err(|e| eprintln!("❌ Syntax Error:\n{}", e))
        .ok()
}

pub fn parse_command(pairs: Pairs<'_, Rule>) -> Vec<Command> {
    pairs
        .filter(|p| p.as_rule() == Rule::command)
        .map(|p| {
            let inner = p.into_inner().next().unwrap();
            let mut parts = inner.clone().into_inner().map(|i| clean(i.as_str()));

            match inner.as_rule() {
                Rule::CmdGoto => Command::Goto(parts.next().unwrap()),
                Rule::CmdRefresh => Command::Refresh,
                Rule::CmdBack => Command::Back,
                Rule::CmdClick => Command::Click(parts.next().unwrap()),
                Rule::CmdFill => Command::Fill(parts.next().unwrap(), parts.next().unwrap()),
                Rule::CmdClear => Command::Clear(parts.next().unwrap()),
                Rule::CmdHover => Command::Hover(parts.next().unwrap()),
                Rule::CmdScrollTo => Command::ScrollTo(parts.next().unwrap()),
                Rule::CmdWait => Command::Wait(parts.next().unwrap().parse().unwrap_or(0), parts.next().unwrap()),
                Rule::CmdWaitElement => Command::WaitElement(parts.next().unwrap()),
                Rule::CmdTriggerEvent => Command::TriggerEvent(parts.next().unwrap(), parts.next().unwrap()),
                Rule::CmdFlowCreation => {
                    let name = inner.as_str()["create flow".len()..].trim().split('\n').next().unwrap().to_string();
                    Command::Flow(name, parse_command(inner.into_inner()))
                }
                Rule::CmdCallFlow => Command::CallFlow(inner.as_str()["using flow".len()..].trim().to_string()),
                Rule::CmdSelectFirst => Command::SelectFirst(parts.next().unwrap()),
                Rule::CmdPressKey => Command::PressKey(parts.next().unwrap()),
                Rule::CmdScreenshot => Command::Screenshot(parts.next().unwrap()),
                Rule::CmdAssertText => Command::AssertText(parts.next().unwrap(), parts.next().unwrap()),
                Rule::CmdHangup => Command::Hangup,
                _ => {
                    println!("Unknown rule: {:?}", inner.as_rule());
                    Command::Hangup
                }
            }
        })
        .collect()
}

fn clean(s: &str) -> String {
    s.trim_matches(|c| c == '\'' || c == '"').to_string()
}
