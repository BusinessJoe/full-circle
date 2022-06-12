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
        const { topic, payload } = JSON.parse(message.data);
        switch (topic) {
            case "circle":
                drawCircle(payload);
                break;
            case "room-link":
                const link_elem = document.getElementById("room-link");
                const link_text_elem = document.getElementById("room-link-text");

                const link = location.host + payload;

                link_elem.href = link;
                link_text_elem.innerHTML = link;
                break;
            case "new-image":
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
//# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIm5vZGVfbW9kdWxlcy9icm93c2VyLXBhY2svX3ByZWx1ZGUuanMiLCJwdWJsaWMvanMvY29tbW9uLmpzIiwicHVibGljL2pzL2pvaW4uanMiXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IkFBQUE7QUNBQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBOztBQ2hFQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0EiLCJmaWxlIjoiZ2VuZXJhdGVkLmpzIiwic291cmNlUm9vdCI6IiIsInNvdXJjZXNDb250ZW50IjpbIihmdW5jdGlvbigpe2Z1bmN0aW9uIHIoZSxuLHQpe2Z1bmN0aW9uIG8oaSxmKXtpZighbltpXSl7aWYoIWVbaV0pe3ZhciBjPVwiZnVuY3Rpb25cIj09dHlwZW9mIHJlcXVpcmUmJnJlcXVpcmU7aWYoIWYmJmMpcmV0dXJuIGMoaSwhMCk7aWYodSlyZXR1cm4gdShpLCEwKTt2YXIgYT1uZXcgRXJyb3IoXCJDYW5ub3QgZmluZCBtb2R1bGUgJ1wiK2krXCInXCIpO3Rocm93IGEuY29kZT1cIk1PRFVMRV9OT1RfRk9VTkRcIixhfXZhciBwPW5baV09e2V4cG9ydHM6e319O2VbaV1bMF0uY2FsbChwLmV4cG9ydHMsZnVuY3Rpb24ocil7dmFyIG49ZVtpXVsxXVtyXTtyZXR1cm4gbyhufHxyKX0scCxwLmV4cG9ydHMscixlLG4sdCl9cmV0dXJuIG5baV0uZXhwb3J0c31mb3IodmFyIHU9XCJmdW5jdGlvblwiPT10eXBlb2YgcmVxdWlyZSYmcmVxdWlyZSxpPTA7aTx0Lmxlbmd0aDtpKyspbyh0W2ldKTtyZXR1cm4gb31yZXR1cm4gcn0pKCkiLCJcbmZ1bmN0aW9uIGRyYXdDaXJjbGUoY2lyY2xlKSB7XG4gICAgY29uc3QgY2FudmFzID0gZG9jdW1lbnQuZ2V0RWxlbWVudEJ5SWQoJ2RyYXdpbmcnKTtcbiAgICBjb25zdCBjdHggPSBjYW52YXMuZ2V0Q29udGV4dCgnMmQnKTtcblxuICAgIGNvbnN0IHsgaW1neCwgaW1neSwgY2VudGVyLCByYWRpdXMsIGNvbG9yIH0gPSBjaXJjbGU7XG5cbiAgICBjb25zdCBzY2FsZV94ID0gY2FudmFzLndpZHRoIC8gaW1neDtcbiAgICBjb25zdCBzY2FsZV95ID0gY2FudmFzLmhlaWdodCAvIGltZ3k7XG4gICAgY3R4LmZpbGxTdHlsZSA9IGByZ2JhKCR7Y29sb3JbMF19LCR7Y29sb3JbMV19LCR7Y29sb3JbMl19LCR7Y29sb3JbM10vMjU1fWA7XG4gICAgY3R4LmJlZ2luUGF0aCgpO1xuICAgIGN0eC5lbGxpcHNlKFxuICAgICAgICBjZW50ZXJbMF0gKiBzY2FsZV94LFxuICAgICAgICBjZW50ZXJbMV0gKiBzY2FsZV95LFxuICAgICAgICByYWRpdXMgKiBzY2FsZV94LFxuICAgICAgICByYWRpdXMgKiBzY2FsZV95LFxuICAgICAgICAwLFxuICAgICAgICAwLFxuICAgICAgICAyICogTWF0aC5QSVxuICAgICk7XG4gICAgY3R4LmZpbGwoKTtcbn1cblxuZnVuY3Rpb24gaW5pdFdlYnNvY2tldCh1cmkpIHtcbiAgICBjb25zdCBjYW52YXMgPSBkb2N1bWVudC5nZXRFbGVtZW50QnlJZCgnZHJhd2luZycpO1xuICAgIGNvbnN0IGN0eCA9IGNhbnZhcy5nZXRDb250ZXh0KCcyZCcpO1xuICAgIGNvbnN0IGVwb2NoQnRuID0gZG9jdW1lbnQuZ2V0RWxlbWVudEJ5SWQoJ2Vwb2NoJyk7XG5cbiAgICBjb25zb2xlLmxvZyh1cmkpO1xuICAgIGNvbnN0IHdzID0gbmV3IFdlYlNvY2tldCh1cmkpO1xuXG4gICAgd3Mub25vcGVuID0gKCkgPT4ge2NvbnNvbGUubG9nKFwiV2Vic29ja2V0IG9wZW5lZFwiKTt9O1xuICAgIHdzLm9ubWVzc2FnZSA9IChtZXNzYWdlKSA9PiB7XG4gICAgICAgIGNvbnNvbGUubG9nKEpTT04ucGFyc2UobWVzc2FnZS5kYXRhKSk7XG4gICAgICAgIGNvbnN0IHsgdG9waWMsIHBheWxvYWQgfSA9IEpTT04ucGFyc2UobWVzc2FnZS5kYXRhKTtcbiAgICAgICAgc3dpdGNoICh0b3BpYykge1xuICAgICAgICAgICAgY2FzZSBcImNpcmNsZVwiOlxuICAgICAgICAgICAgICAgIGRyYXdDaXJjbGUocGF5bG9hZCk7XG4gICAgICAgICAgICAgICAgYnJlYWs7XG4gICAgICAgICAgICBjYXNlIFwicm9vbS1saW5rXCI6XG4gICAgICAgICAgICAgICAgY29uc3QgbGlua19lbGVtID0gZG9jdW1lbnQuZ2V0RWxlbWVudEJ5SWQoXCJyb29tLWxpbmtcIik7XG4gICAgICAgICAgICAgICAgY29uc3QgbGlua190ZXh0X2VsZW0gPSBkb2N1bWVudC5nZXRFbGVtZW50QnlJZChcInJvb20tbGluay10ZXh0XCIpO1xuXG4gICAgICAgICAgICAgICAgY29uc3QgbGluayA9IGxvY2F0aW9uLmhvc3QgKyBwYXlsb2FkO1xuXG4gICAgICAgICAgICAgICAgbGlua19lbGVtLmhyZWYgPSBsaW5rO1xuICAgICAgICAgICAgICAgIGxpbmtfdGV4dF9lbGVtLmlubmVySFRNTCA9IGxpbms7XG4gICAgICAgICAgICAgICAgYnJlYWs7XG4gICAgICAgICAgICBjYXNlIFwibmV3LWltYWdlXCI6XG4gICAgICAgICAgICAgICAgY29uc3QgW3dpZHRoLCBoZWlnaHRdID0gcGF5bG9hZC5kaW1lbnNpb25zO1xuICAgICAgICAgICAgICAgIGNhbnZhcy53aWR0aCA9IHdpZHRoO1xuICAgICAgICAgICAgICAgIGNhbnZhcy5oZWlnaHQgPSBoZWlnaHQ7XG4gICAgICAgICAgICAgICAgYnJlYWs7XG4gICAgICAgIH1cbiAgICAgICAgY29uc29sZS5sb2coJ3JlY2VpdmVkJywgbWVzc2FnZS5kYXRhKTtcbiAgICB9O1xuXG4gICAgcmV0dXJuIHdzO1xufVxuXG5tb2R1bGUuZXhwb3J0cyA9IHtcbiAgICBkcmF3Q2lyY2xlLCBcbiAgICBpbml0V2Vic29ja2V0XG59O1xuIiwiY29uc3Qge2RyYXdDaXJjbGUsIGluaXRXZWJzb2NrZXR9ID0gcmVxdWlyZSgnLi9jb21tb24uanMnKTtcblxuXG5hc3luYyBmdW5jdGlvbiBydW4oKSB7XG4gICAgY29uc3Qgcm9vbV9pZCA9IGxvY2F0aW9uLnBhdGhuYW1lLnNwbGl0KCcvJylbMl07XG4gICAgY29uc29sZS5sb2cocm9vbV9pZCk7XG4gICAgY29uc3QgdXJpID0gJ3dzOi8vJyArIGxvY2F0aW9uLmhvc3RuYW1lICsgJzozMDAxJyArICcvam9pbi8nICsgcm9vbV9pZDtcbiAgICBpbml0V2Vic29ja2V0KHVyaSk7XG59XG5cbndpbmRvdy5hZGRFdmVudExpc3RlbmVyKFwibG9hZFwiLCBhc3luYyBmdW5jdGlvbigpIHtcbiAgICBjb25zb2xlLmxvZyhcImxvYWRlZFwiKTtcbiAgICBhd2FpdCBydW4oKTtcbn0pO1xuIl19
