use honggfuzz::fuzz;
use binrw::io::{BufReader, Read, Seek, SeekFrom};
use std::io::{Cursor};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

// Counter and Read, Seek implementation

struct Counter<T> {
    inner: T,
    reads: usize,
}

impl<T> Counter<T> {
    fn new(inner: T) -> Self {
        Counter { inner, reads: 0 }
    }
}

impl<T: Read> Read for Counter<T> {
    fn read(&mut self, buf: &mut [u8]) -> binrw::io::Result<usize> {
        self.reads += 1;
        self.inner.read(buf)
    }
}

impl<T: Seek> Seek for Counter<T> {
    fn seek(&mut self, pos: SeekFrom) -> binrw::io::Result<u64> {
        self.inner.seek(pos)
    }
}

// Fuzzing

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if data.is_empty() {
                return;
            }

            // Create a seed for rng
            let mut seed = [0u8; 32];
            for (dst, src) in seed.iter_mut().zip(data.iter()) {
                *dst = *src;
            }
            let mut rng = StdRng::from_seed(seed);

            //let mut rng = rand::thread_rng();
            let buffer_size = rng.gen_range(1..data.len() + 1);
            let cursor = Cursor::new(data);
            let mut stream = BufReader::with_capacity(buffer_size, Counter::new(cursor));

			let iterations = rng.gen_range(50..250);
            for _ in 0..iterations {
                let read_size = rng.gen_range(1..data.len() + 1);
                let mut read_data = vec![0; read_size];
                let _ = stream.read(&mut read_data);

                let seek_from_choices = [
                    SeekFrom::Start(rng.gen_range(0..data.len() as u64)),
                    SeekFrom::Current(rng.gen_range(-(data.len() as i64)..data.len() as i64)),
                    SeekFrom::End(rng.gen_range(-(data.len() as i64)..1)),
                ];

                let seek_from = seek_from_choices.choose(&mut rng).unwrap();
                let _ = stream.seek(*seek_from);
            }
        });
    }
}