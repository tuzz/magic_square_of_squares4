## Magic Square of Squares

An attempt to solve [open problem 1](http://www.multimagie.com/English/Problems.htm#SquaresOfSquares)
by searching for patterns 2 to 6 from [this paper](http://www.multimagie.com/Search.pdf#page2)
subject to these criteria:

1) All cells are 1 mod 24
2) All cells only contain prime factors that are 8k + 1, 5 or 7 (this criterion can be disabled)
3) The magic sum is 3 mod 72

The search is highly optimised to run quickly. It exhaustively checks ~335
billion magic sums per second on my AMD 7950x CPU. It uses SIMD, multithreading
and makes efficient use of data structures. The search periodically writes a
checkpoint to disk so that it can be resumed in case of power failure.

## Earlier attempts

See the following repositories for earlier experiments:

- [experiment1](https://github.com/tuzz/magic_square_of_squares_experiment1)
- [experiment2](https://github.com/tuzz/magic_square_of_squares_experiment2)
- [experiment3](https://github.com/tuzz/magic_square_of_squares_experiment3)
