import { Game } from "waterguns-vs-paperplanes";

(() => {
    let game = Game.new();

    function gameLoop() {
        game.draw();

        window.requestAnimationFrame(gameLoop);
    }
    gameLoop();

    // warn user when reloading
    window.onbeforeunload = function () {
        return "reloading";
    };
})();