use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct Fork {
    id: u32,
    mutex: Mutex<()>,
}

#[derive(Debug)]
struct Philosopher {
    id: u32,
    name: String,
    left_fork: Arc<Fork>,
    right_fork: Arc<Fork>,
}

impl Philosopher {
    fn new(id: u32, name: String, left_fork: Arc<Fork>, right_fork: Arc<Fork>) -> Self {
        Philosopher {
            id,
            name,
            left_fork,
            right_fork,
        }
    }

    fn eat(&self) {
        let (first_fork, second_fork) = if self.id % 2 == 0 {
            (&self.left_fork, &self.right_fork)
        } else {
            (&self.right_fork, &self.left_fork)
        };

        let _first_guard = first_fork.mutex.lock().unwrap();
        println!("{} picked up fork {}", self.name, first_fork.id);

        let _seconde_guard = second_fork.mutex.lock().unwrap();
        println!("{} picked up fork {}", self.name, second_fork.id);

        println!("{} is eating", self.name);

        thread::sleep(Duration::from_secs(10));

        println!("{} finished eating", self.name);

        println!("{} put down fork {}", self.name, first_fork.id);
        println!("{} put down fork {}", self.name, second_fork.id);
    }
}

fn main() {
    println!("Dining Philosophers Problem:  15 Philosophers, 4 Forks...Yikes!!");

    let forks = (0..4).map(|id| {
        Arc::new(Fork {
            id,
            mutex: Mutex::new(()),
        })
    }).collect::<Vec<_>>();

    let philosophers = vec![
        ("Jürgen Habermas", 0, 1),
        ("Friedrich Engels", 1, 2),
        ("Karl Marx", 2, 3),
        ("Thomas Piketty", 3, 0),
        ("Michel Foucault", 0, 1),
        ("Socrates", 1, 2),
        ("Plato", 2, 3),
        ("Aristotle", 3, 0),
        ("Pythagoras", 0, 1),
        ("Heraclitus", 1, 2),
        ("Democritus", 2, 3),
        ("Diogenes", 3, 0),
        ("Epicurus", 0, 1),
        ("Zeno of Citium", 1, 2),
        ("Thales of Miletus", 2, 3),
    ]
    .into_iter()
    .enumerate()
    .map(|(id, (name, left, right))| {
        Philosopher::new(
            id as u32,
            name.to_string(),
            Arc::clone(&forks[left]),
            Arc::clone(&forks[right]),
        )
    })
    .collect::<Vec<_>>();

    println!("{:?}", philosophers);

    let start = Instant::now();
    let handlers = philosophers
        .into_iter()
        .map(|philosopher| thread::spawn(move || philosopher.eat()))
        .collect::<Vec<_>>();

    for handle in handlers {
        handle.join().unwrap();
    }

    println!("Total time: {:?}", start.elapsed());
}
