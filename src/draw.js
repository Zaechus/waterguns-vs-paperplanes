"use strict";
exports.__esModule = true;
function drawPlane(ctx, x, y, r) {
    ctx.beginPath();
    ctx.moveTo(x, y - r);
    ctx.lineTo(x + r, y + r);
    ctx.lineTo(x - r, y + r);
    ctx.fillStyle = "#FFFFFF";
    ctx.fill();
    ctx.closePath();
}
exports.drawPlane = drawPlane;
