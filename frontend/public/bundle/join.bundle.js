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
        console.log('received from ws:', JSON.parse(message.data));
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
            case "PlayerList":
                console.log(payload);
                for (let player of payload) {
                    console.log(player.id);
                }
                break;
            default:
                console.error(`Type ${type} not recognized`);
                break;
        }
    };

    return ws;
}

function sendWsEvent(ws, type, payload) {
    const message = JSON.stringify({ [type]: payload });
    ws.send(message);
}

module.exports = {
    drawCircle, 
    initWebsocket,
    sendWsEvent,
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
//# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIm5vZGVfbW9kdWxlcy9icm93c2VyLXBhY2svX3ByZWx1ZGUuanMiLCJwdWJsaWMvanMvY29tbW9uLmpzIiwicHVibGljL2pzL2pvaW4uanMiXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IkFBQUE7QUNBQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7O0FDaEZBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQSIsImZpbGUiOiJnZW5lcmF0ZWQuanMiLCJzb3VyY2VSb290IjoiIiwic291cmNlc0NvbnRlbnQiOlsiKGZ1bmN0aW9uKCl7ZnVuY3Rpb24gcihlLG4sdCl7ZnVuY3Rpb24gbyhpLGYpe2lmKCFuW2ldKXtpZighZVtpXSl7dmFyIGM9XCJmdW5jdGlvblwiPT10eXBlb2YgcmVxdWlyZSYmcmVxdWlyZTtpZighZiYmYylyZXR1cm4gYyhpLCEwKTtpZih1KXJldHVybiB1KGksITApO3ZhciBhPW5ldyBFcnJvcihcIkNhbm5vdCBmaW5kIG1vZHVsZSAnXCIraStcIidcIik7dGhyb3cgYS5jb2RlPVwiTU9EVUxFX05PVF9GT1VORFwiLGF9dmFyIHA9bltpXT17ZXhwb3J0czp7fX07ZVtpXVswXS5jYWxsKHAuZXhwb3J0cyxmdW5jdGlvbihyKXt2YXIgbj1lW2ldWzFdW3JdO3JldHVybiBvKG58fHIpfSxwLHAuZXhwb3J0cyxyLGUsbix0KX1yZXR1cm4gbltpXS5leHBvcnRzfWZvcih2YXIgdT1cImZ1bmN0aW9uXCI9PXR5cGVvZiByZXF1aXJlJiZyZXF1aXJlLGk9MDtpPHQubGVuZ3RoO2krKylvKHRbaV0pO3JldHVybiBvfXJldHVybiByfSkoKSIsIlxuZnVuY3Rpb24gZHJhd0NpcmNsZShjaXJjbGUpIHtcbiAgICBjb25zdCBjYW52YXMgPSBkb2N1bWVudC5nZXRFbGVtZW50QnlJZCgnZHJhd2luZycpO1xuICAgIGNvbnN0IGN0eCA9IGNhbnZhcy5nZXRDb250ZXh0KCcyZCcpO1xuXG4gICAgY29uc3QgeyBpbWd4LCBpbWd5LCBjZW50ZXIsIHJhZGl1cywgY29sb3IgfSA9IGNpcmNsZTtcblxuICAgIGNvbnN0IHNjYWxlX3ggPSBjYW52YXMud2lkdGggLyBpbWd4O1xuICAgIGNvbnN0IHNjYWxlX3kgPSBjYW52YXMuaGVpZ2h0IC8gaW1neTtcbiAgICBjdHguZmlsbFN0eWxlID0gYHJnYmEoJHtjb2xvclswXX0sJHtjb2xvclsxXX0sJHtjb2xvclsyXX0sJHtjb2xvclszXS8yNTV9YDtcbiAgICBjdHguYmVnaW5QYXRoKCk7XG4gICAgY3R4LmVsbGlwc2UoXG4gICAgICAgIGNlbnRlclswXSAqIHNjYWxlX3gsXG4gICAgICAgIGNlbnRlclsxXSAqIHNjYWxlX3ksXG4gICAgICAgIHJhZGl1cyAqIHNjYWxlX3gsXG4gICAgICAgIHJhZGl1cyAqIHNjYWxlX3ksXG4gICAgICAgIDAsXG4gICAgICAgIDAsXG4gICAgICAgIDIgKiBNYXRoLlBJXG4gICAgKTtcbiAgICBjdHguZmlsbCgpO1xufVxuXG5mdW5jdGlvbiBpbml0V2Vic29ja2V0KHVyaSkge1xuICAgIGNvbnN0IGNhbnZhcyA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKCdkcmF3aW5nJyk7XG4gICAgY29uc3QgY3R4ID0gY2FudmFzLmdldENvbnRleHQoJzJkJyk7XG4gICAgY29uc3QgZXBvY2hCdG4gPSBkb2N1bWVudC5nZXRFbGVtZW50QnlJZCgnZXBvY2gnKTtcblxuICAgIGNvbnNvbGUubG9nKHVyaSk7XG4gICAgY29uc3Qgd3MgPSBuZXcgV2ViU29ja2V0KHVyaSk7XG5cbiAgICB3cy5vbm9wZW4gPSAoKSA9PiB7Y29uc29sZS5sb2coXCJXZWJzb2NrZXQgb3BlbmVkXCIpO307XG4gICAgd3Mub25tZXNzYWdlID0gKG1lc3NhZ2UpID0+IHtcbiAgICAgICAgY29uc29sZS5sb2coJ3JlY2VpdmVkIGZyb20gd3M6JywgSlNPTi5wYXJzZShtZXNzYWdlLmRhdGEpKTtcbiAgICAgICAgY29uc3QgZGF0YSA9IEpTT04ucGFyc2UobWVzc2FnZS5kYXRhKTtcbiAgICAgICAgY29uc3QgdHlwZSA9IE9iamVjdC5rZXlzKGRhdGEpWzBdO1xuICAgICAgICBjb25zdCBwYXlsb2FkID0gZGF0YVt0eXBlXTtcbiAgICAgICAgc3dpdGNoICh0eXBlKSB7XG4gICAgICAgICAgICBjYXNlIFwiQ2lyY2xlXCI6XG4gICAgICAgICAgICAgICAgZHJhd0NpcmNsZShwYXlsb2FkKTtcbiAgICAgICAgICAgICAgICBicmVhaztcbiAgICAgICAgICAgIGNhc2UgXCJSb29tUGF0aFwiOlxuICAgICAgICAgICAgICAgIGNvbnN0IGxpbmtfZWxlbSA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKFwicm9vbS1saW5rXCIpO1xuICAgICAgICAgICAgICAgIGNvbnN0IGxpbmtfdGV4dF9lbGVtID0gZG9jdW1lbnQuZ2V0RWxlbWVudEJ5SWQoXCJyb29tLWxpbmstdGV4dFwiKTtcblxuICAgICAgICAgICAgICAgIGNvbnN0IGxpbmsgPSBsb2NhdGlvbi5ob3N0ICsgcGF5bG9hZDtcblxuICAgICAgICAgICAgICAgIGxpbmtfZWxlbS5ocmVmID0gbGluaztcbiAgICAgICAgICAgICAgICBsaW5rX3RleHRfZWxlbS5pbm5lckhUTUwgPSBsaW5rO1xuICAgICAgICAgICAgICAgIGJyZWFrO1xuICAgICAgICAgICAgY2FzZSBcIk5ld0ltYWdlXCI6XG4gICAgICAgICAgICAgICAgY29uc3QgW3dpZHRoLCBoZWlnaHRdID0gcGF5bG9hZC5kaW1lbnNpb25zO1xuICAgICAgICAgICAgICAgIGNhbnZhcy53aWR0aCA9IHdpZHRoO1xuICAgICAgICAgICAgICAgIGNhbnZhcy5oZWlnaHQgPSBoZWlnaHQ7XG4gICAgICAgICAgICAgICAgYnJlYWs7XG4gICAgICAgICAgICBjYXNlIFwiUGxheWVyTGlzdFwiOlxuICAgICAgICAgICAgICAgIGNvbnNvbGUubG9nKHBheWxvYWQpO1xuICAgICAgICAgICAgICAgIGZvciAobGV0IHBsYXllciBvZiBwYXlsb2FkKSB7XG4gICAgICAgICAgICAgICAgICAgIGNvbnNvbGUubG9nKHBsYXllci5pZCk7XG4gICAgICAgICAgICAgICAgfVxuICAgICAgICAgICAgICAgIGJyZWFrO1xuICAgICAgICAgICAgZGVmYXVsdDpcbiAgICAgICAgICAgICAgICBjb25zb2xlLmVycm9yKGBUeXBlICR7dHlwZX0gbm90IHJlY29nbml6ZWRgKTtcbiAgICAgICAgICAgICAgICBicmVhaztcbiAgICAgICAgfVxuICAgIH07XG5cbiAgICByZXR1cm4gd3M7XG59XG5cbmZ1bmN0aW9uIHNlbmRXc0V2ZW50KHdzLCB0eXBlLCBwYXlsb2FkKSB7XG4gICAgY29uc3QgbWVzc2FnZSA9IEpTT04uc3RyaW5naWZ5KHsgW3R5cGVdOiBwYXlsb2FkIH0pO1xuICAgIHdzLnNlbmQobWVzc2FnZSk7XG59XG5cbm1vZHVsZS5leHBvcnRzID0ge1xuICAgIGRyYXdDaXJjbGUsIFxuICAgIGluaXRXZWJzb2NrZXQsXG4gICAgc2VuZFdzRXZlbnQsXG59O1xuIiwiY29uc3Qge2RyYXdDaXJjbGUsIGluaXRXZWJzb2NrZXR9ID0gcmVxdWlyZSgnLi9jb21tb24uanMnKTtcblxuXG5hc3luYyBmdW5jdGlvbiBydW4oKSB7XG4gICAgY29uc3Qgcm9vbV9pZCA9IGxvY2F0aW9uLnBhdGhuYW1lLnNwbGl0KCcvJylbMl07XG4gICAgY29uc29sZS5sb2cocm9vbV9pZCk7XG4gICAgY29uc3QgdXJpID0gJ3dzOi8vJyArIGxvY2F0aW9uLmhvc3RuYW1lICsgJzozMDAxJyArICcvam9pbi8nICsgcm9vbV9pZDtcbiAgICBpbml0V2Vic29ja2V0KHVyaSk7XG59XG5cbndpbmRvdy5hZGRFdmVudExpc3RlbmVyKFwibG9hZFwiLCBhc3luYyBmdW5jdGlvbigpIHtcbiAgICBjb25zb2xlLmxvZyhcImxvYWRlZFwiKTtcbiAgICBhd2FpdCBydW4oKTtcbn0pO1xuIl19
