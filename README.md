# pull-timer
A simple "pull" timer.

## Usage
```rust
# use pull_timer::PullTimer;
#
let mut timer = PullTimer::new();

timer.add(10, "an event!");

assert_eq!(timer.poll(), None);

timer.update(10);

assert_eq!(timer.poll(), Some("an event!"));
```