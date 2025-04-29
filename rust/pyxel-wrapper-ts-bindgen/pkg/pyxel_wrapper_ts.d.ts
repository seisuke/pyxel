declare module "pyxel" {
  export function init(_width: number, _height: number): void;
  export class Image {
    new(width: number, height: number): Image;
    width(): number;
    height(): number;
  }
}