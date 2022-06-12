(function(){function r(e,n,t){function o(i,f){if(!n[i]){if(!e[i]){var c="function"==typeof require&&require;if(!f&&c)return c(i,!0);if(u)return u(i,!0);var a=new Error("Cannot find module '"+i+"'");throw a.code="MODULE_NOT_FOUND",a}var p=n[i]={exports:{}};e[i][0].call(p.exports,function(r){var n=e[i][1][r];return o(n||r)},p,p.exports,r,e,n,t)}return n[i].exports}for(var u="function"==typeof require&&require,i=0;i<t.length;i++)o(t[i]);return o}return r})()({1:[function(require,module,exports){

function drawCircle(circle) {
    const canvas = document.getElementById('drawing');
    const ctx = canvas.getContext('2d');

    const { imgx, imgy, center, radius, color } = circle;

    const scale_x = canvas.width / imgx;
    const scale_y = canvas.height / imgy;
    ctx.fillStyle = `rgba(${color[0]},${color[1]},${color[2]},${color[3]/255}`;
    ctx.beginPath();
    ctx.ellipse(
        center[0] * scale_x,
        center[1] * scale_y,
        radius * scale_x,
        radius * scale_y,
        0,
        0,
        2 * Math.PI
    );
    ctx.fill();
}

function initWebsocket(uri) {
    const canvas = document.getElementById('drawing');
    const ctx = canvas.getContext('2d');
    const epochBtn = document.getElementById('epoch');

    console.log(uri);
    const ws = new WebSocket(uri);

    ws.onopen = () => {console.log("Websocket opened");};
    ws.onmessage = (message) => {
        console.log(JSON.parse(message.data));
        const data = JSON.parse(message.data);
        const type = Object.keys(data)[0];
        const payload = data[type];
        switch (type) {
            case "Circle":
                drawCircle(payload);
                break;
            case "RoomPath":
                const link_elem = document.getElementById("room-link");
                const link_text_elem = document.getElementById("room-link-text");

                const link = location.host + payload;

                link_elem.href = link;
                link_text_elem.innerHTML = link;
                break;
            case "NewImage":
                const [width, height] = payload.dimensions;
                canvas.width = width;
                canvas.height = height;
                break;
        }
        console.log('received', message.data);
    };

    return ws;
}

module.exports = {
    drawCircle, 
    initWebsocket
};

},{}],2:[function(require,module,exports){
const {drawCircle, initWebsocket} = require('./common.js');


async function run() {
    const room_id = location.pathname.split('/')[2];
    console.log(room_id);
    const uri = 'ws://' + location.hostname + ':3001' + '/join/' + room_id;
    initWebsocket(uri);
}

window.addEventListener("load", async function() {
    console.log("loaded");
    await run();
});

},{"./common.js":1}]},{},[2])
//# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIm5vZGVfbW9kdWxlcy9icm93c2VyLXBhY2svX3ByZWx1ZGUuanMiLCJwdWJsaWMvanMvY29tbW9uLmpzIiwicHVibGljL2pzL2pvaW4uanMiXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IkFBQUE7QUNBQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTs7QUNsRUE7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBIiwiZmlsZSI6ImdlbmVyYXRlZC5qcyIsInNvdXJjZVJvb3QiOiIiLCJzb3VyY2VzQ29udGVudCI6WyIoZnVuY3Rpb24oKXtmdW5jdGlvbiByKGUsbix0KXtmdW5jdGlvbiBvKGksZil7aWYoIW5baV0pe2lmKCFlW2ldKXt2YXIgYz1cImZ1bmN0aW9uXCI9PXR5cGVvZiByZXF1aXJlJiZyZXF1aXJlO2lmKCFmJiZjKXJldHVybiBjKGksITApO2lmKHUpcmV0dXJuIHUoaSwhMCk7dmFyIGE9bmV3IEVycm9yKFwiQ2Fubm90IGZpbmQgbW9kdWxlICdcIitpK1wiJ1wiKTt0aHJvdyBhLmNvZGU9XCJNT0RVTEVfTk9UX0ZPVU5EXCIsYX12YXIgcD1uW2ldPXtleHBvcnRzOnt9fTtlW2ldWzBdLmNhbGwocC5leHBvcnRzLGZ1bmN0aW9uKHIpe3ZhciBuPWVbaV1bMV1bcl07cmV0dXJuIG8obnx8cil9LHAscC5leHBvcnRzLHIsZSxuLHQpfXJldHVybiBuW2ldLmV4cG9ydHN9Zm9yKHZhciB1PVwiZnVuY3Rpb25cIj09dHlwZW9mIHJlcXVpcmUmJnJlcXVpcmUsaT0wO2k8dC5sZW5ndGg7aSsrKW8odFtpXSk7cmV0dXJuIG99cmV0dXJuIHJ9KSgpIiwiXG5mdW5jdGlvbiBkcmF3Q2lyY2xlKGNpcmNsZSkge1xuICAgIGNvbnN0IGNhbnZhcyA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKCdkcmF3aW5nJyk7XG4gICAgY29uc3QgY3R4ID0gY2FudmFzLmdldENvbnRleHQoJzJkJyk7XG5cbiAgICBjb25zdCB7IGltZ3gsIGltZ3ksIGNlbnRlciwgcmFkaXVzLCBjb2xvciB9ID0gY2lyY2xlO1xuXG4gICAgY29uc3Qgc2NhbGVfeCA9IGNhbnZhcy53aWR0aCAvIGltZ3g7XG4gICAgY29uc3Qgc2NhbGVfeSA9IGNhbnZhcy5oZWlnaHQgLyBpbWd5O1xuICAgIGN0eC5maWxsU3R5bGUgPSBgcmdiYSgke2NvbG9yWzBdfSwke2NvbG9yWzFdfSwke2NvbG9yWzJdfSwke2NvbG9yWzNdLzI1NX1gO1xuICAgIGN0eC5iZWdpblBhdGgoKTtcbiAgICBjdHguZWxsaXBzZShcbiAgICAgICAgY2VudGVyWzBdICogc2NhbGVfeCxcbiAgICAgICAgY2VudGVyWzFdICogc2NhbGVfeSxcbiAgICAgICAgcmFkaXVzICogc2NhbGVfeCxcbiAgICAgICAgcmFkaXVzICogc2NhbGVfeSxcbiAgICAgICAgMCxcbiAgICAgICAgMCxcbiAgICAgICAgMiAqIE1hdGguUElcbiAgICApO1xuICAgIGN0eC5maWxsKCk7XG59XG5cbmZ1bmN0aW9uIGluaXRXZWJzb2NrZXQodXJpKSB7XG4gICAgY29uc3QgY2FudmFzID0gZG9jdW1lbnQuZ2V0RWxlbWVudEJ5SWQoJ2RyYXdpbmcnKTtcbiAgICBjb25zdCBjdHggPSBjYW52YXMuZ2V0Q29udGV4dCgnMmQnKTtcbiAgICBjb25zdCBlcG9jaEJ0biA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKCdlcG9jaCcpO1xuXG4gICAgY29uc29sZS5sb2codXJpKTtcbiAgICBjb25zdCB3cyA9IG5ldyBXZWJTb2NrZXQodXJpKTtcblxuICAgIHdzLm9ub3BlbiA9ICgpID0+IHtjb25zb2xlLmxvZyhcIldlYnNvY2tldCBvcGVuZWRcIik7fTtcbiAgICB3cy5vbm1lc3NhZ2UgPSAobWVzc2FnZSkgPT4ge1xuICAgICAgICBjb25zb2xlLmxvZyhKU09OLnBhcnNlKG1lc3NhZ2UuZGF0YSkpO1xuICAgICAgICBjb25zdCBkYXRhID0gSlNPTi5wYXJzZShtZXNzYWdlLmRhdGEpO1xuICAgICAgICBjb25zdCB0eXBlID0gT2JqZWN0LmtleXMoZGF0YSlbMF07XG4gICAgICAgIGNvbnN0IHBheWxvYWQgPSBkYXRhW3R5cGVdO1xuICAgICAgICBzd2l0Y2ggKHR5cGUpIHtcbiAgICAgICAgICAgIGNhc2UgXCJDaXJjbGVcIjpcbiAgICAgICAgICAgICAgICBkcmF3Q2lyY2xlKHBheWxvYWQpO1xuICAgICAgICAgICAgICAgIGJyZWFrO1xuICAgICAgICAgICAgY2FzZSBcIlJvb21QYXRoXCI6XG4gICAgICAgICAgICAgICAgY29uc3QgbGlua19lbGVtID0gZG9jdW1lbnQuZ2V0RWxlbWVudEJ5SWQoXCJyb29tLWxpbmtcIik7XG4gICAgICAgICAgICAgICAgY29uc3QgbGlua190ZXh0X2VsZW0gPSBkb2N1bWVudC5nZXRFbGVtZW50QnlJZChcInJvb20tbGluay10ZXh0XCIpO1xuXG4gICAgICAgICAgICAgICAgY29uc3QgbGluayA9IGxvY2F0aW9uLmhvc3QgKyBwYXlsb2FkO1xuXG4gICAgICAgICAgICAgICAgbGlua19lbGVtLmhyZWYgPSBsaW5rO1xuICAgICAgICAgICAgICAgIGxpbmtfdGV4dF9lbGVtLmlubmVySFRNTCA9IGxpbms7XG4gICAgICAgICAgICAgICAgYnJlYWs7XG4gICAgICAgICAgICBjYXNlIFwiTmV3SW1hZ2VcIjpcbiAgICAgICAgICAgICAgICBjb25zdCBbd2lkdGgsIGhlaWdodF0gPSBwYXlsb2FkLmRpbWVuc2lvbnM7XG4gICAgICAgICAgICAgICAgY2FudmFzLndpZHRoID0gd2lkdGg7XG4gICAgICAgICAgICAgICAgY2FudmFzLmhlaWdodCA9IGhlaWdodDtcbiAgICAgICAgICAgICAgICBicmVhaztcbiAgICAgICAgfVxuICAgICAgICBjb25zb2xlLmxvZygncmVjZWl2ZWQnLCBtZXNzYWdlLmRhdGEpO1xuICAgIH07XG5cbiAgICByZXR1cm4gd3M7XG59XG5cbm1vZHVsZS5leHBvcnRzID0ge1xuICAgIGRyYXdDaXJjbGUsIFxuICAgIGluaXRXZWJzb2NrZXRcbn07XG4iLCJjb25zdCB7ZHJhd0NpcmNsZSwgaW5pdFdlYnNvY2tldH0gPSByZXF1aXJlKCcuL2NvbW1vbi5qcycpO1xuXG5cbmFzeW5jIGZ1bmN0aW9uIHJ1bigpIHtcbiAgICBjb25zdCByb29tX2lkID0gbG9jYXRpb24ucGF0aG5hbWUuc3BsaXQoJy8nKVsyXTtcbiAgICBjb25zb2xlLmxvZyhyb29tX2lkKTtcbiAgICBjb25zdCB1cmkgPSAnd3M6Ly8nICsgbG9jYXRpb24uaG9zdG5hbWUgKyAnOjMwMDEnICsgJy9qb2luLycgKyByb29tX2lkO1xuICAgIGluaXRXZWJzb2NrZXQodXJpKTtcbn1cblxud2luZG93LmFkZEV2ZW50TGlzdGVuZXIoXCJsb2FkXCIsIGFzeW5jIGZ1bmN0aW9uKCkge1xuICAgIGNvbnNvbGUubG9nKFwibG9hZGVkXCIpO1xuICAgIGF3YWl0IHJ1bigpO1xufSk7XG4iXX0=
