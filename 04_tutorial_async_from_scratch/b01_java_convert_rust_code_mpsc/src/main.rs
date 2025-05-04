use std::sync::{Arc, Mutex};
use std::thread;

struct ResultData {
    accum_value: i32,
}

impl ResultData {
    fn add_value(&mut self, value: i32) {
        self.accum_value += value;
    }
}

fn main() {
    use std::sync::mpsc;

    println!("[작업 처리 요청]");

    // 공유 객체
    let result = Arc::new(Mutex::new(ResultData { accum_value: 0 }));

    // 채널로 완료 기다림
    let (tx, rx) = mpsc::channel();

    // 작업 A: 1~5 합
    {
        let result = Arc::clone(&result);
        let tx = tx.clone();
        thread::spawn(move || {
            let mut sum = 0;
            for i in 1..=5 {
                sum += i;
            }
            let mut res = result.lock().unwrap();
            res.add_value(sum);
            tx.send(()).unwrap();
        });
    }

    // 작업 B: 6~10 합
    {
        let result = Arc::clone(&result);
        let tx = tx.clone();
        thread::spawn(move || {
            let mut sum = 0;
            for i in 6..=10 {
                sum += i;
            }
            let mut res = result.lock().unwrap();
            res.add_value(sum);
            tx.send(()).unwrap();
        });
    }

    // 두 작업 완료 대기
    rx.recv().unwrap();
    rx.recv().unwrap();

    let final_result = result.lock().unwrap();
    println!("[두 작업 처리 결과] {}", final_result.accum_value);
    println!("[작업 처리 완료]");
}
