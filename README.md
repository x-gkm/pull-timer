# pull-timer
A simple "pull" timer.

## Usage
```rust
use pull_timer::PullTimer;

let mut timer = PullTimer::new();
timer.add(10, "an event!");
assert_eq!(timer.next(), None);

timer.tick(10);
assert_eq!(timer.next(), Some("an event!"));
```