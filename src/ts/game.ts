import { Game } from "waterguns-vs-paperplanes";

(() => {
    var mouseX = 0;
    var mouseY = 0;
    var mousePressed = false;

    let game = Game.new();

    // render the actual game
    function gameLoop() {
        game.draw(mouseX, mouseY, mousePressed);

        window.requestAnimationFrame(gameLoop);
    }
    gameLoop();

    // handle events
    function mouseMove(e: MouseEvent) {
        mouseX = e.clientX;
        mouseY = e.clientY;
    }
    function mouseDown(e: MouseEvent) {
        mousePressed = true;
    }
    function mouseUp(e: MouseEvent) {
        mousePressed = false;
    }
    document.addEventListener('mousemove', mouseMove);
    document.addEventListener('mousedown', mouseDown);
    document.addEventListener('mouseup', mouseUp);

    // warn user when reloading
    window.onbeforeunload = function () {
        return "reloading";
    };
})();