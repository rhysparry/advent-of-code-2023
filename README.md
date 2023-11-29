# Advent of Code 2023

These are my solutions for the 2023 Advent of Code. I'm using this as an opportunity to improve my Rust skills.

## Inputs

Each day's input is stored in the `inputs` directory, using the format `day-<day>.txt`.

## Running

If `just` is installed on your system, you can run the solutions for a given day using the `just` command:

```bash
$ just run 1
```

This will automatically use the input for the day, and print the solution to stdout.

You can also run the solution directly by using `cargo run`:

```bash
$ cargo run -- 1
```

Using this method you can supply an alternative input (or if not provided, the solution will read from stdin).

## Testing

You can run the tests using `just`

```bash
$ just test
```