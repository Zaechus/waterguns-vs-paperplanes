export function drawPlane(ctx: CanvasRenderingContext2D, x: number, y: number, w: number, h: number) {
    ctx.beginPath();
    ctx.rect(x, y, w, h);
    ctx.fillStyle = "#FFFFFF";
    ctx.fill();
    ctx.closePath();
}