extern crate serde;
extern crate serde_json;

use std::env;
use std::fs;
use std::io::{self, Read};
use serde::{
	ser::{SerializeStruct},
};



struct Process {
  pid: String,
  cmdline: Option<String>,
  uid: Option<Vec<String>>,
  gid: Option<Vec<String>>,
  children: Vec<Process>,
}

// trait to serialize process to json_output
impl serde::Serialize for Process {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut state = serializer.serialize_struct("Process", 5)?;
    state.serialize_field("pid", &self.pid)?;
    state.serialize_field("cmdline", &self.cmdline)?;
    state.serialize_field("uid", &self.uid)?;
    state.serialize_field("gid", &self.gid)?;
    state.serialize_field("children", &self.children)?;
    state.end()
  }
}
fn read_cmdline(pid: &str) -> io::Result<Option<String>> {
  let path = format!("/proc/{}/cmdline", pid);
  let mut cmdline = String::new();
  fs::File::open(&path)?.read_to_string(&mut cmdline)?;
  // Replace null characters with spaces and trim
  Ok(Some(cmdline.replace("\u{0}", " ").trim().to_string()))
}

fn read_status(pid: &str) -> io::Result<(Option<Vec<String>>, Option<Vec<String>>)> {
  let path = format!("/proc/{}/status", pid);
  let status = fs::read_to_string(&path)?;
  let mut uids = None;
  let mut gids = None;
  
  for line in status.lines() {
    if line.starts_with("Uid:") {
      uids = Some(line.split_whitespace().skip(1).map(String::from).collect());
    } else if line.starts_with("Gid:") {
      gids = Some(line.split_whitespace().skip(1).map(String::from).collect());
    }
  }
  
  Ok((uids, gids))
}

fn read_children(pid: &str) -> io::Result<Vec<String>> {
  let path = format!("/proc/{}/task/{}/children", pid, pid);
  let children = fs::read_to_string(&path)?;
  Ok(children.split_whitespace().map(String::from).collect())
}

fn pstree(pid: &str) -> Option<Process> {
  let cmdline = read_cmdline(pid).ok()?;
  let (uids, gids) = read_status(pid).ok()?;
  let children = read_children(pid).ok()?;
  
  let mut process = Process {
    pid: pid.to_string(),
    cmdline,
    uid: uids,
    gid: gids,
    children: Vec::new(),
  };
  
  for child_pid in children {
    if let Some(child_process) = pstree(&child_pid) {
      process.children.push(child_process);
    }
  }
  
  Some(process)
}

fn main() {
  let psnum = env::args().nth(1).unwrap_or_else(|| "1".to_string());
  let process_tree = pstree(&psnum);
  match process_tree {
    Some(tree) => {
      let json_output = serde_json::to_string(&tree).unwrap();
      println!("{}", json_output);
    }
    None => {
      eprintln!("Failed to retrieve process tree for PID: {}", psnum);
    }
  }
}
