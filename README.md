# Survivor Game

Survivor Game is a Rust-based game project, heavily inspired by the game mechanics of Vampire Survivor and the ability and character control of The Binding of Isaac. It was developed as a hobby project and is entirely written in Rust using the Bevy game engine.

This project has been open sourced because its development was halted in favor of other projects and due to the lack of a clear direction in the game design aspect. Please note that all the original assets have been removed due to licensing issues.

While the documentation might not be up to date with the latest update, it should still provide a good grasp of the project's workings. A Trello board was used during the development process, but due to the considerable effort required to clean it up for public access, it's not provided.

## Features

1. **Quad-tree and Collision Detection/Resolution**: The project implements a self-made quad-tree for efficient spatial partitioning, as well as a custom collision detection and resolution system.

2. **Components Application**: We've developed a unique approach for applying components (abilities) with generic systems as a response to an event.

3. **Entities and Components Description**: We've created a system that describes and parses entities and their components as JSON files. This system allows the creation of abilities using just JSON files without requiring any changes to the game code.

4. **Debug Console**: We've built an in-game debug console for enabling cheats and facilitating more efficient game development and testing.

5. **Enemy Spawning**: We've developed a spawner system that spawns enemies in patterns and spreads their spawning over multiple frames to improve performance.

6. **Attribute System**: We have a custom, albeit fairly standard, attribute system in place.

7. **Scheduling**: The game features a well-functioning scheduling system.

8. **Variety of Abilities**: The game already has many different abilities implemented, which can serve as a helpful resource for developers looking for inspiration or trying to replicate game mechanics.

## Contribution

While the original development has stopped, you're welcome to clone, modify, and distribute the project under its license. If you encounter issues or have suggestions, please open an issue.

## License

Survivor Game is open-source software licensed under the MIT License. You can find a copy of this license in the LICENSE file of this repository.

## Disclaimers

The project is provided as-is, and while we've made every effort to ensure it is useful and usable, there are no guarantees of ongoing support or updates. Please refer to the license for further disclaimers and terms.
