use serde::{Serialize,Deserialize};
use fastdate::DateTime;

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SysCaptchaEntity{
  pub id:i64,
  pub code:String,
  pub create_time:DateTime,
  pub expire_time:i128,
}