# Snake Game — Design Document

**Project:** Snake (Rust)
**Author:** Marcus
**Started:** September 28, 2026
**Target:** Playable by end of October 2026
**Status:** Core game working, 16 tests passing

---

## What We're Building

A classic snake game in Rust with a graphical window. The snake moves on a grid, eats food to grow longer, and the game ends when the snake hits a wall or itself. Score increases with each food eaten, and the game speeds up over time.

This is the first project of a planned game studio. The goal is to build it properly, not as a toy.

---

## Design Decisions

Every decision below was made on September 28, 2026. If Marcus wants to change any of them, it should be written here so the reasoning isn't lost.

### 1. Wall collision = death
**Decision:** When the snake hits any edge of the grid, the game is over.
**Why:** This is the classic Nokia snake behavior. It's the most common expectation.
**Alternative considered:** Snake wraps around to the other side. Rejected — less challenging, less classic.

### 2. Self-bite = instant game over
**Decision:** When the snake's head touches any part of its own body, the game is over.
**Why:** Classic behavior. Simple to understand.
**Alternative considered:** Snake shortens from the bite point. Rejected — more complex, less classic.

### 3. Pause = P or Escape key
**Decision:** Press P or Escape to pause and resume.
**Why:** Standard pause keys in most games. Easy to remember.

### 4. Graphics library = macroquad
**Decision:** Use the `macroquad` crate (library) for the game window and drawing.
**Why:** It's simple, well-documented, and works on Mac. It handles the window, keyboard input, and drawing shapes. Without it, we'd need to write hundreds of lines of low-level code just to open a window.
**What it does:** Opens a window, draws the grid/snake/food, reads keyboard input. It does NOT handle game logic — that's all our code.

### 5. Speed increases on food
**Decision:** Each time the snake eats, it gets slightly faster.
**Why:** Makes the game progressively more challenging. The speed increase is small (0.005 seconds faster per food) with a minimum speed cap so it never becomes impossible.

### 6. Best score saved to file
**Decision:** Best score is saved to a file called `best_score.txt` in the same folder as the game.
**Why:** Simple, no database needed. The file is created automatically when you beat your score.

### 7. Code structure = two files
**Decision:** Game logic in `src/game.rs`, graphics and input in `src/main.rs`.
**Why:** Separation of concerns. The game logic can be tested without opening a window. The graphics code is simple and doesn't need testing. If we ever want to change how the game looks, we only touch `main.rs`. If we want to change how the game plays, we only touch `game.rs`.

---

## Controls

| Key | Action |
|-----|--------|
| Arrow keys | Steer the snake |
| P or Escape | Pause / resume |
| Space | Restart after game over |

---

## Technical Details

- **Language:** Rust 1.98.1
- **Graphics:** macroquad 0.4
- **Grid size:** 20 x 20 cells
- **Cell size:** 30 pixels
- **Window size:** 600 x 600 pixels (grid) + space below for score
- **Initial speed:** Snake moves every 0.15 seconds
- **Speed increase:** 0.005 seconds faster per food eaten
- **Minimum speed:** 0.05 seconds per move (fastest possible)

---

## What's Done

- [x] Project set up with Cargo (Rust's build tool)
- [x] Game logic written (movement, collision, scoring, food spawning)
- [x] Graphics and input written (window, drawing, keyboard)
- [x] 16 tests written and passing
- [x] Best score saving works
- [x] Pause works

## What's Left

- [ ] Visual polish (colors, fonts, maybe a menu screen)
- [ ] Sound effects (eating, game over)
- [ ] Build a standalone app (so you can double-click to play, no terminal needed)
- [ ] Put on GitHub (done — see below)

---

## How to Run

**To play the game:**
```
cd /Users/Nicolas/snake-game
cargo run
```

**To run the tests:**
```
cd /Users/Nicolas/snake-game
cargo test
```

---

## GitHub

The code is on GitHub at: https://github.com/nicolas-found42/snake-game

This means:
- Your code is saved safely (not just on your laptop)
- You can show it to people
- If your laptop dies, the code is still there
- Future-you can clone it and keep building

---

## File Map

```
snake-game/
├── Cargo.toml          # Project settings and dependencies
├── Cargo.lock          # Exact versions of all libraries
├── best_score.txt      # Your best score (created automatically)
├── src/
│   ├── main.rs         # Graphics and input (window, drawing, keyboard)
│   └── game.rs         # Game logic (movement, collision, scoring) + tests
└── target/             # Build output (don't touch this)
```

---

*This document should be updated whenever a design decision is made or changed.*
