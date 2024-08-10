# game 🦀

Just move and shoot :)

## Game Features

- **Shooting Mechanics**: 
  - Shoot in four directions (up, down, left, right) using the `H`, `J`, `K`, and `L` keys.
  - Limited bullets with the ability to purchase more from the shop.

- **Enemy AI**:
  - Enemies smoothly move towards the player's position.
  - Increasing difficulty with more enemies per level.

- **Coin Collection**:
  - Coins spawn at intervals and can be collected for points.
  - Coins despawn after a certain time if not collected.

- **Shop System**:
  - Spend points to buy additional bullets, remove enemies, or gamble for a score boost.

- **Player Health and Losing Condition**:
  - Collision with an enemy results in game over.
  - Reset the game by pressing the `Space` key after losing.

- **Level Progression**:
  - Advance to the next level by defeating all enemies.
  - Each level increases the number of enemies.

- **Menu System**:
  - Start the game, access the shop, or quit using the menu.
  - Press `Escape` to toggle the menu at any time.

- **FPS Display**:
  - Real-time FPS counter to monitor game performance.

## Requirements

- Rust programming language installed ([Install Rust](https://www.rust-lang.org/))

## Usage

1. Ensure Rust is installed on your system.
2. Clone the repository:
    ```bash
    git clone https://github.com/Telmo-Sousa/game.git
    cd game
    ```
3. Run the project:
    ```bash
    cargo run --release
    ```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Important
Have fun!
