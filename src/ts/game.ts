import { Game } from "waterguns-vs-paperplanes";

(() => {
    var noContext = document.getElementsByTagName("canvas");
    var mouseX = 0;
    var mouseY = 0;
    var mouseDown = false;
    var mouseUp = false;

    let game = Game.new();

    // render the actual game
    function gameLoop() {
        game.draw(mouseX, mouseY, mouseDown, mouseUp);

        window.requestAnimationFrame(gameLoop);
        mouseUp = false;
    }
    gameLoop();

    // handle mouse events
    document.addEventListener('mousemove', (e: MouseEvent) => {
        mouseX = e.clientX;
        mouseY = e.clientY;
    });
    document.addEventListener('mousedown', (e: MouseEvent) => {
        mouseDown = true;
    });
    document.addEventListener('mouseup', (e: MouseEvent) => {
        mouseUp = true;
        mouseDown = false;
    });

    // prevent the context menu from popping up on a right click
    noContext[0].addEventListener('contextmenu', (e: MouseEvent) => {
        mouseDown = true;
        e.preventDefault();
    });

    // warn user when reloading
    window.onbeforeunload = function () {
        return "reloading";
    };
})();