use std::env;

pub fn get_env_required<K>(key: K) -> String
where
    K: AsRef<str> + std::fmt::Display,
{
    env::var(key.as_ref()).expect(format!("env variable with name {key} is required").as_str())
}
