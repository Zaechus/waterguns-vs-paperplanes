import { Game } from "waterguns-vs-paperplanes";

(() => {
    var mouseX = 0;
    var mouseY = 0;
    var mouseDown = false;
    var mouseUp = false;

    var state = 0;

    let game = Game.new();

    function menuState() {
        alert("MENU");
        let canvas = document.createElement("canvas");
        let ctx = canvas.getContext("2d");

        ctx.clearRect(0, 0, canvas.width, canvas.height);

        // Switch to game state
        state = 1;
        // Reset the game
        game = Game.init_new();
    }

    // Render the actual game
    function gameState() {
        game.draw(mouseX, mouseY, mouseDown, mouseUp);

        if (game.isDefeated()) {
            // Switch to fail state
            state = 2;
        }

        mouseUp = false;
    }

    function failState() {
        alert("YOU WERE DEFEATED");

        // Switch back to menu state
        state = 0;
    }

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
    document.addEventListener('contextmenu', (e: MouseEvent) => {
        e.preventDefault();
    });

    // warn user when reloading
    window.onbeforeunload = function () {
        return "reloading";
    };

    function states() {
        switch (state) {
            case 0:
                menuState();
                break;
            case 1:
                gameState();
                break;
            case 2:
                failState();
                break;
            default:
                alert("DEFAULT");
        }
        window.requestAnimationFrame(states);
    }
    states();
})();