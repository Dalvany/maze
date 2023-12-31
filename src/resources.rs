use bevy::prelude::*;
use maze_generator::{
    ellers_algorithm::EllersGenerator,
    growing_tree::GrowingTreeGenerator,
    prelude::{Generator, Maze},
    prims_algorithm::PrimsGenerator,
    recursive_backtracking::RbGenerator,
};

#[derive(Default, Clone, Copy)]
pub(crate) enum Algorithm {
    Ellers,
    #[default]
    GrowingTree,
    Prims,
    RecursiveBacktracking,
}

#[derive(Resource, Clone, Copy)]
pub(crate) struct MazeConfig {
    algorithm: Algorithm,
    height: i32,
    width: i32,
}

/*
impl MazeConfig {
    pub(crate) fn set_algorithm(&mut self, algorithm: Algorithm) {
        self.algorithm = algorithm;
    }

    pub(crate) fn set_height(&mut self, height: i32) {
        self.height = height;
    }

    pub(crate) fn set_width(&mut self, width: i32) {
        self.width = width;
    }
}
*/

impl Default for MazeConfig {
    fn default() -> Self {
        Self {
            algorithm: Default::default(),
            height: 15,
            width: 15,
        }
    }
}

impl TryFrom<&MazeConfig> for Maze {
    type Error = anyhow::Error;

    fn try_from(value: &MazeConfig) -> Result<Self, Self::Error> {
        // Seeds using getrandom crate with JS feature enable
        // for wasm in JS environment to work.
        #[cfg(feature = "js")]
        let seed = {
            let mut buf = [0u8; 32];
            getrandom::getrandom(&mut buf).map_err(|err| anyhow::Error::msg(err.to_string()))?;
            Some(buf)
        };
        // Otherwise don't bother
        #[cfg(not(feature = "js"))]
        let seed = None;

        match value.algorithm {
            Algorithm::Ellers => EllersGenerator::new(seed).generate(value.width, value.height),
            Algorithm::GrowingTree => {
                GrowingTreeGenerator::new(seed).generate(value.width, value.height)
            }
            Algorithm::Prims => PrimsGenerator::new(seed).generate(value.width, value.height),
            Algorithm::RecursiveBacktracking => {
                RbGenerator::new(seed).generate(value.width, value.height)
            }
        }
    }
}
