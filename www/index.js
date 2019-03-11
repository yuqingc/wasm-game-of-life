import { Universe, Cell } from "../pkg";
import { memory } from "../pkg/wasm_game_of_life_bg";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
// To have a 1px border for each cell
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d')

// setInterval(() => {
//     drawGrid();
//     drawCells();
//     universe.tick();
// }, 100);

function drawGrid() {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
}

function getIndex(row, col) {
    return row * width + col;
}

function drawCells() {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
        
            ctx.fillStyle = cells[idx] === Cell.Dead
                ? DEAD_COLOR
                : ALIVE_COLOR;
        
            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }
    
    ctx.stroke();
}

// let startTime = null;
let animationId = null;
const renderLoop = (timestamp) => {
    // if (!startTime) startTime = timestamp;
    drawGrid();
    drawCells();
    universe.tick();

    // const progress = timestamp - startTime;
    animationId = requestAnimationFrame(renderLoop);
}

function isPaused() {
    return animationId === null;
}

const playPauseButton = document.getElementById("play-pause");

function play() {
    playPauseButton.textContent = "Pause";
    renderLoop();
}

function pause() {
    playPauseButton.textContent = "Play";
    cancelAnimationFrame(animationId);
    animationId = null;
}

playPauseButton.addEventListener("click", function(e) {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

canvas.addEventListener("click", function(event) {
    const boundingRect = canvas.getBoundingClientRect();

    // console.log('canvas width height', canvas.width, canvas.height);
    // console.log('rect', boundingRect);

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    universe.toggle_cell(row, col);

    drawGrid();
    drawCells();
});

play();

// drawGrid();
// drawCells();
// requestAnimationFrame(renderLoop);
