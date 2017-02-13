# cron [![](https://api.travis-ci.org/zslayton/cron.png?branch=master)](https://travis-ci.org/zslayton/cron) [![](http://meritbadge.herokuapp.com/cron)](https://crates.io/crates/cron)
## A Cron Expression Parser in Rust

```rust
extern crate cron;
extern crate chrono;

use cron::Schedule;
use chrono::UTC;
use std::str::FromStr;

fn main() {
  //               sec  min   hour   day of month   month   day of week   year
  let expression = "0   30   9,12,15     1,15       May-Aug  Mon,Wed,Fri  2018/2";
  let schedule = Schedule::from_str(expression).unwrap();
  println!("Upcoming fire times:");
  for datetime in schedule.upcoming(UTC).take(10) {
    println!("-> {}", datetime);
  }
}

/*
Upcoming fire times:
-> 2018-06-01 09:30:00 UTC
-> 2018-06-01 12:30:00 UTC
-> 2018-06-01 15:30:00 UTC
-> 2018-06-15 09:30:00 UTC
-> 2018-06-15 12:30:00 UTC
-> 2018-06-15 15:30:00 UTC
-> 2018-08-01 09:30:00 UTC
-> 2018-08-01 12:30:00 UTC
-> 2018-08-01 15:30:00 UTC
-> 2018-08-15 09:30:00 UTC
*/
```
