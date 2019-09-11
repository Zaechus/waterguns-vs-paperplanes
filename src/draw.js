export function drawPlane(ctx, x, y, w, h) {
    ctx.beginPath();
    ctx.rect(x, y, w, h);
    ctx.fillStyle = "#FFFFFF";
    ctx.fill();
    ctx.closePath();
}