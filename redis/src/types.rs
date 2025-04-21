use std::fmt;
use chrono::{Utc, Duration, DateTime};
use std::{
    collections::{HashMap,HashSet},
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

pub type Db = Arc<Mutex<HashMap<String, RedisValue>>>;
pub type CACHE = Arc<Mutex<HashMap<String, DateTime<Utc>>>>;

#[derive(Debug)]
pub struct LinkedList {
    pub value: String,
    pub next: Option<Box<LinkedList>>,
}

impl LinkedList {
    pub fn new(value: String) -> Self {
        LinkedList { value, next: None }
    }

    pub fn append(&mut self, value: String) {
        match &mut self.next {
            Some(next_node) => next_node.append(value),
            None => {
                self.next = Some(Box::new(LinkedList::new(value)));
            }
        }
    }
}

#[derive(Debug)]
pub enum ValueType {
    String(String),
    List(Vec<String>),
    Set(HashSet<String>),
    Hash(HashMap<String, String>),
    SortedSet(Vec<(f64, String)>),
    LinkedList(LinkedList),
}

#[derive(Debug)]
pub struct RedisValue {
    pub value: ValueType,
}

impl fmt::Display for RedisValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            ValueType::String(val) => write!(f, "String({})", val),
            ValueType::List(vals) => write!(f, "List({:?})", vals),
            ValueType::Set(vals) => write!(f, "Set({:?})", vals),
            ValueType::Hash(vals) => write!(f, "Hash({:?})", vals),
            ValueType::SortedSet(vals) => write!(f, "SortedSet({:?})", vals),
            ValueType::LinkedList(linked_list) => write!(f, "LinkedList({:?})", linked_list),
        }
    }
}

pub type RedisDb = HashMap<String, RedisValue>;
