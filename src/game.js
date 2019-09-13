"use strict";
exports.__esModule = true;
var waterguns_vs_paperplanes_1 = require("waterguns-vs-paperplanes");
(function () {
    waterguns_vs_paperplanes_1.init();
    var canvas = document.getElementById("gameCanvas");
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    var mouseX = canvas.width / 2;
    var ctx = canvas.getContext("2d");
    var planeImage = new Image(50, 50);
    planeImage.src = "static/plane.png";
    var planes = [];
    for (var x = 0; x < 7; x += 1) {
        planes.push(waterguns_vs_paperplanes_1.PaperPlane["new"](planeImage, 50 + x * 100, 100, 50, 50, 10));
    }
    function draw() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        for (var x = 0; x < 7; x += 1) {
            planes[x].draw(ctx);
            planes[x].fly();
        }
    }
    function mouseMove(e) {
        mouseX = e.screenX;
    }
    document.addEventListener('mousemove', mouseMove);
    setInterval(draw, 10);
})();
