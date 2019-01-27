/**
 * The ID of the canvas element.
 */
const CANVAS_ID = 'canvas';

/**
 * The ID of the logo element.
 */
const LOGO_ID = 'logo';

/**
 * The URL of the start-up image.
 */
const STARTUP_URL = 'https://newrainsoftware.com/labyru/hex/4x5/image.svg';


const seed = 1234;
const walls = 6;
const width = 20;
const height = 20;


let gameloop = (app, gl) => {
    let previous = undefined;
    let tick = (current) => {
        let d = previous ? current - previous : 0.0;
        previous = current;
        app.render(gl);
        requestAnimationFrame(tick);
    };
    requestAnimationFrame(tick);
};


let main = () => {
    // We keep references for simplicity
    let canvas = document.getElementById(CANVAS_ID);
    let logo = document.getElementById(LOGO_ID);

    // Load the wasm module
    let app = import('./pkg/maze_wasm')
        .then(maze_wasm => new maze_wasm.App(seed, walls, width, height));

    // Retrieve a WebGL context
    let gl = new Promise((resolve, reject) => {
        let image = new Image();
        image.onload = () => {
            let gl = canvas.getContext('webgl');
            resolve(gl)
        };
        image.src = STARTUP_URL;
    });

    // Enter full screen
    let fullscreen = new Promise((resolve, reject) => {
        let fullscreen = () => canvas.requestFullscreen()
            .then(resolve)
            .catch(reject);
        logo.addEventListener('click', fullscreen, false);
    })

    // Synchronise game view to window
    let resizer = () => {
        let width  = canvas.clientWidth;
        let height = canvas.clientHeight;

        if (canvas.width != width || canvas.height != height) {
            canvas.width = width;
            canvas.height = height;
        }
    };
    window.addEventListener('resize', resizer, false);
    resizer();

    // Finally start the game loop
    Promise.all([app, gl, fullscreen])
        .then(values => {
            let app = values[0];
            let gl = values[1];

            // Initialise game loop
            gameloop(app, gl);
        });
};

main();
