use rocket::fs::TempFile;

#[derive(FromForm)]
pub struct Upload<'r> {
  pub name: String,
  pub file: TempFile<'r>,
}
