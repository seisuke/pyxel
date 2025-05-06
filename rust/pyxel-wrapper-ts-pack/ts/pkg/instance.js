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

export let instance;

export const instancePromise = (async () => {
  const module = await initModule({ canvas });
  globalThis.instance = module;
  instance = module;
  const FS = module.FS;
  FS.mkdir(PYXEL_WORKING_DIRECTORY);
  FS.chdir(PYXEL_WORKING_DIRECTORY);
  return {
    exports: module,
    FS,
  };
})();

export const ready = instancePromise.then(() => {});
