# async(concurrency , await)

- https://natkr.com/2025-04-10-async-from-scratch-1/


# 내 쓰레드 체크
- https://doc.rust-lang.org/std/thread/fn.available_parallelism.html

# 철도 레일

- concurency(Web에서 많이 쓴다. await)
  - 철도 레일 1개짜리 로한다. concurency
  - await 쓰레드를 깨워도(자는넘을 깨워, 일하라고 )

- parallelism
  - 철도 레일 우리가 생각 병렬실행
  - 레일이 여러개야
  - 제거 기준 인텔 쓰레드20개
    - 철도 레일 이 20개 있다.
