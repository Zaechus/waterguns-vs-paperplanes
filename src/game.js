"use strict";
exports.__esModule = true;
var waterguns_vs_paperplanes_1 = require("waterguns-vs-paperplanes");
var draw_js_1 = require("./draw.js");
(function () {
    waterguns_vs_paperplanes_1.init();
    var canvas = document.getElementById("gameCanvas");
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    var mouseX = canvas.width / 2;
    var ctx = canvas.getContext("2d");
    var planeImage = new Image(50, 50);
    var plane = waterguns_vs_paperplanes_1.PaperPlane["new"](50, 100);
    var planes = [];
    for (var x = 0; x < 7; x += 1) {
        planes.push(waterguns_vs_paperplanes_1.PaperPlane["new"](50 + x * 100, 100));
    }
    planeImage.src = "static/plane.png";
    function draw() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        for (var x = 0; x < 7; x += 1) {
            draw_js_1.drawPlane(ctx, planeImage, planes[x].x(), planes[x].y(), planeImage.width, planeImage.height);
            planes[x].fly();
        }
    }
    function mouseMove(e) {
        mouseX = e.screenX;
    }
    document.addEventListener('mousemove', mouseMove);
    setInterval(draw, 10);
})();
