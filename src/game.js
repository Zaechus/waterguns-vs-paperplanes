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
    var plane = waterguns_vs_paperplanes_1.PaperPlane["new"](10, 100);
    function draw() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        draw_js_1.drawPlane(ctx, plane.x(), plane.y(), 25, 25);
        plane.fly();
    }
    function mouseMove(e) {
        mouseX = e.screenX;
    }
    document.addEventListener('mousemove', mouseMove);
    setInterval(draw, 10);
})();
