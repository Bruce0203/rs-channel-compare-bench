simple comparation benchmark between kanal, flume, crossbeam. 

```
compare             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ corssbeam_bench  9.642 ns      │ 49.01 ns      │ 10.14 ns      │ 10.92 ns      │ 1000    │ 1000000
├─ flume_bench      10.05 ns      │ 206.4 ns      │ 33.66 ns      │ 37.55 ns      │ 1000    │ 1000000
╰─ kanal_bench      6.892 ns      │ 19.97 ns      │ 7.684 ns      │ 7.811 ns      │ 1000    │ 1000000

```

