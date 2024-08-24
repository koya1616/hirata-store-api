use dotenv::dotenv;
use std::env;
use std::sync::OnceLock;

pub struct RuntimeConfig {
  pub supabase_url: String,
  pub supabase_key: String,
}

impl RuntimeConfig {
  pub fn global() -> &'static RuntimeConfig {
    static INSTANCE: OnceLock<RuntimeConfig> = OnceLock::new();
    INSTANCE.get_or_init(|| {
      dotenv().ok();
      RuntimeConfig {
        supabase_url: env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        supabase_key: env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
      }
    })
  }
}
