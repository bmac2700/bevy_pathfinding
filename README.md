# bevy_pathfinding
The point of this project was to learn how to implement the A* search algorithm in Rust and how to visualise it with the Bevy game engine.
You can find the A* implementation in the astar.rs file.

# How to run it
First you need to build it, building the game requires the rust toolchain which you can get from https://www.rust-lang.org/
After you have installed the rust toolchain you can compile & run the application by running ```cargo run --release``` in the project folder.

# Game controls
ESC - Locks/Unlocks your mouse cursor
WASD + Space + Shift - Movement keys
You can move the start & goal points by clicking one of the cubes that exist in the world, the green cube is the goal and the red one is the starting point.
