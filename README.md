# Maze

Simple maze game using [Bevy](https://bevyengine.org/) and [Rapier](https://rapier.rs/).

It have 4 maze generation algorithm from [maze_generator](https://crates.io/crates/maze_generator) crate but they are not (yet?) selectable.

You can try it out [here](https://dalvany.github.io/maze/).

## Features

* `diagnostic` (default : not enabled) : add [diagnostic plugins](https://docs.rs/bevy/0.11.1/bevy/diagnostic/index.html).
* `inspector` (default : not enabled) : add [bevy-inspector-egui](https://docs.rs/bevy-inspector-egui/0.19.0/bevy_inspector_egui/)
* `debug` (default : not enabled) : enabled `diagnostic` and `inspector` features
* `js` (default : not enabled) : allow rand to work within JS environment

## Resources

* [Bevy](https://bevyengine.org/)
* [Bevy unofficial book](https://bevy-cheatbook.github.io/)
* [Rapier](https://rapier.rs/)