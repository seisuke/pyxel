import init, { init as pyxelInit, update, draw } from "./pkg/pyxel_wrapper_ts.js";

await init();
pyxelInit(160, 120, 30);

function loop() {
    update();
    draw();
    requestAnimationFrame(loop);
}
loop();
