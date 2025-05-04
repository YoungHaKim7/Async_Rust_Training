# Result
- Async/await and multi-thread Tokio runtime
  - https://users.rust-lang.org/t/async-await-and-multi-thread-tokio-runtime/110107

```bash
Handle: Handle { inner: MultiThread(multi_thread::Handle { ... }) }
Main: start 'main' ThreadId(1)
Task 1: start 'tokio-runtime-worker' ThreadId(9)
Task 2: start 'tokio-runtime-worker' ThreadId(9)
Task 2: end 'tokio-runtime-worker' ThreadId(2)
Task 1: sleeped 'tokio-runtime-worker' ThreadId(2)
Task 1: end 'tokio-runtime-worker' ThreadId(2)
Main: sleeped 'main' ThreadId(1)
Main: end 'main' ThreadId(1)
```

