const PYXEL_WORKING_DIRECTORY = "/pyxel_working_directory";

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
}).then((module) => {
  const FS = module.FS;
  return {
    exports: module,
    FS,
  };
});

instancePromise.then(({ exports: _, FS }) => {
  FS.mkdir(PYXEL_WORKING_DIRECTORY);
  FS.chdir(PYXEL_WORKING_DIRECTORY);
});
