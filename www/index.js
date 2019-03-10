import { Universe } from "../pkg";

const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new();

setInterval(() => {
    pre.textContent = universe.render();
    universe.tick();
}, 200);

// const renderLoop = () => {
//     pre.textContent = universe.render();
//     universe.tick();

//     requestAnimationFrame(renderLoop);
// }

// requestAnimationFrame(renderLoop);
