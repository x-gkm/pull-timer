#![doc = include_str!("../README.md")]

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct PullTimer<T>(VecDeque<(u32, T)>);

impl<T> PullTimer<T> {
    pub fn new() -> PullTimer<T> {
        PullTimer(VecDeque::new())
    }

    pub fn next_in(&self) -> Option<u32> {
        self.0.front().map(|&(deadline, _)| deadline)
    }

    pub fn tick(&mut self, elapsed: u32) {
        let mut remaining = elapsed;
        for (delta, _) in &mut self.0 {
            let temp = *delta;
            *delta = delta.saturating_sub(elapsed);
            remaining = remaining.saturating_sub(temp);

            if remaining == 0 {
                break;
            }
        }
    }

    pub fn add(&mut self, deadline: u32, event: T) {
        let mut sum = 0;
        let mut insertion_point = 0;

        for (index, &(delta, _)) in self.0.iter().enumerate() {
            if sum + delta > deadline {
                break;
            }
            insertion_point = index + 1;
            sum += delta;
        }

        let insertion_delta = deadline - sum;

        if let Some((delta, _)) = &mut self.0.get_mut(insertion_point) {
            *delta = delta.saturating_sub(insertion_delta);
        }

        self.0.insert(insertion_point, (insertion_delta, event));
    }

    pub fn remove(&mut self, event: T) -> Option<u32>
    where
        T: PartialEq,
    {
        let mut sum = 0;
        let mut target = None;

        for (index, &(delta, ref element)) in self.0.iter().enumerate() {
            sum += delta;
            if *element == event {
                target = Some(index);
                break;
            }
        }

        let index = target?;
        let (delta, _) = self.0.remove(index)?;

        if let Some((next_delta, _)) = self.0.get_mut(index) {
            *next_delta += delta;
        }

        Some(sum)
    }
}

impl<T> Iterator for PullTimer<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let &(delta, _) = self.0.front()?;

        if delta == 0 {
            self.0.pop_front().map(|(_, event)| event)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_preserves_fifo_order() {
        let mut timer = PullTimer::new();

        timer.add(0, "testing");
        timer.add(0, "one two three");

        assert_eq!(timer.next(), Some("testing"));
        assert_eq!(timer.next(), Some("one two three"));
        assert_eq!(timer.next(), None);
    }

    #[test]
    fn timer_fires_in_order() {
        let mut timer = PullTimer::new();

        timer.add(4, "test");
        timer.add(3, "a");
        timer.add(2, "is");
        timer.add(1, "this");

        timer.tick(4);

        assert_eq!(timer.next(), Some("this"));
        assert_eq!(timer.next(), Some("is"));
        assert_eq!(timer.next(), Some("a"));
        assert_eq!(timer.next(), Some("test"));
        assert_eq!(timer.next(), None);
    }

    #[test]
    fn timer_fires_in_time() {
        let mut timer = PullTimer::new();

        timer.add(40, 40);
        timer.add(20, 20);
        timer.add(0, 0);
        timer.add(30, 30);
        timer.add(10, 10);

        for i in 0..=41 {
            if let Some(value) = timer.next() {
                assert_eq!(value, i);
            }
            timer.tick(1);
        }
    }

    #[test]
    fn timer_next_in() {
        let mut timer = PullTimer::new();

        timer.add(0, "hi");
        timer.add(20, "!");
        timer.add(10, "there");

        assert_eq!(timer.next_in(), Some(0));
        assert_eq!(timer.next(), Some("hi"));

        assert_eq!(timer.next_in(), Some(10));

        timer.tick(10);
        assert_eq!(timer.next_in(), Some(0));
        assert_eq!(timer.next(), Some("there"));

        timer.tick(3);
        assert_eq!(timer.next_in(), Some(7));
    }

    #[test]
    fn timer_remove() {
        let mut timer = PullTimer::new();

        timer.add(100, "boom!");
        timer.tick(50);
        assert_eq!(timer.remove("boom!"), Some(50));
        assert_eq!(timer.next_in(), None);
    }

    #[test]
    fn timer_fires_after_remove() {
        let mut timer = PullTimer::new();

        timer.add(30, 30);
        timer.add(20, 20);
        timer.add(40, 40);
        timer.add(10, 10);
        timer.add(50, 50);

        assert_eq!(timer.remove(50), Some(50));
        assert_eq!(timer.remove(10), Some(10));

        for i in 0..=41 {
            if let Some(value) = timer.next() {
                assert_eq!(value, i);
            }
            timer.tick(1);
        }
    }
}
