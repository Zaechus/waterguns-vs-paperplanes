export function drawPlane(ctx: CanvasRenderingContext2D, img: CanvasImageSource, x: number, y: number, w: number, h: number) {
    ctx.drawImage(img, x, y, w, h);
}