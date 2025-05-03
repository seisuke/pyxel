globalThis._virtualGamepadStates = [
  false, // Up
  false, // Down
  false, // Left
  false, // Right
  false, // A
  false, // B
  false, // X
  false, // Y
];

import initModule from './pyxel_wrapper_ts.js';

const canvas = document.getElementById("canvas");

export const instancePromise = initModule({
  canvas
}).then((module) => ({ exports: module }));
