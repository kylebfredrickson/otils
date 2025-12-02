// use rayon::ThreadPool;

mod or_shuffle;

pub fn shuffle<T: Send>(data: &mut [T]) {
    or_shuffle::or_shuffle(data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle() {
        // let threads = 1;
        // let pool = rayon::ThreadPoolBuilder::new()
        //     .num_threads(threads)
        //     .build()
        //     .unwrap();

        let mut data: Vec<i64> = (0..10).into_iter().collect();
        shuffle(&mut data);
        println!("{:?}", data);

        assert!(true);

        //shuffle
        //sort
        //assert_eq
    }
}
