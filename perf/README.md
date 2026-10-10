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

| target           | what it does                                                                                              | when to use it                           |
| ---------------- | --------------------------------------------------------------------------------------------------------- | ---------------------------------------- |
| `make perf`      | measures v4 and prints every number that changed against the results below beyond its noise               | as often as you like while you change v4 |
| `make perf-save` | measures v4 and writes the results below                                                                  | once you are happy with the change       |
| `make compare`   | installs the v3 binary into `perf/target/v3` once, measures both versions and writes the comparison below | to see v4 next to v3                     |

The results below are the baseline.
Change v4 and run `make perf` as often as you like, it reads the results and never writes this file.
Once you are happy, run `make perf-save` and commit this file with the change.

`make perf` reports every change of an allocation count, a byte count or a peak heap,
a median time beyond 10% on the API path and 60% on the CLI path, where process start follows the load of the machine,
and a peak resident set size beyond 4%, clear of the most unchanged code moves them between runs,
and prints how many scenarios it checked.
`CFONTS_PERF_SCENARIOS=short-plain,long-gradient make perf` checks only the scenarios it names,
`make perf-save` and `make compare` then write tables of only those.

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
Ran 2026-10-10 on macos aarch64, Apple M1 Max, rustc 1.99.0 (b940084d7 2026-09-28)

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
| `short-plain`                 | 4.48 µs       | 1.82 ms       |
| `short-colors`                | 5.93 µs       | 1.81 ms       |
| `short-gradient`              | 52.6 µs       | 1.86 ms       |
| `short-transition`            | 50.3 µs       | 1.88 ms       |
| `short-background`            | 4.67 µs       | 1.80 ms       |
| `short-colors-background`     | 6.14 µs       | 1.82 ms       |
| `short-gradient-background`   | 52.8 µs       | 1.86 ms       |
| `short-transition-background` | 50.2 µs       | 1.86 ms       |
| `long-plain`                  | 187 µs        | 2.29 ms       |
| `long-colors`                 | 237 µs        | 2.56 ms       |
| `long-gradient`               | 2.35 ms       | 5.24 ms       |
| `long-transition`             | 2.24 ms       | 4.63 ms       |
| `long-background`             | 188 µs        | 2.13 ms       |
| `long-colors-background`      | 241 µs        | 2.19 ms       |
| `long-gradient-background`    | 2.29 ms       | 4.70 ms       |
| `long-transition-background`  | 2.22 ms       | 4.66 ms       |
| `startup`                     | n/a           | 2.26 ms       |
| `single-character`            | 913 ns        | 2.03 ms       |
| `console-font`                | 652 ns        | 1.95 ms       |
| `line-breaks`                 | 6.28 µs       | 2.02 ms       |
| `long-center`                 | 188 µs        | 2.10 ms       |
| `long-right`                  | 187 µs        | 2.08 ms       |
| `long-spacing`                | 265 µs        | 2.25 ms       |
| `long-independent-gradient`   | 2.61 ms       | 4.88 ms       |
| `long-candy`                  | 240 µs        | 2.67 ms       |
| `scaling-125`                 | 42.7 µs       | 2.17 ms       |
| `scaling-250`                 | 84.8 µs       | 1.93 ms       |
| `scaling-500`                 | 166 µs        | 2.14 ms       |
| `scaling-1000`                | 334 µs        | 2.68 ms       |
| `alphabet-console`            | 1.17 µs       | 2.15 ms       |
| `alphabet-block`              | 10.2 µs       | 1.82 ms       |
| `alphabet-simpleblock`        | 6.10 µs       | 2.10 ms       |
| `alphabet-simple`             | 5.01 µs       | 2.16 ms       |
| `alphabet-3d`                 | 16.9 µs       | 1.82 ms       |
| `alphabet-chrome`             | 3.83 µs       | 2.06 ms       |
| `alphabet-huge`               | 20.5 µs       | 2.06 ms       |
| `alphabet-shade`              | 9.72 µs       | 1.80 ms       |
| `alphabet-slick`              | 8.27 µs       | 1.80 ms       |
| `alphabet-grid`               | 6.82 µs       | 1.99 ms       |
| `alphabet-pallet`             | 8.21 µs       | 1.81 ms       |
| `alphabet-tiny`               | 2.81 µs       | 1.78 ms       |

<!-- perf:results:speed:end -->

### Allocations, one render

<!-- perf:results:allocations:start -->
Ran 2026-10-10 on macos aarch64, Apple M1 Max, rustc 1.99.0 (b940084d7 2026-09-28)

| scenario                      | v4 api allocations | v4 api bytes | v4 cli allocations | v4 cli bytes |
|-------------------------------|--------------------|--------------|--------------------|--------------|
| `short-plain`                 | 45                 | 12,697 B     | 243                | 32,809 B     |
| `short-colors`                | 47                 | 16,889 B     | 254                | 37,193 B     |
| `short-gradient`              | 631                | 55,698 B     | 838                | 76,002 B     |
| `short-transition`            | 631                | 55,701 B     | 842                | 76,059 B     |
| `short-background`            | 45                 | 12,709 B     | 250                | 32,945 B     |
| `short-colors-background`     | 47                 | 16,901 B     | 261                | 37,353 B     |
| `short-gradient-background`   | 631                | 53,663 B     | 844                | 74,099 B     |
| `short-transition-background` | 631                | 53,666 B     | 848                | 74,156 B     |
| `long-plain`                  | 796                | 434,357 B    | 995                | 455,563 B    |
| `long-colors`                 | 798                | 680,208 B    | 1,006              | 701,606 B    |
| `long-gradient`               | 26,901             | 3,021,386 B  | 27,108             | 3,042,768 B  |
| `long-transition`             | 26,901             | 3,021,389 B  | 27,112             | 3,042,825 B  |
| `long-background`             | 797                | 532,677 B    | 1,002              | 553,991 B    |
| `long-colors-background`      | 799                | 794,917 B    | 1,013              | 816,447 B    |
| `long-gradient-background`    | 26,901             | 2,890,327 B  | 27,114             | 2,911,841 B  |
| `long-transition-background`  | 26,901             | 2,890,330 B  | 27,118             | 2,911,898 B  |
| `startup`                     | n/a                | n/a          | 190                | 19,953 B     |
| `single-character`            | 24                 | 1,815 B      | 223                | 21,923 B     |
| `console-font`                | 13                 | 2,153 B      | 211                | 22,269 B     |
| `line-breaks`                 | 58                 | 14,777 B     | 256                | 34,901 B     |
| `long-center`                 | 796                | 434,357 B    | 998                | 455,649 B    |
| `long-right`                  | 796                | 434,357 B    | 998                | 455,647 B    |
| `long-spacing`                | 968                | 676,181 B    | 1,171              | 697,561 B    |
| `long-independent-gradient`   | 26,901             | 3,021,386 B  | 27,109             | 3,042,818 B  |
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
| `short-colors`                | 9,729 B          | 29,488 B         | 2,080,768 B     |
| `short-gradient`              | 23,355 B         | 46,192 B         | 2,113,536 B     |
| `short-transition`            | 23,358 B         | 46,208 B         | 2,129,920 B     |
| `short-background`            | 7,605 B          | 27,360 B         | 2,080,768 B     |
| `short-colors-background`     | 9,749 B          | 29,616 B         | 2,080,768 B     |
| `short-gradient-background`   | 22,351 B         | 42,224 B         | 2,113,536 B     |
| `short-transition-background` | 22,354 B         | 42,240 B         | 2,129,920 B     |
| `long-plain`                  | 319,937 B        | 354,368 B        | 2,539,520 B     |
| `long-colors`                 | 442,913 B        | 485,616 B        | 2,621,440 B     |
| `long-gradient`               | 1,352,539 B      | 1,387,056 B      | 3,145,728 B     |
| `long-transition`             | 1,352,542 B      | 1,387,072 B      | 3,162,112 B     |
| `long-background`             | 369,109 B        | 403,616 B        | 2,572,288 B     |
| `long-colors-background`      | 500,277 B        | 534,896 B        | 2,654,208 B     |
| `long-gradient-background`    | 1,287,023 B      | 1,321,648 B      | 3,145,728 B     |
| `long-transition-background`  | 1,287,026 B      | 1,321,664 B      | 3,145,728 B     |
| `startup`                     | n/a              | 19,136 B         | 1,998,848 B     |
| `single-character`            | 1,440 B          | 20,656 B         | 2,064,384 B     |
| `console-font`                | 1,552 B          | 20,800 B         | 2,080,768 B     |
| `line-breaks`                 | 10,049 B         | 29,632 B         | 2,064,384 B     |
| `long-center`                 | 319,937 B        | 354,432 B        | 2,539,520 B     |
| `long-right`                  | 319,937 B        | 354,432 B        | 2,539,520 B     |
| `long-spacing`                | 527,393 B        | 585,792 B        | 2,818,048 B     |
| `long-independent-gradient`   | 1,352,539 B      | 1,387,104 B      | 3,112,960 B     |
| `long-candy`                  | 443,441 B        | 486,256 B        | 2,621,440 B     |
| `scaling-125`                 | 76,289 B         | 98,368 B         | 2,195,456 B     |
| `scaling-250`                 | 150,465 B        | 184,000 B        | 2,342,912 B     |
| `scaling-500`                 | 300,161 B        | 332,608 B        | 2,523,136 B     |
| `scaling-1000`                | 600,033 B        | 657,184 B        | 2,834,432 B     |
| `alphabet-console`            | 2,672 B          | 21,920 B         | 2,080,768 B     |
| `alphabet-block`              | 18,593 B         | 39,008 B         | 2,097,152 B     |
| `alphabet-simpleblock`        | 13,761 B         | 33,280 B         | 2,080,768 B     |
| `alphabet-simple`             | 9,632 B          | 29,136 B         | 2,080,768 B     |
| `alphabet-3d`                 | 34,433 B         | 54,208 B         | 2,146,304 B     |
| `alphabet-chrome`             | 8,480 B          | 27,824 B         | 2,080,768 B     |
| `alphabet-huge`               | 34,993 B         | 55,632 B         | 2,146,304 B     |
| `alphabet-shade`              | 19,105 B         | 39,328 B         | 2,097,152 B     |
| `alphabet-slick`              | 16,577 B         | 37,120 B         | 2,097,152 B     |
| `alphabet-grid`               | 15,393 B         | 36,064 B         | 2,097,152 B     |
| `alphabet-pallet`             | 16,577 B         | 37,120 B         | 2,097,152 B     |
| `alphabet-tiny`               | 6,752 B          | 26,064 B         | 2,064,384 B     |

<!-- perf:results:memory:end -->

## Compared with v3

`make compare` writes this section.
A ratio is v4 over v3, `0.25x` means v4 needs a quarter of what v3 needs.

### Speed, median time per render

<!-- perf:compare:speed:start -->
Ran 2026-10-10 on macos aarch64, Apple M1 Max, rustc 1.99.0 (b940084d7 2026-09-28)

| scenario                      | v3 api median | v4 api median | api ratio | v3 cli median | v4 cli median | cli ratio |
|-------------------------------|---------------|---------------|-----------|---------------|---------------|-----------|
| `short-plain`                 | 691 µs        | 4.44 µs       | 0.00642x  | 2.55 ms       | 1.79 ms       | 0.704x    |
| `short-colors`                | 948 µs        | 6.04 µs       | 0.00637x  | 3.12 ms       | 2.12 ms       | 0.679x    |
| `short-gradient`              | 1.66 ms       | 53.9 µs       | 0.0325x   | 4.22 ms       | 2.25 ms       | 0.533x    |
| `short-transition`            | 1.39 ms       | 50.5 µs       | 0.0362x   | 3.39 ms       | 1.86 ms       | 0.549x    |
| `short-background`            | 707 µs        | 4.69 µs       | 0.00663x  | 2.53 ms       | 1.79 ms       | 0.708x    |
| `short-colors-background`     | 941 µs        | 6.10 µs       | 0.00648x  | 2.83 ms       | 1.79 ms       | 0.632x    |
| `short-gradient-background`   | 1.65 ms       | 55.2 µs       | 0.0335x   | 4.37 ms       | 2.40 ms       | 0.550x    |
| `short-transition-background` | 1.40 ms       | 50.6 µs       | 0.0362x   | 4.20 ms       | 3.00 ms       | 0.716x    |
| `long-plain`                  | 881 ms        | 187 µs        | 0.000212x | 883 ms        | 2.15 ms       | 0.00243x  |
| `long-colors`                 | 1.27 s        | 236 µs        | 0.000186x | 1.27 s        | 2.16 ms       | 0.00171x  |
| `long-gradient`               | 928 ms        | 2.35 ms       | 0.00253x  | 928 ms        | 5.49 ms       | 0.00592x  |
| `long-transition`             | 913 ms        | 2.20 ms       | 0.00241x  | 915 ms        | 4.60 ms       | 0.00503x  |
| `long-background`             | 881 ms        | 188 µs        | 0.000213x | 871 ms        | 2.09 ms       | 0.00240x  |
| `long-colors-background`      | 1.26 s        | 240 µs        | 0.000191x | 1.26 s        | 2.24 ms       | 0.00179x  |
| `long-gradient-background`    | 912 ms        | 2.29 ms       | 0.00252x  | 909 ms        | 4.61 ms       | 0.00507x  |
| `long-transition-background`  | 904 ms        | 2.20 ms       | 0.00244x  | 905 ms        | 4.52 ms       | 0.00499x  |
| `startup`                     | n/a           | n/a           | n/a       | 1.66 ms       | 1.80 ms       | 1.08x     |
| `single-character`            | 68.0 µs       | 915 ns        | 0.0135x   | 1.85 ms       | 1.81 ms       | 0.982x    |
| `console-font`                | 24.2 µs       | 651 ns        | 0.0269x   | 1.74 ms       | 1.79 ms       | 1.03x     |
| `line-breaks`                 | 1.23 ms       | 6.26 µs       | 0.00510x  | 3.13 ms       | 1.80 ms       | 0.575x    |
| `long-center`                 | 879 ms        | 186 µs        | 0.000212x | 884 ms        | 2.18 ms       | 0.00247x  |
| `long-right`                  | 882 ms        | 187 µs        | 0.000212x | 878 ms        | 2.23 ms       | 0.00254x  |
| `long-spacing`                | 929 ms        | 264 µs        | 0.000284x | 927 ms        | 2.32 ms       | 0.00251x  |
| `long-independent-gradient`   | 912 ms        | 2.61 ms       | 0.00286x  | 911 ms        | 4.99 ms       | 0.00548x  |
| `long-candy`                  | 1.26 s        | 239 µs        | 0.000190x | 1.26 s        | 2.20 ms       | 0.00175x  |
| `scaling-125`                 | 49.0 ms       | 42.7 µs       | 0.000873x | 51.0 ms       | 1.90 ms       | 0.0372x   |
| `scaling-250`                 | 186 ms        | 84.9 µs       | 0.000456x | 191 ms        | 1.94 ms       | 0.0102x   |
| `scaling-500`                 | 731 ms        | 167 µs        | 0.000229x | 723 ms        | 2.15 ms       | 0.00297x  |
| `scaling-1000`                | 2.85 s        | 332 µs        | 0.000117x | 2.84 s        | 2.25 ms       | 0.000792x |
| `alphabet-console`            | 41.6 µs       | 1.17 µs       | 0.0280x   | 1.98 ms       | 1.80 ms       | 0.910x    |
| `alphabet-block`              | 3.24 ms       | 10.2 µs       | 0.00314x  | 5.45 ms       | 1.85 ms       | 0.339x    |
| `alphabet-simpleblock`        | 294 µs        | 5.96 µs       | 0.0203x   | 2.11 ms       | 1.78 ms       | 0.841x    |
| `alphabet-simple`             | 181 µs        | 4.98 µs       | 0.0275x   | 2.00 ms       | 1.77 ms       | 0.886x    |
| `alphabet-3d`                 | 2.40 ms       | 16.7 µs       | 0.00697x  | 4.46 ms       | 1.87 ms       | 0.418x    |
| `alphabet-chrome`             | 641 µs        | 3.91 µs       | 0.00610x  | 2.53 ms       | 1.81 ms       | 0.715x    |
| `alphabet-huge`               | 7.28 ms       | 20.5 µs       | 0.00281x  | 9.72 ms       | 1.98 ms       | 0.204x    |
| `alphabet-shade`              | 2.46 ms       | 9.76 µs       | 0.00397x  | 4.51 ms       | 1.81 ms       | 0.402x    |
| `alphabet-slick`              | 2.66 ms       | 8.55 µs       | 0.00322x  | 5.19 ms       | 1.88 ms       | 0.363x    |
| `alphabet-grid`               | 2.00 ms       | 7.04 µs       | 0.00353x  | 4.35 ms       | 2.04 ms       | 0.469x    |
| `alphabet-pallet`             | 2.68 ms       | 8.24 µs       | 0.00307x  | 4.51 ms       | 1.80 ms       | 0.399x    |
| `alphabet-tiny`               | 408 µs        | 2.80 µs       | 0.00687x  | 2.37 ms       | 1.82 ms       | 0.769x    |

<!-- perf:compare:speed:end -->

### Allocations, one render

<!-- perf:compare:allocations:start -->
Ran 2026-10-10 on macos aarch64, Apple M1 Max, rustc 1.99.0 (b940084d7 2026-09-28)

| scenario                      | v3 api allocations | v4 api allocations | ratio    | v3 api bytes  | v4 api bytes | v3 cli allocations | v4 cli allocations | v3 cli bytes  | v4 cli bytes |
|-------------------------------|--------------------|--------------------|----------|---------------|--------------|--------------------|--------------------|---------------|--------------|
| `short-plain`                 | 3,494              | 45                 | 0.0129x  | 253,868 B     | 12,697 B     | 3,741              | 243                | 290,590 B     | 32,809 B     |
| `short-colors`                | 4,267              | 47                 | 0.0110x  | 586,796 B     | 16,889 B     | 4,525              | 254                | 623,834 B     | 37,193 B     |
| `short-gradient`              | 14,774             | 631                | 0.0427x  | 944,573 B     | 55,698 B     | 14,874             | 838                | 980,392 B     | 76,002 B     |
| `short-transition`            | 12,643             | 631                | 0.0499x  | 808,046 B     | 55,701 B     | 12,753             | 842                | 844,356 B     | 76,059 B     |
| `short-background`            | 3,504              | 45                 | 0.0128x  | 254,654 B     | 12,709 B     | 3,760              | 250                | 291,767 B     | 32,945 B     |
| `short-colors-background`     | 4,277              | 47                 | 0.0110x  | 587,497 B     | 16,901 B     | 4,545              | 261                | 625,230 B     | 37,353 B     |
| `short-gradient-background`   | 14,785             | 631                | 0.0427x  | 948,901 B     | 53,663 B     | 14,895             | 844                | 985,415 B     | 74,099 B     |
| `short-transition-background` | 12,654             | 631                | 0.0499x  | 812,345 B     | 53,666 B     | 12,773             | 848                | 849,046 B     | 74,156 B     |
| `long-plain`                  | 143,510            | 796                | 0.00555x | 257,913,870 B | 434,357 B    | 143,287            | 995                | 257,946,559 B | 455,563 B    |
| `long-colors`                 | 180,956            | 798                | 0.00441x | 789,917,703 B | 680,208 B    | 181,021            | 1,006              | 789,955,140 B | 701,606 B    |
| `long-gradient`               | 596,026            | 26,901             | 0.0451x  | 285,085,688 B | 3,021,386 B  | 591,153            | 27,108             | 285,081,450 B | 3,042,768 B  |
| `long-transition`             | 534,490            | 26,901             | 0.0503x  | 281,308,607 B | 3,021,389 B  | 529,797            | 27,112             | 281,305,924 B | 3,042,825 B  |
| `long-background`             | 143,520            | 797                | 0.00555x | 257,937,981 B | 532,677 B    | 143,306            | 1,002              | 257,971,061 B | 553,991 B    |
| `long-colors-background`      | 180,966            | 799                | 0.00442x | 789,918,615 B | 794,917 B    | 181,040            | 1,013              | 789,956,443 B | 816,447 B    |
| `long-gradient-background`    | 596,036            | 26,901             | 0.0451x  | 285,090,812 B | 2,890,327 B  | 591,172            | 27,114             | 285,086,965 B | 2,911,841 B  |
| `long-transition-background`  | 534,500            | 26,901             | 0.0503x  | 281,313,752 B | 2,890,330 B  | 529,816            | 27,118             | 281,311,460 B | 2,911,898 B  |
| `startup`                     | n/a                | n/a                | n/a      | n/a           | n/a          | 249                | 190                | 36,738 B      | 19,953 B     |
| `single-character`            | 1,065              | 24                 | 0.0225x  | 59,060 B      | 1,815 B      | 1,318              | 223                | 95,855 B      | 21,923 B     |
| `console-font`                | 496                | 13                 | 0.0262x  | 30,529 B      | 2,153 B      | 747                | 211                | 67,321 B      | 22,269 B     |
| `line-breaks`                 | 4,694              | 58                 | 0.0124x  | 419,180 B     | 14,777 B     | 4,935              | 256                | 455,824 B     | 34,901 B     |
| `long-center`                 | 143,526            | 796                | 0.00555x | 261,060,329 B | 434,357 B    | 143,307            | 998                | 261,093,216 B | 455,649 B    |
| `long-right`                  | 143,499            | 796                | 0.00555x | 263,036,111 B | 434,357 B    | 143,278            | 998                | 263,068,963 B | 455,647 B    |
| `long-spacing`                | 145,805            | 968                | 0.00664x | 299,703,033 B | 676,181 B    | 145,618            | 1,171              | 299,736,566 B | 697,561 B    |
| `long-independent-gradient`   | 585,488            | 26,901             | 0.0459x  | 284,810,803 B | 3,021,386 B  | 580,909            | 27,109             | 284,809,001 B | 3,042,818 B  |
| `long-candy`                  | 181,053            | 799                | 0.00441x | 789,939,043 B | 680,736 B    | 181,121            | 1,007              | 789,977,025 B | 702,143 B    |
| `scaling-125`                 | 32,138             | 194                | 0.00604x | 14,440,237 B  | 106,357 B    | 32,274             | 393                | 14,475,813 B  | 126,713 B    |
| `scaling-250`                 | 64,435             | 378                | 0.00587x | 53,816,426 B  | 208,565 B    | 64,458             | 577                | 53,851,251 B  | 229,171 B    |
| `scaling-500`                 | 130,055            | 718                | 0.00552x | 209,185,537 B | 414,197 B    | 129,876            | 917                | 209,218,630 B | 435,303 B    |
| `scaling-1000`                | 264,502            | 1,409              | 0.00533x | 828,885,217 B | 826,005 B    | 263,967            | 1,608              | 828,915,614 B | 848,111 B    |
| `alphabet-console`            | 887                | 15                 | 0.0169x  | 44,031 B      | 3,945 B      | 1,139              | 213                | 80,884 B      | 24,091 B     |
| `alphabet-block`              | 7,453              | 74                 | 0.00993x | 959,544 B     | 28,887 B     | 7,685              | 273                | 996,071 B     | 49,045 B     |
| `alphabet-simpleblock`        | 3,634              | 64                 | 0.0176x  | 515,181 B     | 19,769 B     | 3,859              | 262                | 551,614 B     | 39,923 B     |
| `alphabet-simple`             | 2,292              | 46                 | 0.0201x  | 214,925 B     | 13,849 B     | 2,532              | 244                | 251,583 B     | 33,993 B     |
| `alphabet-3d`                 | 11,460             | 174                | 0.0152x  | 2,073,194 B   | 52,857 B     | 11,634             | 372                | 2,108,784 B   | 72,993 B     |
| `alphabet-chrome`             | 4,041              | 33                 | 0.00817x | 210,545 B     | 13,337 B     | 4,288              | 231                | 247,315 B     | 33,481 B     |
| `alphabet-huge`               | 14,303             | 136                | 0.00951x | 2,613,243 B   | 51,156 B     | 14,503             | 335                | 2,649,255 B   | 71,312 B     |
| `alphabet-shade`              | 8,341              | 54                 | 0.00647x | 702,944 B     | 26,393 B     | 8,468              | 252                | 737,791 B     | 46,535 B     |
| `alphabet-slick`              | 6,603              | 60                 | 0.00909x | 771,311 B     | 24,888 B     | 6,842              | 259                | 807,950 B     | 45,046 B     |
| `alphabet-grid`               | 6,243              | 47                 | 0.00753x | 581,442 B     | 25,113 B     | 6,448              | 245                | 617,534 B     | 45,253 B     |
| `alphabet-pallet`             | 6,603              | 60                 | 0.00909x | 771,312 B     | 24,888 B     | 6,842              | 259                | 807,954 B     | 45,048 B     |
| `alphabet-tiny`               | 1,495              | 28                 | 0.0187x  | 137,706 B     | 10,585 B     | 1,744              | 226                | 174,502 B     | 30,725 B     |

<!-- perf:compare:allocations:end -->

### Memory, peak per render

<!-- perf:compare:memory:start -->
Ran 2026-10-10 on macos aarch64, Apple M1 Max, rustc 1.99.0 (b940084d7 2026-09-28)

| scenario                      | v3 api peak heap | v4 api peak heap | ratio   | v3 cli peak heap | v4 cli peak heap | v3 cli peak RSS | v4 cli peak RSS |
|-------------------------------|------------------|------------------|---------|------------------|------------------|-----------------|-----------------|
| `short-plain`                 | 46,221 B         | 7,585 B          | 0.164x  | 61,328 B         | 27,264 B         | 2,195,456 B     | 2,064,384 B     |
| `short-colors`                | 55,813 B         | 9,729 B          | 0.174x  | 71,344 B         | 29,488 B         | 2,375,680 B     | 2,080,768 B     |
| `short-gradient`              | 107,901 B        | 23,355 B         | 0.216x  | 125,488 B        | 46,192 B         | 2,637,824 B     | 2,129,920 B     |
| `short-transition`            | 107,815 B        | 23,358 B         | 0.217x  | 125,504 B        | 46,208 B         | 2,539,520 B     | 2,129,920 B     |
| `short-background`            | 46,448 B         | 7,605 B          | 0.164x  | 61,632 B         | 27,360 B         | 2,179,072 B     | 2,080,768 B     |
| `short-colors-background`     | 55,856 B         | 9,749 B          | 0.175x  | 71,408 B         | 29,616 B         | 2,310,144 B     | 2,080,768 B     |
| `short-gradient-background`   | 107,944 B        | 22,351 B         | 0.207x  | 125,488 B        | 42,224 B         | 2,572,288 B     | 2,129,920 B     |
| `short-transition-background` | 107,858 B        | 22,354 B         | 0.207x  | 125,504 B        | 42,240 B         | 2,539,520 B     | 2,129,920 B     |
| `long-plain`                  | 308,271 B        | 319,937 B        | 1.04x   | 341,760 B        | 354,368 B        | 3,112,960 B     | 2,539,520 B     |
| `long-colors`                 | 613,595 B        | 442,913 B        | 0.722x  | 662,368 B        | 485,616 B        | 3,948,544 B     | 2,621,440 B     |
| `long-gradient`               | 2,782,547 B      | 1,352,539 B      | 0.486x  | 2,833,216 B      | 1,387,056 B      | 6,012,928 B     | 3,145,728 B     |
| `long-transition`             | 2,781,459 B      | 1,352,542 B      | 0.486x  | 2,833,232 B      | 1,387,072 B      | 6,160,384 B     | 3,145,728 B     |
| `long-background`             | 320,090 B        | 369,109 B        | 1.15x   | 358,176 B        | 403,616 B        | 3,227,648 B     | 2,572,288 B     |
| `long-colors-background`      | 613,595 B        | 500,277 B        | 0.815x  | 662,368 B        | 534,896 B        | 3,981,312 B     | 2,654,208 B     |
| `long-gradient-background`    | 2,782,547 B      | 1,287,023 B      | 0.463x  | 2,833,216 B      | 1,321,648 B      | 5,963,776 B     | 3,145,728 B     |
| `long-transition-background`  | 2,781,459 B      | 1,287,026 B      | 0.463x  | 2,833,232 B      | 1,321,664 B      | 6,144,000 B     | 3,178,496 B     |
| `startup`                     | n/a              | n/a              | n/a     | 34,048 B         | 19,136 B         | 1,671,168 B     | 1,998,848 B     |
| `single-character`            | 43,639 B         | 1,440 B          | 0.0330x | 58,464 B         | 20,656 B         | 1,851,392 B     | 2,064,384 B     |
| `console-font`                | 24,888 B         | 1,552 B          | 0.0624x | 37,840 B         | 20,800 B         | 1,785,856 B     | 2,080,768 B     |
| `line-breaks`                 | 49,127 B         | 10,049 B         | 0.205x  | 64,832 B         | 29,632 B         | 2,211,840 B     | 2,064,384 B     |
| `long-center`                 | 313,050 B        | 319,937 B        | 1.02x   | 343,264 B        | 354,432 B        | 3,211,264 B     | 2,555,904 B     |
| `long-right`                  | 316,976 B        | 319,937 B        | 1.01x   | 344,576 B        | 354,432 B        | 3,178,496 B     | 2,539,520 B     |
| `long-spacing`                | 331,946 B        | 527,393 B        | 1.59x   | 381,360 B        | 585,792 B        | 3,342,336 B     | 2,818,048 B     |
| `long-independent-gradient`   | 2,747,475 B      | 1,352,539 B      | 0.492x  | 2,785,088 B      | 1,387,104 B      | 5,881,856 B     | 3,129,344 B     |
| `long-candy`                  | 613,939 B        | 443,441 B        | 0.722x  | 662,944 B        | 486,256 B        | 3,817,472 B     | 2,621,440 B     |
| `scaling-125`                 | 103,986 B        | 76,289 B         | 0.734x  | 124,064 B        | 98,368 B         | 2,523,136 B     | 2,195,456 B     |
| `scaling-250`                 | 167,123 B        | 150,465 B        | 0.900x  | 197,376 B        | 184,000 B        | 2,850,816 B     | 2,326,528 B     |
| `scaling-500`                 | 293,214 B        | 300,161 B        | 1.02x   | 331,632 B        | 332,608 B        | 3,063,808 B     | 2,523,136 B     |
| `scaling-1000`                | 546,384 B        | 600,033 B        | 1.10x   | 576,192 B        | 657,184 B        | 3,506,176 B     | 2,834,432 B     |
| `alphabet-console`            | 24,888 B         | 2,672 B          | 0.107x  | 37,856 B         | 21,920 B         | 1,851,392 B     | 2,080,768 B     |
| `alphabet-block`              | 54,549 B         | 18,593 B         | 0.341x  | 70,624 B         | 39,008 B         | 2,457,600 B     | 2,097,152 B     |
| `alphabet-simpleblock`        | 41,021 B         | 13,761 B         | 0.335x  | 56,352 B         | 33,280 B         | 2,195,456 B     | 2,080,768 B     |
| `alphabet-simple`             | 26,574 B         | 9,632 B          | 0.362x  | 41,088 B         | 29,136 B         | 2,080,768 B     | 2,080,768 B     |
| `alphabet-3d`                 | 81,432 B         | 34,433 B         | 0.423x  | 98,448 B         | 54,208 B         | 2,506,752 B     | 2,146,304 B     |
| `alphabet-chrome`             | 27,444 B         | 8,480 B          | 0.309x  | 40,976 B         | 27,824 B         | 2,097,152 B     | 2,080,768 B     |
| `alphabet-huge`               | 107,751 B        | 34,993 B         | 0.325x  | 127,664 B        | 55,632 B         | 2,686,976 B     | 2,146,304 B     |
| `alphabet-shade`              | 50,942 B         | 19,105 B         | 0.375x  | 68,208 B         | 39,328 B         | 2,342,912 B     | 2,080,768 B     |
| `alphabet-slick`              | 49,279 B         | 16,577 B         | 0.336x  | 65,136 B         | 37,120 B         | 2,310,144 B     | 2,097,152 B     |
| `alphabet-grid`               | 46,284 B         | 15,393 B         | 0.333x  | 62,000 B         | 36,064 B         | 2,326,528 B     | 2,097,152 B     |
| `alphabet-pallet`             | 49,280 B         | 16,577 B         | 0.336x  | 65,104 B         | 37,120 B         | 2,326,528 B     | 2,097,152 B     |
| `alphabet-tiny`               | 25,652 B         | 6,752 B          | 0.263x  | 38,800 B         | 26,064 B         | 1,998,848 B     | 2,064,384 B     |

<!-- perf:compare:memory:end -->
