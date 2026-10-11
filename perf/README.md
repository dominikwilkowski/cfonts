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
Ran 2026-10-11 on macos aarch64, Apple M1 Max, rustc 1.99.0 (b940084d7 2026-09-28)

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
Ran 2026-10-10 on macos aarch64, Apple M1 Max, rustc 1.99.0 (b940084d7 2026-09-28)

| scenario                      | v4 api median | v4 cli median |
|-------------------------------|---------------|---------------|
| `short-plain`                 | 4.47 µs       | 1.78 ms       |
| `short-colors`                | 5.90 µs       | 1.84 ms       |
| `short-gradient`              | 12.4 µs       | 1.79 ms       |
| `short-transition`            | 11.7 µs       | 1.79 ms       |
| `short-background`            | 4.62 µs       | 2.00 ms       |
| `short-colors-background`     | 6.11 µs       | 1.96 ms       |
| `short-gradient-background`   | 12.7 µs       | 2.55 ms       |
| `short-transition-background` | 12.3 µs       | 2.22 ms       |
| `long-plain`                  | 187 µs        | 2.45 ms       |
| `long-colors`                 | 239 µs        | 2.41 ms       |
| `long-gradient`               | 488 µs        | 2.79 ms       |
| `long-transition`             | 486 µs        | 2.83 ms       |
| `long-background`             | 190 µs        | 2.18 ms       |
| `long-colors-background`      | 241 µs        | 2.35 ms       |
| `long-gradient-background`    | 492 µs        | 2.73 ms       |
| `long-transition-background`  | 483 µs        | 2.95 ms       |
| `startup`                     | n/a           | 2.01 ms       |
| `single-character`            | 914 ns        | 2.45 ms       |
| `console-font`                | 725 ns        | 2.32 ms       |
| `line-breaks`                 | 6.29 µs       | 1.89 ms       |
| `long-center`                 | 185 µs        | 2.07 ms       |
| `long-right`                  | 186 µs        | 2.14 ms       |
| `long-spacing`                | 263 µs        | 2.19 ms       |
| `long-independent-gradient`   | 804 µs        | 2.94 ms       |
| `long-candy`                  | 238 µs        | 2.14 ms       |
| `scaling-125`                 | 42.6 µs       | 1.86 ms       |
| `scaling-250`                 | 84.7 µs       | 1.94 ms       |
| `scaling-500`                 | 166 µs        | 2.17 ms       |
| `scaling-1000`                | 332 µs        | 2.24 ms       |
| `alphabet-console`            | 1.15 µs       | 2.24 ms       |
| `alphabet-block`              | 10.6 µs       | 2.17 ms       |
| `alphabet-simpleblock`        | 6.07 µs       | 1.89 ms       |
| `alphabet-simple`             | 5.01 µs       | 1.93 ms       |
| `alphabet-3d`                 | 16.7 µs       | 1.88 ms       |
| `alphabet-chrome`             | 3.91 µs       | 1.94 ms       |
| `alphabet-huge`               | 20.4 µs       | 2.32 ms       |
| `alphabet-shade`              | 10.1 µs       | 2.01 ms       |
| `alphabet-slick`              | 8.32 µs       | 1.83 ms       |
| `alphabet-grid`               | 6.89 µs       | 1.80 ms       |
| `alphabet-pallet`             | 8.20 µs       | 1.80 ms       |
| `alphabet-tiny`               | 2.77 µs       | 1.79 ms       |

<!-- perf:results:speed:end -->

### Allocations, one render

<!-- perf:results:allocations:start -->
Ran 2026-10-10 on macos aarch64, Apple M1 Max, rustc 1.99.0 (b940084d7 2026-09-28)

| scenario                      | v4 api allocations | v4 api bytes | v4 cli allocations | v4 cli bytes |
|-------------------------------|--------------------|--------------|--------------------|--------------|
| `short-plain`                 | 45                 | 12,697 B     | 243                | 32,809 B     |
| `short-colors`                | 47                 | 16,889 B     | 254                | 37,193 B     |
| `short-gradient`              | 55                 | 42,131 B     | 261                | 62,419 B     |
| `short-transition`            | 55                 | 42,134 B     | 265                | 62,476 B     |
| `short-background`            | 45                 | 12,708 B     | 250                | 32,944 B     |
| `short-colors-background`     | 47                 | 16,900 B     | 261                | 37,352 B     |
| `short-gradient-background`   | 55                 | 42,142 B     | 268                | 62,578 B     |
| `short-transition-background` | 55                 | 42,145 B     | 272                | 62,635 B     |
| `long-plain`                  | 796                | 434,357 B    | 995                | 455,563 B    |
| `long-colors`                 | 798                | 680,208 B    | 1,006              | 701,606 B    |
| `long-gradient`               | 807                | 2,368,435 B  | 1,013              | 2,389,801 B  |
| `long-transition`             | 807                | 2,368,438 B  | 1,017              | 2,389,858 B  |
| `long-background`             | 797                | 532,676 B    | 1,002              | 553,990 B    |
| `long-colors-background`      | 799                | 794,916 B    | 1,013              | 816,446 B    |
| `long-gradient-background`    | 807                | 2,368,446 B  | 1,020              | 2,389,960 B  |
| `long-transition-background`  | 807                | 2,368,449 B  | 1,024              | 2,390,017 B  |
| `startup`                     | n/a                | n/a          | 190                | 19,953 B     |
| `single-character`            | 24                 | 1,815 B      | 223                | 21,923 B     |
| `console-font`                | 13                 | 2,153 B      | 211                | 22,269 B     |
| `line-breaks`                 | 58                 | 14,777 B     | 256                | 34,901 B     |
| `long-center`                 | 796                | 434,357 B    | 998                | 455,649 B    |
| `long-right`                  | 796                | 434,357 B    | 998                | 455,647 B    |
| `long-spacing`                | 968                | 676,181 B    | 1,171              | 697,561 B    |
| `long-independent-gradient`   | 807                | 2,368,435 B  | 1,014              | 2,389,851 B  |
| `long-candy`                  | 799                | 680,736 B    | 1,007              | 702,143 B    |
| `scaling-125`                 | 194                | 106,357 B    | 393                | 126,713 B    |
| `scaling-250`                 | 378                | 208,565 B    | 577                | 229,171 B    |
| `scaling-500`                 | 718                | 414,197 B    | 917                | 435,303 B    |
| `scaling-1000`                | 1,409              | 826,005 B    | 1,608              | 848,111 B    |
| `alphabet-console`            | 15                 | 3,945 B      | 213                | 24,091 B     |
| `alphabet-block`              | 74                 | 28,887 B     | 273                | 49,045 B     |
| `alphabet-simpleblock`        | 64                 | 19,769 B     | 262                | 39,923 B     |
| `alphabet-simple`             | 46                 | 13,849 B     | 244                | 33,993 B     |
| `alphabet-3d`                 | 174                | 52,857 B     | 372                | 72,993 B     |
| `alphabet-chrome`             | 33                 | 13,337 B     | 231                | 33,481 B     |
| `alphabet-huge`               | 136                | 51,156 B     | 335                | 71,312 B     |
| `alphabet-shade`              | 54                 | 26,393 B     | 252                | 46,535 B     |
| `alphabet-slick`              | 60                 | 24,888 B     | 259                | 45,046 B     |
| `alphabet-grid`               | 47                 | 25,113 B     | 245                | 45,253 B     |
| `alphabet-pallet`             | 60                 | 24,888 B     | 259                | 45,048 B     |
| `alphabet-tiny`               | 28                 | 10,585 B     | 226                | 30,725 B     |

<!-- perf:results:allocations:end -->

### Memory, peak per render

<!-- perf:results:memory:start -->
Ran 2026-10-10 on macos aarch64, Apple M1 Max, rustc 1.99.0 (b940084d7 2026-09-28)

| scenario                      | v4 api peak heap | v4 cli peak heap | v4 cli peak RSS |
|-------------------------------|------------------|------------------|-----------------|
| `short-plain`                 | 7,585 B          | 27,264 B         | 2,064,384 B     |
| `short-colors`                | 9,729 B          | 29,488 B         | 2,064,384 B     |
| `short-gradient`              | 22,311 B         | 42,064 B         | 2,097,152 B     |
| `short-transition`            | 22,314 B         | 42,080 B         | 2,097,152 B     |
| `short-background`            | 7,604 B          | 27,360 B         | 2,064,384 B     |
| `short-colors-background`     | 9,748 B          | 29,616 B         | 2,064,384 B     |
| `short-gradient-background`   | 22,330 B         | 42,192 B         | 2,097,152 B     |
| `short-transition-background` | 22,333 B         | 42,208 B         | 2,097,152 B     |
| `long-plain`                  | 319,937 B        | 354,368 B        | 2,572,288 B     |
| `long-colors`                 | 442,913 B        | 485,616 B        | 2,588,672 B     |
| `long-gradient`               | 1,286,983 B      | 1,321,488 B      | 3,096,576 B     |
| `long-transition`             | 1,286,986 B      | 1,321,504 B      | 3,096,576 B     |
| `long-background`             | 369,108 B        | 403,616 B        | 2,555,904 B     |
| `long-colors-background`      | 500,276 B        | 534,896 B        | 2,621,440 B     |
| `long-gradient-background`    | 1,287,002 B      | 1,321,616 B      | 3,096,576 B     |
| `long-transition-background`  | 1,287,005 B      | 1,321,632 B      | 3,112,960 B     |
| `startup`                     | n/a              | 19,136 B         | 1,998,848 B     |
| `single-character`            | 1,440 B          | 20,656 B         | 2,064,384 B     |
| `console-font`                | 1,552 B          | 20,800 B         | 2,080,768 B     |
| `line-breaks`                 | 10,049 B         | 29,632 B         | 2,064,384 B     |
| `long-center`                 | 319,937 B        | 354,432 B        | 2,539,520 B     |
| `long-right`                  | 319,937 B        | 354,432 B        | 2,539,520 B     |
| `long-spacing`                | 527,393 B        | 585,792 B        | 2,801,664 B     |
| `long-independent-gradient`   | 1,286,983 B      | 1,321,536 B      | 3,096,576 B     |
| `long-candy`                  | 443,441 B        | 486,256 B        | 2,605,056 B     |
| `scaling-125`                 | 76,289 B         | 98,368 B         | 2,195,456 B     |
| `scaling-250`                 | 150,465 B        | 184,000 B        | 2,326,528 B     |
| `scaling-500`                 | 300,161 B        | 332,608 B        | 2,523,136 B     |
| `scaling-1000`                | 600,033 B        | 657,184 B        | 2,834,432 B     |
| `alphabet-console`            | 2,672 B          | 21,920 B         | 2,080,768 B     |
| `alphabet-block`              | 18,593 B         | 39,008 B         | 2,097,152 B     |
| `alphabet-simpleblock`        | 13,761 B         | 33,280 B         | 2,097,152 B     |
| `alphabet-simple`             | 9,632 B          | 29,136 B         | 2,080,768 B     |
| `alphabet-3d`                 | 34,433 B         | 54,208 B         | 2,146,304 B     |
| `alphabet-chrome`             | 8,480 B          | 27,824 B         | 2,080,768 B     |
| `alphabet-huge`               | 34,993 B         | 55,632 B         | 2,129,920 B     |
| `alphabet-shade`              | 19,105 B         | 39,328 B         | 2,097,152 B     |
| `alphabet-slick`              | 16,577 B         | 37,120 B         | 2,097,152 B     |
| `alphabet-grid`               | 15,393 B         | 36,064 B         | 2,080,768 B     |
| `alphabet-pallet`             | 16,577 B         | 37,120 B         | 2,097,152 B     |
| `alphabet-tiny`               | 6,752 B          | 26,064 B         | 2,080,768 B     |

<!-- perf:results:memory:end -->

## Compared with v3

`make compare` writes this section.
A ratio is v4 over v3, `0.25x` means v4 needs a quarter of what v3 needs.

### Speed, median time per render

<!-- perf:compare:speed:start -->
Ran 2026-10-11 on macos aarch64, Apple M1 Max, rustc 1.99.0 (b940084d7 2026-09-28)

| scenario                      | v3 api median | v4 api median | api ratio | v3 cli median | v4 cli median | cli ratio |
|-------------------------------|---------------|---------------|-----------|---------------|---------------|-----------|
| `short-plain`                 | 686 µs        | 4.79 µs       | 0.00699x  | 2.99 ms       | 1.93 ms       | 0.645x    |
| `short-colors`                | 935 µs        | 6.14 µs       | 0.00657x  | 2.91 ms       | 1.71 ms       | 0.588x    |
| `short-gradient`              | 1.58 ms       | 16.4 µs       | 0.0103x   | 3.63 ms       | 1.69 ms       | 0.464x    |
| `short-transition`            | 1.32 ms       | 15.5 µs       | 0.0117x   | 3.33 ms       | 1.65 ms       | 0.495x    |
| `short-background`            | 687 µs        | 4.94 µs       | 0.00719x  | 2.58 ms       | 1.67 ms       | 0.649x    |
| `short-colors-background`     | 929 µs        | 6.29 µs       | 0.00678x  | 2.85 ms       | 1.69 ms       | 0.593x    |
| `short-gradient-background`   | 1.59 ms       | 16.5 µs       | 0.0104x   | 3.66 ms       | 1.69 ms       | 0.462x    |
| `short-transition-background` | 1.32 ms       | 15.7 µs       | 0.0119x   | 3.50 ms       | 1.77 ms       | 0.507x    |
| `long-plain`                  | 881 ms        | 208 µs        | 0.000236x | 886 ms        | 2.15 ms       | 0.00243x  |
| `long-colors`                 | 1.25 s        | 250 µs        | 0.000200x | 1.25 s        | 2.02 ms       | 0.00162x  |
| `long-gradient`               | 901 ms        | 633 µs        | 0.000703x | 916 ms        | 2.60 ms       | 0.00284x  |
| `long-transition`             | 893 ms        | 635 µs        | 0.000711x | 899 ms        | 2.57 ms       | 0.00286x  |
| `long-background`             | 867 ms        | 207 µs        | 0.000238x | 868 ms        | 1.99 ms       | 0.00230x  |
| `long-colors-background`      | 1.24 s        | 253 µs        | 0.000204x | 1.25 s        | 2.14 ms       | 0.00171x  |
| `long-gradient-background`    | 899 ms        | 639 µs        | 0.000710x | 907 ms        | 2.60 ms       | 0.00287x  |
| `long-transition-background`  | 893 ms        | 638 µs        | 0.000715x | 901 ms        | 2.58 ms       | 0.00286x  |
| `startup`                     | n/a           | n/a           | n/a       | 1.68 ms       | 1.63 ms       | 0.973x    |
| `single-character`            | 66.1 µs       | 881 ns        | 0.0133x   | 1.82 ms       | 1.78 ms       | 0.973x    |
| `console-font`                | 22.8 µs       | 638 ns        | 0.0279x   | 1.73 ms       | 1.66 ms       | 0.958x    |
| `line-breaks`                 | 1.21 ms       | 6.46 µs       | 0.00533x  | 3.12 ms       | 1.67 ms       | 0.535x    |
| `long-center`                 | 868 ms        | 203 µs        | 0.000234x | 869 ms        | 2.02 ms       | 0.00233x  |
| `long-right`                  | 870 ms        | 204 µs        | 0.000235x | 875 ms        | 1.97 ms       | 0.00226x  |
| `long-spacing`                | 913 ms        | 285 µs        | 0.000312x | 919 ms        | 2.07 ms       | 0.00225x  |
| `long-independent-gradient`   | 898 ms        | 961 µs        | 0.00107x  | 908 ms        | 2.89 ms       | 0.00318x  |
| `long-candy`                  | 1.24 s        | 254 µs        | 0.000206x | 1.25 s        | 2.04 ms       | 0.00163x  |
| `scaling-125`                 | 48.5 ms       | 47.3 µs       | 0.000976x | 50.6 ms       | 1.76 ms       | 0.0347x   |
| `scaling-250`                 | 183 ms        | 92.5 µs       | 0.000505x | 188 ms        | 1.82 ms       | 0.00966x  |
| `scaling-500`                 | 718 ms        | 184 µs        | 0.000257x | 725 ms        | 1.97 ms       | 0.00272x  |
| `scaling-1000`                | 2.84 s        | 368 µs        | 0.000130x | 2.85 s        | 2.22 ms       | 0.000778x |
| `alphabet-console`            | 39.1 µs       | 1.28 µs       | 0.0327x   | 1.77 ms       | 1.65 ms       | 0.932x    |
| `alphabet-block`              | 3.20 ms       | 10.8 µs       | 0.00338x  | 5.30 ms       | 1.68 ms       | 0.318x    |
| `alphabet-simpleblock`        | 285 µs        | 6.86 µs       | 0.0241x   | 2.23 ms       | 1.72 ms       | 0.770x    |
| `alphabet-simple`             | 175 µs        | 5.74 µs       | 0.0328x   | 2.01 ms       | 1.66 ms       | 0.829x    |
| `alphabet-3d`                 | 2.38 ms       | 17.7 µs       | 0.00747x  | 4.46 ms       | 1.72 ms       | 0.385x    |
| `alphabet-chrome`             | 631 µs        | 4.44 µs       | 0.00704x  | 2.48 ms       | 1.70 ms       | 0.684x    |
| `alphabet-huge`               | 7.19 ms       | 21.2 µs       | 0.00295x  | 9.20 ms       | 1.70 ms       | 0.185x    |
| `alphabet-shade`              | 2.36 ms       | 11.3 µs       | 0.00477x  | 4.52 ms       | 1.70 ms       | 0.375x    |
| `alphabet-slick`              | 2.63 ms       | 9.40 µs       | 0.00357x  | 4.64 ms       | 1.67 ms       | 0.360x    |
| `alphabet-grid`               | 1.94 ms       | 7.86 µs       | 0.00404x  | 3.81 ms       | 1.72 ms       | 0.452x    |
| `alphabet-pallet`             | 2.62 ms       | 9.33 µs       | 0.00356x  | 4.55 ms       | 1.74 ms       | 0.382x    |
| `alphabet-tiny`               | 396 µs        | 3.15 µs       | 0.00797x  | 2.19 ms       | 1.65 ms       | 0.752x    |

<!-- perf:compare:speed:end -->

### Allocations, one render

<!-- perf:compare:allocations:start -->
Ran 2026-10-11 on macos aarch64, Apple M1 Max, rustc 1.99.0 (b940084d7 2026-09-28)

| scenario                      | v3 api allocations | v4 api allocations | ratio    | v3 api bytes  | v4 api bytes | v3 cli allocations | v4 cli allocations | v3 cli bytes  | v4 cli bytes |
|-------------------------------|--------------------|--------------------|----------|---------------|--------------|--------------------|--------------------|---------------|--------------|
| `short-plain`                 | 3,494              | 45                 | 0.0129x  | 253,868 B     | 9,937 B      | 3,741              | 243                | 290,590 B     | 30,033 B     |
| `short-colors`                | 4,267              | 47                 | 0.0110x  | 586,796 B     | 14,129 B     | 4,525              | 254                | 623,834 B     | 34,417 B     |
| `short-gradient`              | 14,774             | 55                 | 0.00372x | 944,573 B     | 39,371 B     | 14,874             | 261                | 980,392 B     | 59,643 B     |
| `short-transition`            | 12,643             | 55                 | 0.00435x | 808,046 B     | 39,374 B     | 12,753             | 265                | 844,356 B     | 59,700 B     |
| `short-background`            | 3,504              | 45                 | 0.0128x  | 254,654 B     | 9,948 B      | 3,760              | 250                | 291,767 B     | 30,168 B     |
| `short-colors-background`     | 4,277              | 47                 | 0.0110x  | 587,497 B     | 14,140 B     | 4,545              | 261                | 625,230 B     | 34,576 B     |
| `short-gradient-background`   | 14,785             | 55                 | 0.00372x | 948,901 B     | 39,382 B     | 14,895             | 268                | 985,415 B     | 59,802 B     |
| `short-transition-background` | 12,654             | 55                 | 0.00435x | 812,345 B     | 39,385 B     | 12,773             | 272                | 849,046 B     | 59,859 B     |
| `long-plain`                  | 143,510            | 796                | 0.00555x | 257,913,870 B | 305,837 B    | 143,287            | 995                | 257,946,559 B | 327,027 B    |
| `long-colors`                 | 180,956            | 798                | 0.00441x | 789,917,703 B | 551,688 B    | 181,021            | 1,006              | 789,955,140 B | 573,070 B    |
| `long-gradient`               | 596,026            | 807                | 0.00135x | 285,085,688 B | 2,239,915 B  | 591,153            | 1,013              | 285,081,450 B | 2,261,265 B  |
| `long-transition`             | 534,490            | 807                | 0.00151x | 281,308,607 B | 2,239,918 B  | 529,797            | 1,017              | 281,305,924 B | 2,261,322 B  |
| `long-background`             | 143,520            | 797                | 0.00555x | 257,937,981 B | 404,156 B    | 143,306            | 1,002              | 257,971,061 B | 425,454 B    |
| `long-colors-background`      | 180,966            | 799                | 0.00442x | 789,918,615 B | 666,396 B    | 181,040            | 1,013              | 789,956,443 B | 687,910 B    |
| `long-gradient-background`    | 596,036            | 807                | 0.00135x | 285,090,812 B | 2,239,926 B  | 591,172            | 1,020              | 285,086,965 B | 2,261,424 B  |
| `long-transition-background`  | 534,500            | 807                | 0.00151x | 281,313,752 B | 2,239,929 B  | 529,816            | 1,024              | 281,311,460 B | 2,261,481 B  |
| `startup`                     | n/a                | n/a                | n/a      | n/a           | n/a          | 249                | 190                | 36,738 B      | 19,937 B     |
| `single-character`            | 1,065              | 24                 | 0.0225x  | 59,060 B      | 1,455 B      | 1,318              | 223                | 95,855 B      | 21,547 B     |
| `console-font`                | 496                | 13                 | 0.0262x  | 30,529 B      | 1,881 B      | 747                | 211                | 67,321 B      | 21,981 B     |
| `line-breaks`                 | 4,694              | 58                 | 0.0124x  | 419,180 B     | 11,057 B     | 4,935              | 256                | 455,824 B     | 31,165 B     |
| `long-center`                 | 143,526            | 796                | 0.00555x | 261,060,329 B | 305,837 B    | 143,307            | 998                | 261,093,216 B | 327,113 B    |
| `long-right`                  | 143,499            | 796                | 0.00555x | 263,036,111 B | 305,837 B    | 143,278            | 998                | 263,068,963 B | 327,111 B    |
| `long-spacing`                | 145,805            | 968                | 0.00664x | 299,703,033 B | 371,741 B    | 145,618            | 1,171              | 299,736,566 B | 393,105 B    |
| `long-independent-gradient`   | 585,488            | 807                | 0.00138x | 284,810,803 B | 2,239,915 B  | 580,909            | 1,014              | 284,809,001 B | 2,261,315 B  |
| `long-candy`                  | 181,053            | 799                | 0.00441x | 789,939,043 B | 552,216 B    | 181,121            | 1,007              | 789,977,025 B | 573,607 B    |
| `scaling-125`                 | 32,138             | 194                | 0.00604x | 14,440,237 B  | 76,477 B     | 32,274             | 393                | 14,475,813 B  | 96,817 B     |
| `scaling-250`                 | 64,435             | 378                | 0.00587x | 53,816,426 B  | 150,125 B    | 64,458             | 577                | 53,851,251 B  | 170,715 B    |
| `scaling-500`                 | 130,055            | 718                | 0.00552x | 209,185,537 B | 297,677 B    | 129,876            | 917                | 209,218,630 B | 318,767 B    |
| `scaling-1000`                | 264,502            | 1,409              | 0.00533x | 828,885,217 B | 593,085 B    | 263,967            | 1,608              | 828,915,614 B | 615,175 B    |
| `alphabet-console`            | 887                | 15                 | 0.0169x  | 44,031 B      | 3,385 B      | 1,139              | 213                | 80,884 B      | 23,515 B     |
| `alphabet-block`              | 7,453              | 74                 | 0.00993x | 959,544 B     | 22,527 B     | 7,685              | 273                | 996,071 B     | 42,669 B     |
| `alphabet-simpleblock`        | 3,634              | 64                 | 0.0176x  | 515,181 B     | 14,469 B     | 3,859              | 262                | 551,614 B     | 34,607 B     |
| `alphabet-simple`             | 2,292              | 46                 | 0.0201x  | 214,925 B     | 9,609 B      | 2,532              | 244                | 251,583 B     | 29,737 B     |
| `alphabet-3d`                 | 11,460             | 174                | 0.0152x  | 2,073,194 B   | 43,317 B     | 11,634             | 372                | 2,108,784 B   | 63,437 B     |
| `alphabet-chrome`             | 4,041              | 33                 | 0.00817x | 210,545 B     | 10,157 B     | 4,288              | 231                | 247,315 B     | 30,285 B     |
| `alphabet-huge`               | 14,303             | 136                | 0.00951x | 2,613,243 B   | 39,496 B     | 14,503             | 335                | 2,649,255 B   | 59,636 B     |
| `alphabet-shade`              | 8,341              | 54                 | 0.00647x | 702,944 B     | 17,913 B     | 8,468              | 252                | 737,791 B     | 38,039 B     |
| `alphabet-slick`              | 6,603              | 60                 | 0.00909x | 771,311 B     | 18,528 B     | 6,842              | 259                | 807,950 B     | 38,670 B     |
| `alphabet-grid`               | 6,243              | 47                 | 0.00753x | 581,442 B     | 18,753 B     | 6,448              | 245                | 617,534 B     | 38,877 B     |
| `alphabet-pallet`             | 6,603              | 60                 | 0.00909x | 771,312 B     | 18,528 B     | 6,842              | 259                | 807,954 B     | 38,672 B     |
| `alphabet-tiny`               | 1,495              | 28                 | 0.0187x  | 137,706 B     | 8,465 B      | 1,744              | 226                | 174,502 B     | 28,589 B     |

<!-- perf:compare:allocations:end -->

### Memory, peak per render

<!-- perf:compare:memory:start -->
Ran 2026-10-11 on macos aarch64, Apple M1 Max, rustc 1.99.0 (b940084d7 2026-09-28)

| scenario                      | v3 api peak heap | v4 api peak heap | ratio   | v3 cli peak heap | v4 cli peak heap | v3 cli peak RSS | v4 cli peak RSS |
|-------------------------------|------------------|------------------|---------|------------------|------------------|-----------------|-----------------|
| `short-plain`                 | 46,221 B         | 4,825 B          | 0.104x  | 61,328 B         | 24,176 B         | 2,195,456 B     | 1,654,784 B     |
| `short-colors`                | 55,813 B         | 6,969 B          | 0.125x  | 71,344 B         | 26,400 B         | 2,375,680 B     | 1,654,784 B     |
| `short-gradient`              | 107,901 B        | 19,551 B         | 0.181x  | 125,488 B        | 38,976 B         | 2,605,056 B     | 1,687,552 B     |
| `short-transition`            | 107,815 B        | 19,554 B         | 0.181x  | 125,504 B        | 38,992 B         | 2,572,288 B     | 1,687,552 B     |
| `short-background`            | 46,448 B         | 4,844 B          | 0.104x  | 61,632 B         | 24,272 B         | 2,179,072 B     | 1,671,168 B     |
| `short-colors-background`     | 55,856 B         | 6,988 B          | 0.125x  | 71,408 B         | 26,528 B         | 2,342,912 B     | 1,671,168 B     |
| `short-gradient-background`   | 107,944 B        | 19,570 B         | 0.181x  | 125,488 B        | 39,104 B         | 2,621,440 B     | 1,720,320 B     |
| `short-transition-background` | 107,858 B        | 19,573 B         | 0.181x  | 125,504 B        | 39,120 B         | 2,588,672 B     | 1,720,320 B     |
| `long-plain`                  | 308,271 B        | 191,417 B        | 0.621x  | 341,760 B        | 217,648 B        | 3,145,728 B     | 1,998,848 B     |
| `long-colors`                 | 613,595 B        | 314,393 B        | 0.512x  | 662,368 B        | 348,896 B        | 3,801,088 B     | 2,048,000 B     |
| `long-gradient`               | 2,782,547 B      | 1,158,463 B      | 0.416x  | 2,833,216 B      | 1,184,768 B      | 6,340,608 B     | 2,572,288 B     |
| `long-transition`             | 2,781,459 B      | 1,158,466 B      | 0.416x  | 2,833,232 B      | 1,184,784 B      | 6,160,384 B     | 2,588,672 B     |
| `long-background`             | 320,090 B        | 240,588 B        | 0.752x  | 358,176 B        | 266,896 B        | 3,112,960 B     | 2,048,000 B     |
| `long-colors-background`      | 613,595 B        | 371,756 B        | 0.606x  | 662,368 B        | 398,176 B        | 3,981,312 B     | 2,113,536 B     |
| `long-gradient-background`    | 2,782,547 B      | 1,158,482 B      | 0.416x  | 2,833,216 B      | 1,184,896 B      | 6,144,000 B     | 2,605,056 B     |
| `long-transition-background`  | 2,781,459 B      | 1,158,485 B      | 0.417x  | 2,833,232 B      | 1,184,912 B      | 6,029,312 B     | 2,605,056 B     |
| `startup`                     | n/a              | n/a              | n/a     | 34,048 B         | 19,120 B         | 1,671,168 B     | 1,572,864 B     |
| `single-character`            | 43,639 B         | 1,080 B          | 0.0247x | 58,464 B         | 20,352 B         | 1,851,392 B     | 1,654,784 B     |
| `console-font`                | 24,888 B         | 1,280 B          | 0.0514x | 37,840 B         | 20,496 B         | 1,785,856 B     | 1,654,784 B     |
| `line-breaks`                 | 49,127 B         | 6,329 B          | 0.129x  | 64,832 B         | 25,968 B         | 2,195,456 B     | 1,654,784 B     |
| `long-center`                 | 313,050 B        | 191,417 B        | 0.611x  | 343,264 B        | 217,712 B        | 3,112,960 B     | 2,015,232 B     |
| `long-right`                  | 316,976 B        | 191,417 B        | 0.604x  | 344,576 B        | 217,712 B        | 3,080,192 B     | 2,015,232 B     |
| `long-spacing`                | 331,946 B        | 222,953 B        | 0.672x  | 381,360 B        | 257,264 B        | 3,244,032 B     | 2,097,152 B     |
| `long-independent-gradient`   | 2,747,475 B      | 1,158,463 B      | 0.422x  | 2,785,088 B      | 1,184,816 B      | 5,914,624 B     | 2,572,288 B     |
| `long-candy`                  | 613,939 B        | 314,921 B        | 0.513x  | 662,944 B        | 349,536 B        | 3,964,928 B     | 2,048,000 B     |
| `scaling-125`                 | 103,986 B        | 46,409 B         | 0.446x  | 124,064 B        | 67,632 B         | 2,605,056 B     | 1,785,856 B     |
| `scaling-250`                 | 167,123 B        | 92,025 B         | 0.551x  | 197,376 B        | 122,928 B        | 2,949,120 B     | 1,900,544 B     |
| `scaling-500`                 | 293,214 B        | 183,641 B        | 0.626x  | 331,632 B        | 209,328 B        | 3,063,808 B     | 1,982,464 B     |
| `scaling-1000`                | 546,384 B        | 367,113 B        | 0.672x  | 576,192 B        | 406,992 B        | 3,506,176 B     | 2,179,072 B     |
| `alphabet-console`            | 24,888 B         | 2,112 B          | 0.0849x | 37,856 B         | 21,392 B         | 1,818,624 B     | 1,654,784 B     |
| `alphabet-block`              | 54,549 B         | 12,233 B         | 0.224x  | 70,624 B         | 32,368 B         | 2,473,984 B     | 1,703,936 B     |
| `alphabet-simpleblock`        | 41,021 B         | 8,461 B          | 0.206x  | 56,352 B         | 28,144 B         | 2,179,072 B     | 1,671,168 B     |
| `alphabet-simple`             | 26,574 B         | 5,392 B          | 0.203x  | 41,088 B         | 24,960 B         | 2,080,768 B     | 1,654,784 B     |
| `alphabet-3d`                 | 81,432 B         | 24,893 B         | 0.306x  | 98,448 B         | 44,976 B         | 2,490,368 B     | 1,736,704 B     |
| `alphabet-chrome`             | 27,444 B         | 5,300 B          | 0.193x  | 40,976 B         | 24,640 B         | 2,080,768 B     | 1,654,784 B     |
| `alphabet-huge`               | 107,751 B        | 23,333 B         | 0.217x  | 127,664 B        | 44,352 B         | 2,605,056 B     | 1,720,320 B     |
| `alphabet-shade`              | 50,942 B         | 10,625 B         | 0.209x  | 68,208 B         | 30,096 B         | 2,310,144 B     | 1,654,784 B     |
| `alphabet-slick`              | 49,279 B         | 10,217 B         | 0.207x  | 65,136 B         | 30,672 B         | 2,293,760 B     | 1,687,552 B     |
| `alphabet-grid`               | 46,284 B         | 9,033 B          | 0.195x  | 62,000 B         | 28,752 B         | 2,342,912 B     | 1,654,784 B     |
| `alphabet-pallet`             | 49,280 B         | 10,217 B         | 0.207x  | 65,104 B         | 30,672 B         | 2,310,144 B     | 1,687,552 B     |
| `alphabet-tiny`               | 25,652 B         | 4,632 B          | 0.181x  | 38,800 B         | 23,936 B         | 1,998,848 B     | 1,654,784 B     |

<!-- perf:compare:memory:end -->
