declare module "pyxel" {
  export function init(width: number, height: number): void;
  export function cls(color: number): void;
  export class Image {
    new(width: number, height: number): Image;
    width(): number;
    height(): number;
  }
}