const EPIC_REPLS = [
    // My cool repls #notbiases
    ["BD103", "Something-that-Ages"],
    ["BD103", "EmoScript"],

    // Other cool repls
    ["RayhanADev", "Replit-Creates-2"], // Conway's Game of Life
    ["RayhanADev", "Replit-Creates-4"], // Facecam Emoji Detector
    ["RayhanADev", "Replit-Creates-5"], // Expanding Paint

    ["IroncladDev", "Emoji-Cannon"],

    ["IanAtReplit", "Replit-Creates-1-Worst-Website"], // Smiley fading in and out

    ["quanuhs", "Ages-website"],
    ["quanuhs", "emojily"], // Emoji guesser game

    ["nullmetry", "Firemoji"], // Emoji Explosions

    ["conspicous", "Watch-CodingCactus-Age"],

    ["phamn23", "A-Very-Long-Cat"],
    ["phamn23", "Cherry-Tree"],

    ["ItsArnob", "text-to-emoji"],
];

export default defineEventHandler((event) => {
    // Pick a random repl
    let repl = EPIC_REPLS[Math.floor(Math.random() * EPIC_REPLS.length)];

    return {
        creator: repl[0],
        name: repl[1],
    }
});
