mod ser;
use ser::*;


use tokio::*;
use std::{
  path::PathBuf,
  env
};


#[main]
async fn main() {
  let args=Operation::new();
  
  println!("{:?}",&args);

  match args {
    Operation::Build { dir }=> build(dir).await,
    Operation::Init { path,template,ts,.. }=> init(path,template,ts).await,
    _=> todo!()
  }.unwrap()
}

async fn build(dir: PathBuf)-> io::Result<()> {
  ensure_dir(&dir).await?;

  let mut process=process::Command::new("deno");
  process.arg("compile")
  .args(["--no-prompt","-o",dir.join("xd").to_str().unwrap(),])
  .arg("./proton-xd-src/main.ts");

  std::process::exit(process.spawn()?.wait().await?.code().unwrap_or_default())
}

async fn ensure_dir(path: &PathBuf)-> io::Result<()> {
  if fs::try_exists(path).await? {
    return Ok(());
  }
  fs::create_dir(path).await
}

async fn init(path: Option<PathBuf>,_template: Option<Box<str>>,ts: bool)-> io::Result<()> {
  let path=path.unwrap_or(env::current_dir()?);
  ensure_empty(&path).await?;

  let _url=format!("https://github.com/kakashi-69-xd/proton-xd-templates/{}/{}",lang(ts),"next");




  Ok(())
}

fn lang(ts: bool)-> Box<str> {
  match ts {
    true=> "ts",
    false=> "js",
  }.into()
}

async fn ensure_empty(path: &PathBuf)-> io::Result<()> {
  match fs::read_dir(path).await?.next_entry().await? {
    Some(path)=> panic!("{path:?} is not an empty directory!"),
    None=> Ok(()),
  }
}
