import { Game } from "waterguns-vs-paperplanes";

(() => {
    let game = Game.new();

    function start() {
        game.draw();

        window.requestAnimationFrame(start);
    }

    window.onbeforeunload = function () {
        return "reloading";
    };

    start();
})();