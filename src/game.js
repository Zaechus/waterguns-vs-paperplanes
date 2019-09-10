import { PaperPlane, init } from "waterguns-vs-paperplanes";

(() => {
    init();

    const canvas = document.getElementById("gameCanvas");
    canvas.width = 800;
    canvas.height = 600;

    var mouseX = canvas.width / 2;

    const ctx = canvas.getContext("2d");

    let plane = PaperPlane.new(10);

    function draw() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        ctx.beginPath();
        ctx.rect(plane.x(), 100, 10, 10);
        ctx.fillStyle = "#FFFFFF";
        ctx.fill();
        ctx.closePath();

        plane.fly();
    }

    function mouseMove(e) {
        mouseX = e.screenX;
    }

    document.addEventListener('mousemove', mouseMove);

    setInterval(draw, 10);
})();