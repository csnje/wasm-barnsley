const wasm_file = await fetch('./target/wasm32-unknown-unknown/release/wasm_barnsley.wasm');
const importObj = { Math };
const { instance } = await WebAssembly.instantiateStreaming(wasm_file, importObj);

const exports = instance.exports;
const wasmMemory = exports.memory;

const minX = exports.min_x();
const maxX = exports.max_x();
const minY = exports.min_y();
const maxY = exports.max_y();

const canvas = document.getElementById('image');
canvas.width = 800;
canvas.height = 800;

const ctx = canvas.getContext('2d');
// scale and translate to draw in expected direction and location
ctx.scale(canvas.width / (maxX - minX), -canvas.height / (maxY - minY));
ctx.translate(-minX, -maxY);
ctx.fillStyle = 'rgb(0 127 0 / 20%)';

const numPointsPerIteration = 1000;
let xptsPtr = exports.create_array(numPointsPerIteration);
let yptsPtr = exports.create_array(numPointsPerIteration);
let xpts = new Float64Array(wasmMemory.buffer, xptsPtr, numPointsPerIteration);
let ypts = new Float64Array(wasmMemory.buffer, yptsPtr, numPointsPerIteration);

let xpt_prev = 0.0;
let ypt_prev = 0.0;
async function step() {
    // get points
    exports.points(xpt_prev, ypt_prev, xptsPtr, yptsPtr, numPointsPerIteration);

    // draw points
    for (let i = 0; i < numPointsPerIteration; i++) {
        ctx.beginPath();
        ctx.arc(xpts[i], ypts[i], 0.002, 0, 2.0 * Math.PI);
        ctx.fill();
    }

    xpt_prev = xpts[numPointsPerIteration - 1];
    ypt_prev = ypts[numPointsPerIteration - 1];

    requestAnimationFrame(step);
}
requestAnimationFrame(step);
