use std::time::SystemTime;
use humantime::format_rfc3339_nanos;

fn main() {
  println!("{}", format_rfc3339_nanos(SystemTime::now()));
}
