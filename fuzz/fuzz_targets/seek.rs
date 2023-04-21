use honggfuzz::fuzz;
use binrw::io::{NoSeek, Read, Seek, SeekFrom, Write};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if data.len() < 32 {
                return;
            }
            
            // Create a seed for rng
            let mut seed = [0u8; 32];
            for (dst, src) in seed.iter_mut().zip(data.iter()) {
                *dst = *src;
            }
            let mut rng = StdRng::from_seed(seed);

            // Split the input data into two random parts
            // let mut rng = rand::thread_rng();
            let split_index = rng.gen_range(0..data.len());
            let (data1, data2) = data.split_at(split_index);

            // Test write functionality
            let mut write_stream = NoSeek::new(Vec::new());
            let _ = write_stream.write(data1);
            let _ = write_stream.write(data2);

            // Test additional read functionality
            let mut read_stream = NoSeek::new(data2);
            let mut read_data_1 = vec![0; data2.len()];
            let _ = read_stream.read_exact(&mut read_data_1);

            // Test with random seek positions
            let mut stream = NoSeek::new(data);
            let seek_position = rng.gen_range(0..data.len() as u64);
            let _ = stream.seek(SeekFrom::Start(seek_position));
            let mut read_data_2 = vec![0; data.len()];
            let _ = stream.read(&mut read_data_2);
        });
    }
}