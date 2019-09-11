import { PaperPlane, init } from "waterguns-vs-paperplanes";

import { drawPlane } from "./draw.js";

(() => {
    init();

    const canvas = <HTMLCanvasElement>document.getElementById("gameCanvas");
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    var mouseX = canvas.width / 2;

    const ctx = canvas.getContext("2d");

    let plane = PaperPlane.new(10, 100);

    function draw() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        drawPlane(ctx, plane.x(), plane.y(), 25);

        plane.fly();
    }

    function mouseMove(e) {
        mouseX = e.screenX;
    }
    document.addEventListener('mousemove', mouseMove);

    setInterval(draw, 10);
})();