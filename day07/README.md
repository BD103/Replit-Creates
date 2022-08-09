# Day 7

> "Tell someone their fortune"

This was an interesting prompt, though I feel it is a bit overused in "learning to code" scenarios. That's beyond the point, though. This is my epic cool fortune teller tent. While not as impressive as my other projects, it has a few cool features. Before you continue reading, I recommend you run the repl first. (Spoilers ahead!)

## What is the 50%?

To be honest, I have no idea. I just wanted to make something funny, and slightly creepy. It ended up reminding me of the beginning of Deltarune, where your information is "DISCARDED."

In reality, here's how my program works:

1. It figures out the username of the person running the project, or generated a random one instead. (You can find the username through the `REPL_OWNER` environment variable.)
2. Appends the current date to the username, so the output changes each day.
3. Applies an MD5 hash on the username, so it is now a set length of bytes.
4. Converts the bytes to numbers, then finds the sum of said numbers.

What the above steps do is give me a random number that changes daily for each person that plays the project. Using this, I figure out whether that number is even or odd. If even, then the person is "IN THE 50%."

While maybe not magical, it was fun to mess around with bytes, random chance, and scrolling text. Until next time,

~BD103
