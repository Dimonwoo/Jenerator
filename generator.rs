use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::thread::JoinHandle;
use num_cpus;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn generate_tokens_multithreaded(num_tokens_to_generate: usize) -> Vec<String> {
    let num_threads: usize = num_cpus::get();
    let tokens: Arc<Mutex<Vec<Vec<u8>>>> = Arc::new(Mutex::new(Vec::with_capacity(num_tokens_to_generate)));

    let mut handles: Vec<JoinHandle<()>> = vec![];

    for _ in 0..num_threads {
        let tokens_clone: Arc<Mutex<Vec<Vec<u8>>>> = Arc::clone(&tokens);

        let handle: JoinHandle<()> = thread::spawn(move || {
            let mut rng: StdRng = StdRng::from_entropy();
            let num_tokens: usize = num_tokens_to_generate / num_threads;

            let mut generated_tokens: Vec<Vec<u8>> = Vec::with_capacity(num_tokens);
            let valid_chars: Vec<u8> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".bytes().collect();

            for _ in 0..num_tokens {
                let token: Vec<u8> = (0..59)
                    .map(|_| valid_chars[rng.gen_range(0..valid_chars.len())])
                    .collect();
                let mut formatted_token: Vec<u8> = Vec::with_capacity(64);
                formatted_token.extend_from_slice(&token[0..23]);
                formatted_token.push(b'.');
                formatted_token.extend_from_slice(&token[24..46]);
                formatted_token.push(b'.');
                formatted_token.extend_from_slice(&token[47..]);
                generated_tokens.push(formatted_token);
            }

            let mut tokens: MutexGuard<Vec<Vec<u8>>> = tokens_clone.lock().unwrap();
            tokens.extend(generated_tokens);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let tokens: MutexGuard<Vec<Vec<u8>>> = tokens.lock().unwrap();
    tokens.iter().map(|token| String::from_utf8_lossy(token).to_string()).collect()
}