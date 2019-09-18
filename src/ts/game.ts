import { PaperPlane, Tower, Rect, init } from "waterguns-vs-paperplanes";

(() => {
    init();

    const canvas = <HTMLCanvasElement>document.getElementById("gameCanvas");
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    var mouseX = canvas.width / 2;

    const ctx = canvas.getContext("2d");

    const planeImage = new Image(10, 10);
    planeImage.src = "static/plane.png";

    const gunImage = new Image(10, 10);
    gunImage.src = "static/gun.png";

    let planes = [];
    for (let i = 0; i < 7; i += 1) {
        planes.push(PaperPlane.new(planeImage, Rect.new(50 + i * 100, 10 + i * 25, 50, 50), 50));
    }

    let tower = Tower.new(gunImage, Rect.new(canvas.width / 2, canvas.height / 2, 75, 75), 10, 50);

    function kill() {
        for (let i = 0; i < planes.length; i += 1) {
            if (planes[i].y() > tower.y()) {
                planes[i].take_damage(tower.damage());
            }
        }
    }

    function draw() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        tower.draw(ctx);

        for (let i = 0; i < planes.length; i += 1) {
            planes[i].draw(ctx);
            planes[i].fly();

            if (planes[i].hp() <= 0) {
                planes.splice(i, 1)
            }
        }
    }

    function mouseMove(e: MouseEvent) {
        mouseX = e.screenX;
    }
    document.addEventListener("mousemove", mouseMove);

    setInterval(kill, 1000);
    setInterval(draw, 10);
})();