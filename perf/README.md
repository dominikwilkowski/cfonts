# Performance

This package measures cfonts v4, this repository, on speed, allocations and memory.
It can also compare v4 with cfonts v3 (1.3.0-rust I call v3 in this doc as node and rust had different versions), as a side feature.

## What it measures

Every scenario runs on two paths:

- **API**, one render in process through the library
- **CLI**, one run of the shipped binary from spawn to exit, startup included

Three tests measure each scenario on both paths:

- **speed**, the median time per render, timed after a short warm-up in samples until a time budget is spent
  and at least 5 samples are taken, the budget grows with the time the render took in the fairness check
- **allocations**, the heap blocks and bytes of one render, counted by dhat on the API path
  and by the interposing library in `interpose/` for the whole process on the CLI path
- **memory**, the peak heap of one render, counted by dhat in a fresh child process on the API path,
  and on the CLI path the peak heap of the whole process from the interposing library
  and the peak resident set size the operating system reports for a second run with nothing injected

`src/lib.rs` lists every scenario and how it runs on each path.

## Fixed conditions

Every render wraps at 80 columns and paints in true color.
The API path passes both to v4's `render_with` as overrides and rolls candy colors from a fixed seed.
The CLI path runs the binary with stdin on `/dev/null`, its output piped, `FORCE_COLOR=3` and, for v4, `FORCE_SIZE=80`.
Every child starts with no other variable,
the shell's `PATH`, `HOME` and `TMPDIR` would move the bytes a process allocates at startup.
v3 has no width variable and falls back to 80 columns without a terminal,
so the API path renders in child processes of the runner with no terminal attached.

## Make targets

Run them from the repository root.

| target            | what it does                                                                                              | when to use it                                    |
| ----------------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| `make perf`       | measures v4 and prints every number that changed against the results below beyond its noise               | as often as you like while you change v4          |
| `make perf-save`  | measures v4 and writes the results below                                                                  | once you are happy with the change                |
| `make compare`    | installs the v3 binary into `perf/target/v3` once, measures both versions and writes the comparison below | to see v4 next to v3                              |
| `make flamegraph` | writes a flamegraph and a heap profile of v4 per scenario into `perf/target/flamegraphs`                  | to find where a render spends its time and memory |

The results below are the baseline.
Change v4 and run `make perf` as often as you like, it reads the results and never writes this file.
Once you are happy, run `make perf-save` and commit this file with the change.

`make perf` reports every change of an allocation count, a byte count or a peak heap,
a median time beyond 10% on the API path and 60% on the CLI path, where process start follows the load of the machine,
and a peak resident set size beyond 4%, clear of the most unchanged code moves them between runs,
and prints how many scenarios it checked.
`CFONTS_PERF_SCENARIOS=short-plain,long-gradient make perf` checks only the scenarios it names,
`make perf-save` and `make compare` then write tables of only those.

`make flamegraph` renders each scenario on the API path for three seconds under a sampling profiler
and once more under dhat, and writes `<scenario>.svg` and `<scenario>.dhat.json`.
Open the graph in a browser and the heap profile in [dhat's viewer](https://nnethercote.github.io/dh_view/dh_view.html).
The CLI path has no graph, the profiler samples the runner itself and the CLI runs the same render behind process start.
## The fairness check

The runner checks every scenario once before it measures anything.
It renders a probe and each scenario once per path and measured version in a child process with a 30 second limit.

- A render passes when every row fits 80 columns and the output carries every escape code family the scenario paints with.
- The probe, 200 console font characters in a gradient, has to wrap at exactly 80 columns in true color,
  otherwise every scenario of that path and version counts as unfair.
- A version that crashes, times out or breaks a condition is never measured for that scenario, its table cells say why.
- With v3 in the run, the check also compares how many rows carry glyphs in each version.

So every number below comes from a render at 80 columns in true color that ran to its end.

## The numbers depend on the machine

Times and resident set sizes change with the CPU, the operating system, the load and the temperature,
so only numbers from the same machine compare.
Allocation counts and peak heaps are exact for one build on one system, they change with the code, the compiler and the allocator.
Each generated block below starts with the date, the system, the CPU and the rustc it ran with.

## Scenarios

<!-- perf:scenarios:start -->
Ran 2026-10-25 on macos aarch64, Apple M1 Max, rustc 1.99.1 (58cf8f9c3 2026-10-13)

| scenario                      | why                                                                                                 |
|-------------------------------|-----------------------------------------------------------------------------------------------------|
| `short-plain`                 | glyph lookup, wrapping and layout without any paint                                                 |
| `short-colors`                | the two color slots of the block font in two named colors                                           |
| `short-gradient`              | a two stop gradient, one true color code per column                                                 |
| `short-transition`            | a three stop transition, one true color code per column                                             |
| `short-background`            | a true color background behind every row                                                            |
| `short-colors-background`     | named colors on a true color background                                                             |
| `short-gradient-background`   | a two stop gradient on a true color background                                                      |
| `short-transition-background` | a three stop transition on a true color background                                                  |
| `long-plain`                  | glyph lookup, wrapping and layout without any paint                                                 |
| `long-colors`                 | the two color slots of the block font in two named colors                                           |
| `long-gradient`               | a two stop gradient, one true color code per column                                                 |
| `long-transition`             | a three stop transition, one true color code per column                                             |
| `long-background`             | a true color background behind every row                                                            |
| `long-colors-background`      | named colors on a true color background                                                             |
| `long-gradient-background`    | a two stop gradient on a true color background                                                      |
| `long-transition-background`  | a three stop transition on a true color background                                                  |
| `startup`                     | process start and exit without a render, the floor under every CLI number                           |
| `single-character`            | the fixed cost of one render of one glyph                                                           |
| `console-font`                | the cheapest font, the overhead floor of a render                                                   |
| `line-breaks`                 | three lines through the pipe character                                                              |
| `long-center`                 | centering every wrapped line                                                                        |
| `long-right`                  | right aligning every wrapped line                                                                   |
| `long-spacing`                | letter spacing 3 and line height 2, wider gaps and more rows                                        |
| `long-independent-gradient`   | a gradient that restarts on every line                                                              |
| `long-candy`                  | random named colors, v4's API rolls from a fixed seed, v3 and the v4 binary roll new ones every run |
| `scaling-125`                 | the start of the prose, how the cost grows with the text                                            |
| `scaling-250`                 | the start of the prose, how the cost grows with the text                                            |
| `scaling-500`                 | the start of the prose, how the cost grows with the text                                            |
| `scaling-1000`                | the start of the prose, how the cost grows with the text                                            |
| `alphabet-console`            | every letter in a font both versions ship                                                           |
| `alphabet-block`              | every letter in a font both versions ship                                                           |
| `alphabet-simpleblock`        | every letter in a font both versions ship                                                           |
| `alphabet-simple`             | every letter in a font both versions ship                                                           |
| `alphabet-3d`                 | every letter in a font both versions ship                                                           |
| `alphabet-chrome`             | every letter in a font both versions ship                                                           |
| `alphabet-huge`               | every letter in a font both versions ship                                                           |
| `alphabet-shade`              | every letter in a font both versions ship                                                           |
| `alphabet-slick`              | every letter in a font both versions ship                                                           |
| `alphabet-grid`               | every letter in a font both versions ship                                                           |
| `alphabet-pallet`             | every letter in a font both versions ship                                                           |
| `alphabet-tiny`               | every letter in a font both versions ship                                                           |

<!-- perf:scenarios:end -->

## Results

`make perf-save` writes this section, v4 alone, `make perf` compares with it.

### Speed, median time per render

<!-- perf:results:speed:start -->
Ran 2026-10-25 on macos aarch64, Apple M1 Max, rustc 1.99.1 (58cf8f9c3 2026-10-13)

| scenario                      | v4 api median | v4 cli median |
|-------------------------------|---------------|---------------|
| `short-plain`                 | 4.43 µs       | 1.63 ms       |
| `short-colors`                | 5.72 µs       | 1.64 ms       |
| `short-gradient`              | 11.2 µs       | 1.66 ms       |
| `short-transition`            | 10.5 µs       | 1.65 ms       |
| `short-background`            | 4.56 µs       | 1.63 ms       |
| `short-colors-background`     | 5.90 µs       | 1.65 ms       |
| `short-gradient-background`   | 11.4 µs       | 1.64 ms       |
| `short-transition-background` | 10.6 µs       | 1.67 ms       |
| `long-plain`                  | 188 µs        | 1.91 ms       |
| `long-colors`                 | 235 µs        | 1.99 ms       |
| `long-gradient`               | 428 µs        | 2.36 ms       |
| `long-transition`             | 424 µs        | 2.34 ms       |
| `long-background`             | 193 µs        | 1.95 ms       |
| `long-colors-background`      | 239 µs        | 2.04 ms       |
| `long-gradient-background`    | 432 µs        | 2.37 ms       |
| `long-transition-background`  | 427 µs        | 2.35 ms       |
| `startup`                     | n/a           | 1.61 ms       |
| `single-character`            | 826 ns        | 1.64 ms       |
| `console-font`                | 590 ns        | 1.63 ms       |
| `line-breaks`                 | 6.04 µs       | 1.63 ms       |
| `long-center`                 | 189 µs        | 1.95 ms       |
| `long-right`                  | 190 µs        | 1.93 ms       |
| `long-spacing`                | 252 µs        | 2.01 ms       |
| `long-independent-gradient`   | 478 µs        | 2.41 ms       |
| `long-candy`                  | 240 µs        | 1.99 ms       |
| `scaling-125`                 | 43.4 µs       | 1.71 ms       |
| `scaling-250`                 | 85.9 µs       | 1.82 ms       |
| `scaling-500`                 | 170 µs        | 1.89 ms       |
| `scaling-1000`                | 340 µs        | 2.14 ms       |
| `alphabet-console`            | 1.16 µs       | 1.62 ms       |
| `alphabet-block`              | 10.1 µs       | 1.64 ms       |
| `alphabet-simpleblock`        | 6.24 µs       | 1.64 ms       |
| `alphabet-simple`             | 5.17 µs       | 1.63 ms       |
| `alphabet-3d`                 | 16.5 µs       | 1.67 ms       |
| `alphabet-chrome`             | 3.91 µs       | 1.67 ms       |
| `alphabet-huge`               | 19.9 µs       | 2.03 ms       |
| `alphabet-shade`              | 10.1 µs       | 1.64 ms       |
| `alphabet-slick`              | 8.55 µs       | 1.86 ms       |
| `alphabet-grid`               | 7.45 µs       | 1.66 ms       |
| `alphabet-pallet`             | 8.51 µs       | 1.65 ms       |
| `alphabet-tiny`               | 2.83 µs       | 1.64 ms       |

<!-- perf:results:speed:end -->

### Allocations, one render

<!-- perf:results:allocations:start -->
Ran 2026-10-25 on macos aarch64, Apple M1 Max, rustc 1.99.1 (58cf8f9c3 2026-10-13)

| scenario                      | v4 api allocations | v4 api bytes | v4 cli allocations | v4 cli bytes |
|-------------------------------|--------------------|--------------|--------------------|--------------|
| `short-plain`                 | 43                 | 9,872 B      | 241                | 29,968 B     |
| `short-colors`                | 45                 | 14,064 B     | 252                | 34,352 B     |
| `short-gradient`              | 55                 | 39,371 B     | 261                | 59,643 B     |
| `short-transition`            | 55                 | 39,374 B     | 265                | 59,700 B     |
| `short-background`            | 43                 | 9,883 B      | 248                | 30,103 B     |
| `short-colors-background`     | 45                 | 14,075 B     | 259                | 34,511 B     |
| `short-gradient-background`   | 55                 | 39,382 B     | 268                | 59,802 B     |
| `short-transition-background` | 55                 | 39,385 B     | 272                | 59,859 B     |
| `long-plain`                  | 794                | 305,772 B    | 993                | 326,962 B    |
| `long-colors`                 | 796                | 551,623 B    | 1,004              | 573,005 B    |
| `long-gradient`               | 807                | 2,239,915 B  | 1,013              | 2,261,265 B  |
| `long-transition`             | 807                | 2,239,918 B  | 1,017              | 2,261,322 B  |
| `long-background`             | 795                | 404,091 B    | 1,000              | 425,389 B    |
| `long-colors-background`      | 797                | 666,331 B    | 1,011              | 687,845 B    |
| `long-gradient-background`    | 807                | 2,239,926 B  | 1,020              | 2,261,424 B  |
| `long-transition-background`  | 807                | 2,239,929 B  | 1,024              | 2,261,481 B  |
| `startup`                     | n/a                | n/a          | 190                | 19,937 B     |
| `single-character`            | 22                 | 1,390 B      | 221                | 21,482 B     |
| `console-font`                | 11                 | 1,828 B      | 209                | 21,928 B     |
| `line-breaks`                 | 56                 | 10,992 B     | 254                | 31,100 B     |
| `long-center`                 | 794                | 305,772 B    | 996                | 327,048 B    |
| `long-right`                  | 794                | 305,772 B    | 996                | 327,046 B    |
| `long-spacing`                | 964                | 368,796 B    | 1,167              | 390,160 B    |
| `long-independent-gradient`   | 807                | 2,239,915 B  | 1,014              | 2,261,315 B  |
| `long-candy`                  | 797                | 552,151 B    | 1,005              | 573,542 B    |
| `scaling-125`                 | 192                | 76,412 B     | 391                | 96,752 B     |
| `scaling-250`                 | 376                | 150,060 B    | 575                | 170,650 B    |
| `scaling-500`                 | 716                | 297,612 B    | 915                | 318,702 B    |
| `scaling-1000`                | 1,407              | 593,020 B    | 1,606              | 615,110 B    |
| `alphabet-console`            | 13                 | 3,320 B      | 211                | 23,450 B     |
| `alphabet-block`              | 72                 | 22,462 B     | 271                | 42,604 B     |
| `alphabet-simpleblock`        | 62                 | 14,404 B     | 260                | 34,542 B     |
| `alphabet-simple`             | 44                 | 9,544 B      | 242                | 29,672 B     |
| `alphabet-3d`                 | 172                | 43,252 B     | 370                | 63,372 B     |
| `alphabet-chrome`             | 31                 | 10,092 B     | 229                | 30,220 B     |
| `alphabet-huge`               | 134                | 39,431 B     | 333                | 59,571 B     |
| `alphabet-shade`              | 52                 | 17,848 B     | 250                | 37,974 B     |
| `alphabet-slick`              | 58                 | 18,463 B     | 257                | 38,605 B     |
| `alphabet-grid`               | 45                 | 18,688 B     | 243                | 38,812 B     |
| `alphabet-pallet`             | 58                 | 18,463 B     | 257                | 38,607 B     |
| `alphabet-tiny`               | 26                 | 8,400 B      | 224                | 28,524 B     |

<!-- perf:results:allocations:end -->

### Memory, peak per render

<!-- perf:results:memory:start -->
Ran 2026-10-25 on macos aarch64, Apple M1 Max, rustc 1.99.1 (58cf8f9c3 2026-10-13)

| scenario                      | v4 api peak heap | v4 cli peak heap | v4 cli peak RSS |
|-------------------------------|------------------|------------------|-----------------|
| `short-plain`                 | 4,760 B          | 24,096 B         | 1,687,552 B     |
| `short-colors`                | 6,904 B          | 26,320 B         | 1,687,552 B     |
| `short-gradient`              | 19,551 B         | 38,976 B         | 1,736,704 B     |
| `short-transition`            | 19,554 B         | 38,992 B         | 1,720,320 B     |
| `short-background`            | 4,779 B          | 24,192 B         | 1,687,552 B     |
| `short-colors-background`     | 6,923 B          | 26,448 B         | 1,687,552 B     |
| `short-gradient-background`   | 19,570 B         | 39,104 B         | 1,720,320 B     |
| `short-transition-background` | 19,573 B         | 39,120 B         | 1,720,320 B     |
| `long-plain`                  | 191,352 B        | 217,568 B        | 2,048,000 B     |
| `long-colors`                 | 314,328 B        | 348,816 B        | 2,113,536 B     |
| `long-gradient`               | 1,158,463 B      | 1,184,768 B      | 2,637,824 B     |
| `long-transition`             | 1,158,466 B      | 1,184,784 B      | 2,621,440 B     |
| `long-background`             | 240,523 B        | 266,816 B        | 2,064,384 B     |
| `long-colors-background`      | 371,691 B        | 398,096 B        | 2,129,920 B     |
| `long-gradient-background`    | 1,158,482 B      | 1,184,896 B      | 2,621,440 B     |
| `long-transition-background`  | 1,158,485 B      | 1,184,912 B      | 2,621,440 B     |
| `startup`                     | n/a              | 19,120 B         | 1,572,864 B     |
| `single-character`            | 1,080 B          | 20,352 B         | 1,687,552 B     |
| `console-font`                | 1,292 B          | 20,496 B         | 1,687,552 B     |
| `line-breaks`                 | 6,264 B          | 25,888 B         | 1,687,552 B     |
| `long-center`                 | 191,352 B        | 217,632 B        | 2,031,616 B     |
| `long-right`                  | 191,352 B        | 217,632 B        | 2,031,616 B     |
| `long-spacing`                | 222,888 B        | 257,184 B        | 2,113,536 B     |
| `long-independent-gradient`   | 1,158,463 B      | 1,184,816 B      | 2,637,824 B     |
| `long-candy`                  | 314,856 B        | 349,456 B        | 2,113,536 B     |
| `scaling-125`                 | 46,344 B         | 67,552 B         | 1,802,240 B     |
| `scaling-250`                 | 91,960 B         | 122,848 B        | 1,884,160 B     |
| `scaling-500`                 | 183,576 B        | 209,248 B        | 2,015,232 B     |
| `scaling-1000`                | 367,048 B        | 406,912 B        | 2,228,224 B     |
| `alphabet-console`            | 2,112 B          | 21,392 B         | 1,687,552 B     |
| `alphabet-block`              | 12,168 B         | 32,288 B         | 1,736,704 B     |
| `alphabet-simpleblock`        | 8,396 B          | 28,064 B         | 1,687,552 B     |
| `alphabet-simple`             | 5,392 B          | 24,960 B         | 1,703,936 B     |
| `alphabet-3d`                 | 24,828 B         | 44,896 B         | 1,736,704 B     |
| `alphabet-chrome`             | 5,300 B          | 24,640 B         | 1,703,936 B     |
| `alphabet-huge`               | 23,268 B         | 44,272 B         | 1,736,704 B     |
| `alphabet-shade`              | 10,560 B         | 30,016 B         | 1,720,320 B     |
| `alphabet-slick`              | 10,152 B         | 30,592 B         | 1,736,704 B     |
| `alphabet-grid`               | 8,968 B          | 28,672 B         | 1,671,168 B     |
| `alphabet-pallet`             | 10,152 B         | 30,592 B         | 1,753,088 B     |
| `alphabet-tiny`               | 4,632 B          | 23,936 B         | 1,671,168 B     |

<!-- perf:results:memory:end -->

## Compared with v3

`make compare` writes this section.
A ratio is v4 over v3, `0.25x` means v4 needs a quarter of what v3 needs.

### Speed, median time per render

<!-- perf:compare:speed:start -->
Ran 2026-10-25 on macos aarch64, Apple M1 Max, rustc 1.99.1 (58cf8f9c3 2026-10-13)

| scenario                      | v3 api median | v4 api median | api ratio | v3 cli median | v4 cli median | cli ratio |
|-------------------------------|---------------|---------------|-----------|---------------|---------------|-----------|
| `short-plain`                 | 686 µs        | 4.39 µs       | 0.00640x  | 2.51 ms       | 1.64 ms       | 0.652x    |
| `short-colors`                | 923 µs        | 5.73 µs       | 0.00621x  | 2.80 ms       | 1.70 ms       | 0.607x    |
| `short-gradient`              | 1.58 ms       | 11.2 µs       | 0.00707x  | 3.59 ms       | 1.65 ms       | 0.458x    |
| `short-transition`            | 1.32 ms       | 10.4 µs       | 0.00788x  | 3.32 ms       | 1.65 ms       | 0.498x    |
| `short-background`            | 685 µs        | 4.58 µs       | 0.00668x  | 2.52 ms       | 1.62 ms       | 0.642x    |
| `short-colors-background`     | 926 µs        | 5.90 µs       | 0.00637x  | 2.82 ms       | 1.63 ms       | 0.579x    |
| `short-gradient-background`   | 1.58 ms       | 11.4 µs       | 0.00718x  | 3.60 ms       | 1.65 ms       | 0.459x    |
| `short-transition-background` | 1.32 ms       | 10.6 µs       | 0.00800x  | 3.32 ms       | 1.64 ms       | 0.495x    |
| `long-plain`                  | 862 ms        | 188 µs        | 0.000218x | 869 ms        | 1.93 ms       | 0.00223x  |
| `long-colors`                 | 1.23 s        | 235 µs        | 0.000191x | 1.25 s        | 2.00 ms       | 0.00160x  |
| `long-gradient`               | 898 ms        | 427 µs        | 0.000476x | 910 ms        | 3.03 ms       | 0.00333x  |
| `long-transition`             | 912 ms        | 424 µs        | 0.000465x | 895 ms        | 2.39 ms       | 0.00267x  |
| `long-background`             | 863 ms        | 192 µs        | 0.000223x | 867 ms        | 1.95 ms       | 0.00225x  |
| `long-colors-background`      | 1.23 s        | 239 µs        | 0.000194x | 1.24 s        | 2.01 ms       | 0.00161x  |
| `long-gradient-background`    | 898 ms        | 432 µs        | 0.000482x | 902 ms        | 2.36 ms       | 0.00262x  |
| `long-transition-background`  | 891 ms        | 427 µs        | 0.000479x | 897 ms        | 2.34 ms       | 0.00261x  |
| `startup`                     | n/a           | n/a           | n/a       | 1.68 ms       | 1.61 ms       | 0.960x    |
| `single-character`            | 66.2 µs       | 824 ns        | 0.0125x   | 1.79 ms       | 1.63 ms       | 0.913x    |
| `console-font`                | 23.3 µs       | 592 ns        | 0.0254x   | 1.71 ms       | 1.62 ms       | 0.950x    |
| `line-breaks`                 | 1.21 ms       | 6.06 µs       | 0.00503x  | 3.06 ms       | 1.62 ms       | 0.530x    |
| `long-center`                 | 864 ms        | 189 µs        | 0.000219x | 868 ms        | 1.92 ms       | 0.00221x  |
| `long-right`                  | 864 ms        | 190 µs        | 0.000219x | 869 ms        | 1.96 ms       | 0.00226x  |
| `long-spacing`                | 911 ms        | 252 µs        | 0.000277x | 914 ms        | 2.02 ms       | 0.00221x  |
| `long-independent-gradient`   | 896 ms        | 476 µs        | 0.000532x | 907 ms        | 2.42 ms       | 0.00267x  |
| `long-candy`                  | 1.23 s        | 240 µs        | 0.000195x | 1.24 s        | 1.99 ms       | 0.00160x  |
| `scaling-125`                 | 48.2 ms       | 43.3 µs       | 0.000899x | 50.4 ms       | 1.69 ms       | 0.0335x   |
| `scaling-250`                 | 184 ms        | 85.9 µs       | 0.000467x | 187 ms        | 1.81 ms       | 0.00973x  |
| `scaling-500`                 | 716 ms        | 169 µs        | 0.000237x | 720 ms        | 1.88 ms       | 0.00261x  |
| `scaling-1000`                | 2.83 s        | 340 µs        | 0.000120x | 2.84 s        | 2.12 ms       | 0.000749x |
| `alphabet-console`            | 39.3 µs       | 1.17 µs       | 0.0297x   | 1.76 ms       | 1.63 ms       | 0.926x    |
| `alphabet-block`              | 3.19 ms       | 10.1 µs       | 0.00316x  | 5.11 ms       | 1.65 ms       | 0.324x    |
| `alphabet-simpleblock`        | 285 µs        | 6.24 µs       | 0.0219x   | 2.09 ms       | 1.65 ms       | 0.790x    |
| `alphabet-simple`             | 176 µs        | 5.16 µs       | 0.0294x   | 2.38 ms       | 1.87 ms       | 0.787x    |
| `alphabet-3d`                 | 2.37 ms       | 16.6 µs       | 0.00698x  | 4.45 ms       | 1.67 ms       | 0.376x    |
| `alphabet-chrome`             | 629 µs        | 3.89 µs       | 0.00618x  | 2.46 ms       | 1.66 ms       | 0.676x    |
| `alphabet-huge`               | 7.17 ms       | 19.8 µs       | 0.00276x  | 9.22 ms       | 1.67 ms       | 0.181x    |
| `alphabet-shade`              | 2.35 ms       | 10.1 µs       | 0.00431x  | 4.26 ms       | 1.64 ms       | 0.384x    |
| `alphabet-slick`              | 2.62 ms       | 8.53 µs       | 0.00326x  | 4.51 ms       | 1.64 ms       | 0.364x    |
| `alphabet-grid`               | 1.93 ms       | 7.15 µs       | 0.00370x  | 3.78 ms       | 1.62 ms       | 0.428x    |
| `alphabet-pallet`             | 2.61 ms       | 8.50 µs       | 0.00325x  | 4.49 ms       | 1.64 ms       | 0.364x    |
| `alphabet-tiny`               | 397 µs        | 2.82 µs       | 0.00709x  | 2.18 ms       | 1.69 ms       | 0.774x    |

<!-- perf:compare:speed:end -->

### Allocations, one render

<!-- perf:compare:allocations:start -->
Ran 2026-10-25 on macos aarch64, Apple M1 Max, rustc 1.99.1 (58cf8f9c3 2026-10-13)

| scenario                      | v3 api allocations | v4 api allocations | ratio    | v3 api bytes  | v4 api bytes | v3 cli allocations | v4 cli allocations | v3 cli bytes  | v4 cli bytes |
|-------------------------------|--------------------|--------------------|----------|---------------|--------------|--------------------|--------------------|---------------|--------------|
| `short-plain`                 | 3,494              | 43                 | 0.0123x  | 253,868 B     | 9,872 B      | 3,741              | 241                | 290,590 B     | 29,968 B     |
| `short-colors`                | 4,267              | 45                 | 0.0105x  | 586,796 B     | 14,064 B     | 4,525              | 252                | 623,834 B     | 34,352 B     |
| `short-gradient`              | 14,774             | 55                 | 0.00372x | 944,573 B     | 39,371 B     | 14,874             | 261                | 980,392 B     | 59,643 B     |
| `short-transition`            | 12,643             | 55                 | 0.00435x | 808,046 B     | 39,374 B     | 12,753             | 265                | 844,356 B     | 59,700 B     |
| `short-background`            | 3,504              | 43                 | 0.0123x  | 254,654 B     | 9,883 B      | 3,760              | 248                | 291,767 B     | 30,103 B     |
| `short-colors-background`     | 4,277              | 45                 | 0.0105x  | 587,497 B     | 14,075 B     | 4,545              | 259                | 625,230 B     | 34,511 B     |
| `short-gradient-background`   | 14,785             | 55                 | 0.00372x | 948,901 B     | 39,382 B     | 14,895             | 268                | 985,415 B     | 59,802 B     |
| `short-transition-background` | 12,654             | 55                 | 0.00435x | 812,345 B     | 39,385 B     | 12,773             | 272                | 849,046 B     | 59,859 B     |
| `long-plain`                  | 143,510            | 794                | 0.00553x | 257,913,870 B | 305,772 B    | 143,287            | 993                | 257,946,559 B | 326,962 B    |
| `long-colors`                 | 180,956            | 796                | 0.00440x | 789,917,703 B | 551,623 B    | 181,021            | 1,004              | 789,955,140 B | 573,005 B    |
| `long-gradient`               | 596,026            | 807                | 0.00135x | 285,085,688 B | 2,239,915 B  | 591,153            | 1,013              | 285,081,450 B | 2,261,265 B  |
| `long-transition`             | 534,490            | 807                | 0.00151x | 281,308,607 B | 2,239,918 B  | 529,797            | 1,017              | 281,305,924 B | 2,261,322 B  |
| `long-background`             | 143,520            | 795                | 0.00554x | 257,937,981 B | 404,091 B    | 143,306            | 1,000              | 257,971,061 B | 425,389 B    |
| `long-colors-background`      | 180,966            | 797                | 0.00440x | 789,918,615 B | 666,331 B    | 181,040            | 1,011              | 789,956,443 B | 687,845 B    |
| `long-gradient-background`    | 596,036            | 807                | 0.00135x | 285,090,812 B | 2,239,926 B  | 591,172            | 1,020              | 285,086,965 B | 2,261,424 B  |
| `long-transition-background`  | 534,500            | 807                | 0.00151x | 281,313,752 B | 2,239,929 B  | 529,816            | 1,024              | 281,311,460 B | 2,261,481 B  |
| `startup`                     | n/a                | n/a                | n/a      | n/a           | n/a          | 249                | 190                | 36,738 B      | 19,937 B     |
| `single-character`            | 1,065              | 22                 | 0.0207x  | 59,060 B      | 1,390 B      | 1,318              | 221                | 95,855 B      | 21,482 B     |
| `console-font`                | 496                | 11                 | 0.0222x  | 30,529 B      | 1,828 B      | 747                | 209                | 67,321 B      | 21,928 B     |
| `line-breaks`                 | 4,694              | 56                 | 0.0119x  | 419,180 B     | 10,992 B     | 4,935              | 254                | 455,824 B     | 31,100 B     |
| `long-center`                 | 143,526            | 794                | 0.00553x | 261,060,329 B | 305,772 B    | 143,307            | 996                | 261,093,216 B | 327,048 B    |
| `long-right`                  | 143,499            | 794                | 0.00553x | 263,036,111 B | 305,772 B    | 143,278            | 996                | 263,068,963 B | 327,046 B    |
| `long-spacing`                | 145,805            | 964                | 0.00661x | 299,703,033 B | 368,796 B    | 145,618            | 1,167              | 299,736,566 B | 390,160 B    |
| `long-independent-gradient`   | 585,488            | 807                | 0.00138x | 284,810,803 B | 2,239,915 B  | 580,909            | 1,014              | 284,809,001 B | 2,261,315 B  |
| `long-candy`                  | 181,053            | 797                | 0.00440x | 789,939,043 B | 552,151 B    | 181,121            | 1,005              | 789,977,025 B | 573,542 B    |
| `scaling-125`                 | 32,138             | 192                | 0.00597x | 14,440,237 B  | 76,412 B     | 32,274             | 391                | 14,475,813 B  | 96,752 B     |
| `scaling-250`                 | 64,435             | 376                | 0.00584x | 53,816,426 B  | 150,060 B    | 64,458             | 575                | 53,851,251 B  | 170,650 B    |
| `scaling-500`                 | 130,055            | 716                | 0.00551x | 209,185,537 B | 297,612 B    | 129,876            | 915                | 209,218,630 B | 318,702 B    |
| `scaling-1000`                | 264,502            | 1,407              | 0.00532x | 828,885,217 B | 593,020 B    | 263,967            | 1,606              | 828,915,614 B | 615,110 B    |
| `alphabet-console`            | 887                | 13                 | 0.0147x  | 44,031 B      | 3,320 B      | 1,139              | 211                | 80,884 B      | 23,450 B     |
| `alphabet-block`              | 7,453              | 72                 | 0.00966x | 959,544 B     | 22,462 B     | 7,685              | 271                | 996,071 B     | 42,604 B     |
| `alphabet-simpleblock`        | 3,634              | 62                 | 0.0171x  | 515,181 B     | 14,404 B     | 3,859              | 260                | 551,614 B     | 34,542 B     |
| `alphabet-simple`             | 2,292              | 44                 | 0.0192x  | 214,925 B     | 9,544 B      | 2,532              | 242                | 251,583 B     | 29,672 B     |
| `alphabet-3d`                 | 11,460             | 172                | 0.0150x  | 2,073,194 B   | 43,252 B     | 11,634             | 370                | 2,108,784 B   | 63,372 B     |
| `alphabet-chrome`             | 4,041              | 31                 | 0.00767x | 210,545 B     | 10,092 B     | 4,288              | 229                | 247,315 B     | 30,220 B     |
| `alphabet-huge`               | 14,303             | 134                | 0.00937x | 2,613,243 B   | 39,431 B     | 14,503             | 333                | 2,649,255 B   | 59,571 B     |
| `alphabet-shade`              | 8,341              | 52                 | 0.00623x | 702,944 B     | 17,848 B     | 8,468              | 250                | 737,791 B     | 37,974 B     |
| `alphabet-slick`              | 6,603              | 58                 | 0.00878x | 771,311 B     | 18,463 B     | 6,842              | 257                | 807,950 B     | 38,605 B     |
| `alphabet-grid`               | 6,243              | 45                 | 0.00721x | 581,442 B     | 18,688 B     | 6,448              | 243                | 617,534 B     | 38,812 B     |
| `alphabet-pallet`             | 6,603              | 58                 | 0.00878x | 771,312 B     | 18,463 B     | 6,842              | 257                | 807,954 B     | 38,607 B     |
| `alphabet-tiny`               | 1,495              | 26                 | 0.0174x  | 137,706 B     | 8,400 B      | 1,744              | 224                | 174,502 B     | 28,524 B     |

<!-- perf:compare:allocations:end -->

### Memory, peak per render

<!-- perf:compare:memory:start -->
Ran 2026-10-25 on macos aarch64, Apple M1 Max, rustc 1.99.1 (58cf8f9c3 2026-10-13)

| scenario                      | v3 api peak heap | v4 api peak heap | ratio   | v3 cli peak heap | v4 cli peak heap | v3 cli peak RSS | v4 cli peak RSS |
|-------------------------------|------------------|------------------|---------|------------------|------------------|-----------------|-----------------|
| `short-plain`                 | 46,221 B         | 4,760 B          | 0.103x  | 61,328 B         | 24,096 B         | 2,146,304 B     | 1,687,552 B     |
| `short-colors`                | 55,813 B         | 6,904 B          | 0.124x  | 71,344 B         | 26,320 B         | 2,342,912 B     | 1,687,552 B     |
| `short-gradient`              | 107,901 B        | 19,551 B         | 0.181x  | 125,488 B        | 38,976 B         | 2,572,288 B     | 1,736,704 B     |
| `short-transition`            | 107,815 B        | 19,554 B         | 0.181x  | 125,504 B        | 38,992 B         | 2,555,904 B     | 1,736,704 B     |
| `short-background`            | 46,448 B         | 4,779 B          | 0.103x  | 61,632 B         | 24,192 B         | 2,195,456 B     | 1,687,552 B     |
| `short-colors-background`     | 55,856 B         | 6,923 B          | 0.124x  | 71,408 B         | 26,448 B         | 2,359,296 B     | 1,687,552 B     |
| `short-gradient-background`   | 107,944 B        | 19,570 B         | 0.181x  | 125,488 B        | 39,104 B         | 2,572,288 B     | 1,720,320 B     |
| `short-transition-background` | 107,858 B        | 19,573 B         | 0.181x  | 125,504 B        | 39,120 B         | 2,588,672 B     | 1,720,320 B     |
| `long-plain`                  | 308,271 B        | 191,352 B        | 0.621x  | 341,760 B        | 217,568 B        | 3,096,576 B     | 2,031,616 B     |
| `long-colors`                 | 613,595 B        | 314,328 B        | 0.512x  | 662,368 B        | 348,816 B        | 3,686,400 B     | 2,113,536 B     |
| `long-gradient`               | 2,782,547 B      | 1,158,463 B      | 0.416x  | 2,833,216 B      | 1,184,768 B      | 7,012,352 B     | 2,621,440 B     |
| `long-transition`             | 2,781,459 B      | 1,158,466 B      | 0.416x  | 2,833,232 B      | 1,184,784 B      | 6,029,312 B     | 2,621,440 B     |
| `long-background`             | 320,090 B        | 240,523 B        | 0.751x  | 358,176 B        | 266,816 B        | 3,260,416 B     | 2,048,000 B     |
| `long-colors-background`      | 613,595 B        | 371,691 B        | 0.606x  | 662,368 B        | 398,096 B        | 3,833,856 B     | 2,129,920 B     |
| `long-gradient-background`    | 2,782,547 B      | 1,158,482 B      | 0.416x  | 2,833,216 B      | 1,184,896 B      | 6,012,928 B     | 2,621,440 B     |
| `long-transition-background`  | 2,781,459 B      | 1,158,485 B      | 0.417x  | 2,833,232 B      | 1,184,912 B      | 6,160,384 B     | 2,621,440 B     |
| `startup`                     | n/a              | n/a              | n/a     | 34,048 B         | 19,120 B         | 1,671,168 B     | 1,572,864 B     |
| `single-character`            | 43,639 B         | 1,080 B          | 0.0247x | 58,464 B         | 20,352 B         | 1,867,776 B     | 1,687,552 B     |
| `console-font`                | 24,888 B         | 1,292 B          | 0.0519x | 37,840 B         | 20,496 B         | 1,802,240 B     | 1,687,552 B     |
| `line-breaks`                 | 49,127 B         | 6,264 B          | 0.128x  | 64,832 B         | 25,888 B         | 2,244,608 B     | 1,687,552 B     |
| `long-center`                 | 313,050 B        | 191,352 B        | 0.611x  | 343,264 B        | 217,632 B        | 3,194,880 B     | 2,031,616 B     |
| `long-right`                  | 316,976 B        | 191,352 B        | 0.604x  | 344,576 B        | 217,632 B        | 3,194,880 B     | 2,031,616 B     |
| `long-spacing`                | 331,946 B        | 222,888 B        | 0.671x  | 381,360 B        | 257,184 B        | 3,391,488 B     | 2,113,536 B     |
| `long-independent-gradient`   | 2,747,475 B      | 1,158,463 B      | 0.422x  | 2,785,088 B      | 1,184,816 B      | 6,209,536 B     | 2,605,056 B     |
| `long-candy`                  | 613,939 B        | 314,856 B        | 0.513x  | 662,944 B        | 349,456 B        | 3,850,240 B     | 2,080,768 B     |
| `scaling-125`                 | 103,986 B        | 46,344 B         | 0.446x  | 124,064 B        | 67,552 B         | 2,572,288 B     | 1,802,240 B     |
| `scaling-250`                 | 167,123 B        | 91,960 B         | 0.550x  | 197,376 B        | 122,848 B        | 2,883,584 B     | 1,900,544 B     |
| `scaling-500`                 | 293,214 B        | 183,576 B        | 0.626x  | 331,632 B        | 209,248 B        | 3,112,960 B     | 2,031,616 B     |
| `scaling-1000`                | 546,384 B        | 367,048 B        | 0.672x  | 576,192 B        | 406,912 B        | 3,489,792 B     | 2,228,224 B     |
| `alphabet-console`            | 24,888 B         | 2,112 B          | 0.0849x | 37,856 B         | 21,392 B         | 1,835,008 B     | 1,687,552 B     |
| `alphabet-block`              | 54,549 B         | 12,168 B         | 0.223x  | 70,624 B         | 32,288 B         | 2,457,600 B     | 1,720,320 B     |
| `alphabet-simpleblock`        | 41,021 B         | 8,396 B          | 0.205x  | 56,352 B         | 28,064 B         | 2,179,072 B     | 1,687,552 B     |
| `alphabet-simple`             | 26,574 B         | 5,392 B          | 0.203x  | 41,088 B         | 24,960 B         | 2,129,920 B     | 1,703,936 B     |
| `alphabet-3d`                 | 81,432 B         | 24,828 B         | 0.305x  | 98,448 B         | 44,896 B         | 2,490,368 B     | 1,736,704 B     |
| `alphabet-chrome`             | 27,444 B         | 5,300 B          | 0.193x  | 40,976 B         | 24,640 B         | 2,113,536 B     | 1,687,552 B     |
| `alphabet-huge`               | 107,751 B        | 23,268 B         | 0.216x  | 127,664 B        | 44,272 B         | 2,621,440 B     | 1,753,088 B     |
| `alphabet-shade`              | 50,942 B         | 10,560 B         | 0.207x  | 68,208 B         | 30,016 B         | 2,342,912 B     | 1,720,320 B     |
| `alphabet-slick`              | 49,279 B         | 10,152 B         | 0.206x  | 65,136 B         | 30,592 B         | 2,293,760 B     | 1,736,704 B     |
| `alphabet-grid`               | 46,284 B         | 8,968 B          | 0.194x  | 62,000 B         | 28,672 B         | 2,359,296 B     | 1,671,168 B     |
| `alphabet-pallet`             | 49,280 B         | 10,152 B         | 0.206x  | 65,104 B         | 30,592 B         | 2,310,144 B     | 1,720,320 B     |
| `alphabet-tiny`               | 25,652 B         | 4,632 B          | 0.181x  | 38,800 B         | 23,936 B         | 1,982,464 B     | 1,671,168 B     |

<!-- perf:compare:memory:end -->
