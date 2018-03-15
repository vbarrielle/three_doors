A rust implementation of two strategies for the
[Monty Hall Problem](https://en.wikipedia.org/wiki/Monty_Hall_problem).
This can be used to show experimentally that the counterintuitive strategy
of switching doors wins with a 2/3 probability.

# Building and running

Simply clone the repository, and use cargo to compile and run:

```
$ git clone https://github.com/vbarrielle/three_doors.git
$ cd three_doors
$ cargo run --release
```

To configure the number of samples, simply pass it as a command line argument:

```bash
$ cargo run --release 100000
```
