import { PaperPlane, init } from "waterguns-vs-paperplanes";

import { drawPlane } from "./draw.js";

(() => {
    init();

    const canvas = <HTMLCanvasElement>document.getElementById("gameCanvas");
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    var mouseX = canvas.width / 2;

    const ctx = canvas.getContext("2d");

    const planeImage = new Image(50, 50);
    let plane = PaperPlane.new(50, 100);

    let planes = [];
    for (let x = 0; x < 7; x += 1) {
        planes.push(PaperPlane.new(50 + x * 100, 100));
    }

    planeImage.src = "static/plane.png";

    function draw() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        for (let x = 0; x < 7; x += 1) {
            drawPlane(ctx, planeImage, planes[x].x(), planes[x].y(), planeImage.width, planeImage.height);
            planes[x].fly();
        }

    }

    function mouseMove(e) {
        mouseX = e.screenX;
    }
    document.addEventListener('mousemove', mouseMove);

    setInterval(draw, 10);
})();