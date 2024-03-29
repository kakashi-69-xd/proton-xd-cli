
#[cfg(test)]
mod test {
  use tokio::test;
  use crate::operation::TEMPLATES;

  use requestty::{
    Question,
    prompt_one
  };

  use crossterm::style::{
    style,
    Color,
    Stylize
  };

  fn rgb((name,r,g,b): (&str,u8,u8,u8))-> String {
    style(name).with(Color::Rgb { r,g,b }).to_string()
  }

  #[test]
  async fn xd() {
    let q=Question::select("Choose").choices(TEMPLATES.map(rgb)).build();

    println!("{:#?}",style(&prompt_one(q).unwrap().as_list_item().unwrap().text).content());
  }

  #[test]
  async fn path() {
    println!("{:?}",std::path::Path::new("./src/ser").file_name().unwrap());
  }


  #[test]
  async fn repo() {
    let _repo=git2::Repository::open("./test/gitrepo").unwrap();

    println!("xd")
  }


  #[test]
  async fn dir_entry_order() {
    for entry in std::fs::read_dir(".").unwrap() {
      println!("{:?}",entry.unwrap().file_name())
    }
  }
}

