#[macro_use]
extern crate lazy_static;

#[cfg(feature = "alphabet")]
pub mod alphabet;
#[cfg(feature = "cell")]
pub mod cell;
#[cfg(feature = "heatmap")]
pub mod heatmap;
#[cfg(feature = "image")]
pub mod image;
#[cfg(feature = "svg")]
pub mod svg;
#[cfg(feature = "voronoi")]
pub mod voronoi;
