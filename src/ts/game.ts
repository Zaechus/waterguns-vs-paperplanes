import { PaperPlane, init } from "waterguns-vs-paperplanes";

(() => {
    init();

    const canvas = <HTMLCanvasElement>document.getElementById("gameCanvas");
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    var mouseX = canvas.width / 2;

    const ctx = canvas.getContext("2d");

    const planeImage = new Image(50, 50);
    planeImage.src = "static/plane.png";

    let planes = [];
    for (let x = 0; x < 7; x += 1) {
        planes.push(PaperPlane.new(planeImage, 50 + x * 100, 100, 50, 50, 10));
    }

    function draw() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        for (let x = 0; x < 7; x += 1) {
            planes[x].draw(ctx);
            planes[x].fly();
        }
    }

    function mouseMove(e: MouseEvent) {
        mouseX = e.screenX;
    }
    document.addEventListener('mousemove', mouseMove);

    setInterval(draw, 10);
})();