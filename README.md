simple comparation benchmark between kanal, flume, crossbeam. 

```
compare             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ corssbeam_bench  8.892 ns      │ 44.35 ns      │ 9.767 ns      │ 10.56 ns      │ 1000    │ 1000000
├─ flume_bench      10.14 ns      │ 249.3 ns      │ 40.8 ns       │ 48.91 ns      │ 1000    │ 1000000
╰─ kanal_bench      6.892 ns      │ 20.05 ns      │ 7.308 ns      │ 7.731 ns      │ 1000    │ 1000000

```

