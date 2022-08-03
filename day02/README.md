# Day 2

> "Create something that ages"

Today I was stumped for ideas. Normally with this kind of prompt, I'd make a tree simulator or something. The problem is that I wasn't sure how to make said tree simulator. Instead I decided to make this noise visualization program. This includes Perlin noise, Simplex noise, fractal Brownian motion, and more. The cool thing about noise is that it's meant to look like natural randomness. It looks more like a pattern than the static of a completely random generator.

As the noise ages, it shifts its colors around. It rotates through hues until it reaches its final form: gray. To be completely honest, I first discovered the gray when messing around with HSL to RGB conversion. It's still a cool effect, so I decided to keep it! :)

## Controls

|Key|Action|
|-|-|
|Space|Change type of noise.|
|R|Reset age to 0.|
|Esc|Close program.|

## Note for web version (WASM)

You may have to zoom in with (Ctrl+'+' / Cmd+'+'). I've been experiencing an issue where everything is small, and haven't had the time to fix it. It may be a bit blurry, but it's the best I can do at the moment.

## Final thoughts

While I could've done more to this, it was the best and coolest option at the moment. If you want to learn more about noise (specifically Perlin noise), I recommend [The Nature of Code's video](https://youtu.be/8ZEMLCnn8v0). You can also read the [Wikipedia article](https://en.wikipedia.org/wiki/Perlin_noise), though it is more complex and hard to understand.

Until tomorrow,

~BD103
