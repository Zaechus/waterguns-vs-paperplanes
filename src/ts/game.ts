import { PaperPlane, Tower, Rect, init } from "waterguns-vs-paperplanes";

(() => {
    init();

    const canvas = <HTMLCanvasElement>document.getElementById("gameCanvas");
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    var mouseX = canvas.width / 2;

    var hp = 100;
    var cash = 0;

    const ctx = canvas.getContext("2d");
    ctx.font = "36px monospace";

    const planeImage = new Image(10, 10);
    planeImage.src = "static/plane.png";

    const gunImage = new Image(10, 10);
    gunImage.src = "static/gun.png";

    let planes = [];
    for (let i = 0; i < 7; i += 1) {
        planes.push(PaperPlane.new(planeImage, Rect.new(-800 + i * 100, (canvas.height / 2) + i * 25, 50, 50), 50));
    }

    let towers = [];
    for (let i = 0; i < 2; i += 1) {
        towers.push(
            Tower.new(gunImage, Rect.new(500 + i * 1000, canvas.height / 2, 75, 75), 15, 250));
    }

    function killPlanes() {
        for (let i = planes.length - 1; i >= 0; i -= 1) {
            for (let tower of towers) {
                planes[i].take_damage(tower, tower.damage(performance));
            }
        }
    }

    function renderText() {
        ctx.beginPath();
        ctx.fillStyle = "blue";
        ctx.fillText("  HP: " + hp.toString(), 10, 40);
        ctx.fillText("Cash: $" + cash.toString(), 10, 80);
        ctx.closePath();
    }

    function handlePlanes() {
        for (let i = 0; i < planes.length; i += 1) {
            if (planes[i].hp() <= 0) {
                planes.splice(i, 1);
                cash += 10;
            } else if (planes[i].x() >= canvas.width) {
                planes.splice(i, 1);
                hp -= 1;
            } else {
                planes[i].draw(ctx);
                planes[i].fly();
            }
        }
    }

    function renderLoop() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        renderText();

        for (let tower of towers) {
            tower.draw(ctx);
        }

        handlePlanes();

        killPlanes();

        requestAnimationFrame(renderLoop);
    }

    function mouseMove(e: MouseEvent) {
        mouseX = e.screenX;
    }
    document.addEventListener("mousemove", mouseMove);

    renderLoop();
})();