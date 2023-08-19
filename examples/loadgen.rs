use std::time::Duration;

use rand::{thread_rng, Rng};
use tokio::time::sleep;

use lodestone::client::Client;

const NUM_CLIENTS: usize = 10;
const NUM_MESSAGES: usize = 10;
const MESSAGE_MIN_SIZE: usize = 100;
const MESSAGE_MAX_SIZE: usize = 500;

#[tokio::main]
async fn main() {
    let mut tasks = vec![];
    for _ in 1..=NUM_CLIENTS {
        tasks.push(tokio::task::spawn(spawn_client()));
        sleep(Duration::from_millis(100)).await;
    }

    for task in tasks {
        let _ = task.await;
    }
}

async fn spawn_client() {
    let mut client = Client::new(generate_random_string(StringSize::Exact(10)));
    client.connect().unwrap();

    for _ in 0..NUM_MESSAGES {
        // client
        //     .send_message(generate_random_string(StringSize::Range(
        //         MESSAGE_MIN_SIZE,
        //         MESSAGE_MAX_SIZE,
        //     )))
        //     .unwrap();

        client
            .send_message(generate_random_string(StringSize::Exact(500)))
            .unwrap();

        sleep(Duration::from_millis(200)).await;
    }

    client.disconnect().unwrap();
}

enum StringSize {
    Exact(usize),
    Range(usize, usize),
}

impl StringSize {
    fn resolve(self) -> usize {
        match self {
            Self::Exact(size) => size,
            Self::Range(min, max) => thread_rng().gen_range(min..=max),
        }
    }
}

fn generate_random_string(size: StringSize) -> String {
    let mut rng = thread_rng();
    let mut random_string = String::new();
    for _ in 0..=size.resolve() {
        random_string.push(rng.gen_range(b'a'..=b'z') as char);
    }

    random_string
}
