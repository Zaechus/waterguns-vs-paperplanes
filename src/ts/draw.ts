export function drawPlane(ctx: CanvasRenderingContext2D, x: number, y: number, r: number) {
    ctx.beginPath();
    ctx.moveTo(x, y - r);
    ctx.lineTo(x + r, y + r);
    ctx.lineTo(x - r, y + r);
    ctx.fillStyle = "#FFFFFF";
    ctx.fill();
    ctx.closePath();
}