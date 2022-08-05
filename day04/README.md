# Day 4

> "Use only emojis"

Welcome to EmoScript, an esoteric scripting language that only uses emojis (and the occasional number). This was very fun to make. I've never implemented a programming language before, even though I tried with BrainF. Since this is a _larger_ project, I'm going to go more over how to use the language, and less of how I made it.

## Programming in EmoScript

EmoScript is cool because it is _almost_ Turing Complete. It does loops and operations, but it only lets you use 6 variables. This can be a restriction at times, but it's not too bad to work around.

### Running `.emo` files

Scripts are stored in `.emo` files. To run a file, pass it to the main executable. If you are working with the source code, add the filename to the end of `cargo run`.

```shell
$ ./emoscript main.emo
# Or with source code
$ cargo run main.emo
```

> **Hint:** EmoScript by default tries to run `main.emo`. You don't need to specify the file unless it is named something else.

### Variables

All variables are represented as different colored hearts. Variables can only be numbers that go up to 255. (They are unsigned 8-bit integers.) To assign a variable, write the following:

```
❤️ 👈 2
```

You have now set the red heart variable to the value of 2. How do we know it's 2? Let's try printing it:

```
🎙 ❤️
# 2
```

The current allowed variables are `❤️`, `🧡`, `💛`, `💚`, `💙`, and `💜`. Because of implementation issues, you can only assign a variable to 0 through 9. You can still use operators to go up to 255, though!

> **Hint:** Indents, empty lines, and lines that start with a hashtag (`#`) are ignored. You can use the `#` to create a comment.

### Operators

All operations are applied to the given variable. You can add, subtract, multiply, and divide in EmoScript. To apply an operation, specify the variable, operation, and value.

```
💜 👈 3
💜 ➕ 5
🎙 💜
# 8
```

The current operators are `➕`, `➖`, `✖️`, and `➗`. For now you cannot operate on two variables at the same time.

```
# ⚠️ Invalid Code
💛 👈 5
💚 👈 9

# Error!
💚 ➖ 💛
```

### Printing and speaking

You can print the number value of a variable with the `🎙` emoji.

```
🎙 💙
```

You can also convert the number to a letter with the `📢` emoji.

```
💙 👈 1
📢 💙
# a
```

The rule for number to letter conversion is simple. 0 is space, 1 is "a", 2 is "b", "z" is 27, etc. All other numbers convert to space. For a full reference, view [`src/speak_map.rs`](src/speak_map.rs).

### Looping

Looping is where this program truly becomes powerful-ish. Loops allow you to repeat an action a specified amount of time, or decrement a variable until it is 0. To specify the start of a loop, use a `🗿` emoji with a counter.

```
# Prints 4 times, because programmers count from 0 :P
🗿 3
    📢 2
🗿
```

You can also loop through a variable. This variable can still be edited within the loop, allowing for infinite loops and more.

```
# Prints 3, 2, 1, 0
❤️ 👈 3
🗿 ❤️
    🎙 ❤️
🗿
```

```
# Infinite loop example
🧡 👈 0
🗿 🧡
    # Adds 1 back to 🧡, preventing it from ever reaching 0
    🧡 ➕ 1
🗿
```

I originally wrote looping to work nested, but it is completely untested and I have no idea whether it works or not! Good luck with that. :)

### Stopping a program early

If you want to stop a program, you can use the stop sign emoji (`🛑`).

```
💛 👈 8

🛑

# This will never run, as stop was already called
🎙 💛
```

### More examples

I have written a few examples that you are free to check out.

- `simple.emo`: Shows how to store, operate, and print variables.
- `hello_world.emo`: Shows how to speak letters, work around number input restriction, and the work around 6 variable restriction.
- `test.emo`: Used for testing all aspects of the language. Can be seen as a reference to everything you can do with EmoScript.

## Final thoughts

This was really fun to make, even if I stayed up entirely too long to make it. I hope you guys have fun writing in EmoScript!

~BD103
