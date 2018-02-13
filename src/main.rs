extern crate hopper;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate tempdir;

use bincode::{deserialize_from, Bounded};
use self::hopper::channel_with_explicit_capacity;
use std::{io, process};
use rand::{Rng, SeedableRng, XorShiftRng};

#[derive(Debug, PartialEq, Deserialize)]
struct Input { // 22 bytes
    seed_a: u32, // 4
    seed_b: u32, // 4
    seed_c: u32, // 4
    seed_d: u32, // 4
    max_in_memory_bytes: u32, // 4
    max_disk_bytes: u16, // 2
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    if let Ok(input) = deserialize_from(&mut stdin, Bounded(22)) {
        let mut input: Input = input;
        let mut rng: XorShiftRng =
            SeedableRng::from_seed([input.seed_a, input.seed_b, input.seed_c, input.seed_d]);

        input.max_in_memory_bytes = if input.max_in_memory_bytes > 2 << 25 {
            2 << 25
        } else {
            input.max_in_memory_bytes
        };

        // We need to be absolutely sure we don't run into another running
        // process. Which, this isn't a guarantee but it's _pretty_ unlikely to hit
        // jackpot.
        let prefix = format!(
            "hopper-{}-{}-{}",
            rng.gen::<u64>(),
            rng.gen::<u64>(),
            rng.gen::<u64>()
        );

        if let Ok(dir) = tempdir::TempDir::new(&prefix) {
            if let Ok((mut snd, mut rcv)) = channel_with_explicit_capacity(
                "afl",
                dir.path(),
                input.max_in_memory_bytes as usize,
                input.max_disk_bytes as usize,
            ) {
                let mut writes = 0;
                let mut rcv_iter = rcv.iter();
                loop {
                    match rng.gen_range(0, 102) {
                        0...50 => {
                            if writes != 0 {
                                let _ = rcv_iter.next();
                            }
                        }
                        50...100 => {
                            snd.send(rng.gen::<u64>());
                            writes += 1;
                        }
                        _ => {
                            process::exit(0);
                        }
                    }
                }
            }
        }
    }
}
