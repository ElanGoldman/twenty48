# 2048 in Rust

A fully-featured desktop implementation of 2048, built with Rust and [egui](https://github.com/emilk/egui) via [eframe](https://github.com/emilk/egui/tree/master/crates/eframe).

## How to Play

Slide tiles with the **arrow keys** or **WASD**. When two tiles with the same number collide, they merge into one. Reach the **2048** tile to win. The game ends when no moves remain.

## What Makes This Different from Standard 2048

### Variable Board Size
The classic game is always 4×4. This version lets you choose any board size (minimum 2×2) at any time. Enter a size in the text box and press **Apply** or **Enter** to immediately start a new game on that grid. Larger boards (6×6, 8×8, …) dramatically change the difficulty and strategy.

### Unlimited Undo and Redo
The game keeps a full history of every move. Press **Undo** to step back as many times as you want, and **Redo** to replay moves you undid. The redo stack is cleared as soon as you make a new move. Each undo/redo saves the current state to the opposite stack, so you can jump freely back and forth.

### Persistent Leaderboard
The top 10 scores are saved to disk and survive between sessions. When a game ends (no moves left), your score is automatically recorded and the leaderboard pops up. You can also check it mid-game with the **Leaderboard** button. Scores include the board size so results across different grids are all visible at a glance.

### Win-and-Continue
Reaching 2048 shows a congratulations message but does **not** freeze the game. You can keep playing to push your score higher. Your score is still saved to the leaderboard when the board eventually fills up.

---

## Advanced Rust Concepts Used

### Trait Objects (`dyn Trait`)
`eframe::Storage` is accessed exclusively through `&dyn eframe::Storage` and `&mut dyn eframe::Storage` — fat pointers to an unknown concrete type. The app itself is handed to eframe as a `Box<dyn eframe::App>`. This is dynamic dispatch: the concrete types are erased at compile time and method calls are resolved at runtime through a vtable.

### `let`-`else` Expressions
Undo and redo use the `let`-`else` form (stabilised in Rust 1.65) to destructure an `Option` and early-return in one expression:

```rust
let Some(previous) = self.undo_stack.pop() else {
    return false;
};
```

This avoids nested `if let` and keeps the happy path unindented.

### Serde Derive Macros and `#[serde(skip)]`
`LeaderboardEntry` and `Leaderboard` are serialised to JSON with `#[derive(Serialize, Deserialize)]`. The `show` field (whether the popup is open) must not be persisted, so it is annotated with `#[serde(skip)]` — the field is excluded from serialisation and falls back to its `Default` value on deserialisation.

### `Option` Method Chaining
Loading the leaderboard chains multiple `Option`/`Result` adapters without any explicit `match`:

```rust
storage
    .get_string(Self::STORAGE_KEY)
    .and_then(|s| serde_json::from_str(&s).ok())
    .unwrap_or_default()
```

`and_then` flat-maps over the `Option`, `.ok()` converts the `Result` to an `Option`, and `unwrap_or_default()` produces a blank leaderboard if anything along the chain is `None`.

### External Crate: `grid`
The board is stored as a `Grid<u32>` from the [`grid`](https://crates.io/crates/grid) crate rather than a nested `Vec<Vec<u32>>`. `Grid` stores its data in a single flat allocation, so there is no pointer chasing between rows. It also provides built-in `transpose()` and `flip_cols()` methods that replace the hand-rolled matrix manipulation that would otherwise be needed.

### Iterator Adapters
`Grid::iter()` yields every cell as a flat sequence with no need for `flatten()`. Iterator combinators like `.any()`, `.filter()`, and `.copied()` are used throughout:

```rust
self.board.iter().any(|&x| x == 0)              // any empty cell?
row.iter().copied().filter(|&x| x != 0).collect() // strip zeros from a row
```

### Associated Constants
`Leaderboard` defines limits as typed constants scoped to the struct:

```rust
const STORAGE_KEY: &'static str = "leaderboard_v1";
const MAX_ENTRIES: usize = 10;
```

Associated constants are resolved at compile time and namespaced under the type, avoiding magic strings and numbers scattered across the codebase.

### Struct Update Syntax (`..Default::default()`)
The window options are created by overriding only a few fields and filling the rest from the type's `Default` implementation:

```rust
let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
        .with_inner_size([560.0, 760.0])
        ...,
    ..Default::default()
};
```

### Derived Trait Implementations
`Direction` derives `Debug`, `Clone`, `Copy`, `PartialEq`, and `Eq` automatically. `Game` derives `Debug` and `Clone`. `Leaderboard` and `LeaderboardEntry` derive `Default`, `Clone`, `Serialize`, and `Deserialize`. Rust generates correct, zero-cost implementations for all of these from the struct or enum shape alone.

### Reduce All Moves to One via Transpose/Reverse
Rather than writing four separate slide algorithms, all four directions are reduced to a single `move_left` operation through in-place matrix transformations:

- **Right** = reverse rows → move left → reverse rows  
- **Up** = transpose → move left → transpose  
- **Down** = transpose → reverse rows → move left → reverse rows → transpose  

The `grid` crate's built-in `transpose()` and `flip_cols()` methods make each transformation a single call, keeping the direction logic concise.

---

## Building and Running

```sh
cargo run --release
```

Requires a working Rust toolchain (edition 2024). The native GUI window opens automatically.
