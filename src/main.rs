use wry::{
  Application,
  Attributes,
  Result
};

fn main() -> Result<()> {
  let mut application = Application::new()?;

  let attributes = Attributes {
      url: Some("https://tauri.studio".to_string()),
      title: String::from("Hello World!"),
      // Initialization scripts can be used to define javascript functions and variables.
      initialization_scripts: vec![
          String::from("breads = NaN"),
          String::from("menacing = 'ゴ'"),
      ],
      ..Default::default()
  };

  let window = application.add_window(attributes)?;
  window.show().expect("Could not create window");

  application.run();
  Ok(())
}
