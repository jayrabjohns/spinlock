use std::thread;

use spinlock::{spin_guard::SpinGuard, spin_lock::SpinLock};

fn main() {
    for _ in 0..1000 {
        threads_should_have_exclusive_access_to_value_when_locked()
    }
}

fn threads_should_have_exclusive_access_to_value_when_locked() {
    let vec_lock: SpinLock<Vec<i32>> = SpinLock::new(Vec::new());

    thread::scope(|scope| {
        scope.spawn(|| {
            let mut vec: SpinGuard<'_, Vec<i32>> = vec_lock.lock();
            vec.push(1)
        });

        scope.spawn(|| {
            let mut vec: SpinGuard<'_, Vec<i32>> = vec_lock.lock();
            vec.push(2);
            vec.push(2);
        });
    });

    let vec = vec_lock.lock();

    let slice: &[i32] = &vec[..];

    assert!(slice == [1, 2, 2] || slice == [2, 2, 1]);
}
