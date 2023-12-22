# labyru

*labyru* generates mazes.

## Usage

```bash
 ./maze-maker --help
Generates mazes

Usage: maze-maker [OPTIONS] --method <METHOD> <PATH>

Arguments:
  <PATH>  The output SVG

Options:
      --walls <SHAPE>            The number of walls per room: 3, 4 or 6 [default: 4]
      --width <WIDTH>            The width of the maze, in rooms
      --height <HEIGHT>          The height of the maze, in rooms
      --method <METHOD>          The initialisation method to use
      --scale <SCALE>            A relative size for the maze, applied to rooms [default: 10]
      --seed <SEED>              A seed for the random number generator
      --margin <MARGIN>          The margin around the maze [default: 10]
      --mask <INITIALIZE>        A mask image to determine which rooms are part of the mask and thenshold luminosity value between 0 and 1 on the form "path,0.5"
      --heat-map <HEATMAP>       Whether to create a heat map
      --background <BACKGROUND>  A background image to colour rooms
      --ratio <RATIO>            A ratio for pixels per room when using a background
      --text <TEXT>              A text to draw on the maze
      --solve <SOLVE>            Whether to solve the maze, and the solution colour. If not specified, the colour defaults to "black"
      --break <POST_BREAK>       Whether to break the maze
  -h, --help                     Print help
  -V, --version                  Print version
```
