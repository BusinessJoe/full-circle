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
const { drawCircle, initWebsocket, sendWsEvent } = require('./common.js');
const { TestStruct } = wasm_bindgen;

function readSingleFile(e) {
    let file = e.target.files[0];
    if (!file) {
        return;
    }

    file.arrayBuffer().then(buffer => {
        worker.postMessage({ type: "init/buffer", payload: buffer });
    });
}

async function initWebWorker(ws) {
    const canvas = document.getElementById('drawing');
    const epochBtn = document.getElementById('epoch');

    await wasm_bindgen('js/pkg/wasm_bg.wasm');

    let worker = new Worker("js/worker.js");
    worker.onmessage = function (e) {
        const { type, payload } = e.data;
        console.log("processing worker event", type);
        switch (type) {
            case "ready":
                const url = "/images/moon.jpeg";
                worker.postMessage({ type: "init/url", payload: url });
                break;
            case "init/done":
                const [width, height] = payload;
                sendWsEvent(ws, "NewImage", { dimensions: [width, height] });
                canvas.width = width;
                canvas.height = height;
                epochBtn.disabled = false;
                break;
            case "epoch/done":
                if (payload) {
                    const { circle, image_data } = payload;
                    // Turn typed arrays (Int32Array, etc.) into normal JS arrays.
                    circle.center = Array.from(circle.center);
                    circle.color = Array.from(circle.color);

                    const message = JSON.stringify({"Circle": circle});
                    sendWsEvent(ws, "Circle", circle);
                } else {
                    console.log("No circle found");
                }
                worker.postMessage({ type: "epoch", payload: { num_gens: 25, gen_size: 100 } });
                break;
            default:
                console.error(`action type '${type}' not recognized`);
                break;
        }
    }

    return worker;
}

async function run() {
    await wasm_bindgen('js/pkg/wasm_bg.wasm');

    const canvas = document.getElementById('drawing');
    const ctx = canvas.getContext('2d');

    const epochBtn = document.getElementById('epoch');

    const uri = 'ws://' + location.hostname + ':3001' + '/room';
    const ws = initWebsocket(uri);

    document.getElementById('file-input')
        .addEventListener('change', readSingleFile, false);

    let worker = await initWebWorker(ws);

    epochBtn.addEventListener("click", () => {
        console.log("starting epoch");
        worker.postMessage({ type: "epoch", payload: { num_gens: 25, gen_size: 100 } });
        //console.log(struct.try_epoch(100, 50));
        //struct.draw(ctx);
    });
}

run();

},{"./common.js":1}]},{},[2])
//# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIm5vZGVfbW9kdWxlcy9icm93c2VyLXBhY2svX3ByZWx1ZGUuanMiLCJwdWJsaWMvanMvY29tbW9uLmpzIiwicHVibGljL2pzL2luZGV4LmpzIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiJBQUFBO0FDQUE7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBOztBQ2hGQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQSIsImZpbGUiOiJnZW5lcmF0ZWQuanMiLCJzb3VyY2VSb290IjoiIiwic291cmNlc0NvbnRlbnQiOlsiKGZ1bmN0aW9uKCl7ZnVuY3Rpb24gcihlLG4sdCl7ZnVuY3Rpb24gbyhpLGYpe2lmKCFuW2ldKXtpZighZVtpXSl7dmFyIGM9XCJmdW5jdGlvblwiPT10eXBlb2YgcmVxdWlyZSYmcmVxdWlyZTtpZighZiYmYylyZXR1cm4gYyhpLCEwKTtpZih1KXJldHVybiB1KGksITApO3ZhciBhPW5ldyBFcnJvcihcIkNhbm5vdCBmaW5kIG1vZHVsZSAnXCIraStcIidcIik7dGhyb3cgYS5jb2RlPVwiTU9EVUxFX05PVF9GT1VORFwiLGF9dmFyIHA9bltpXT17ZXhwb3J0czp7fX07ZVtpXVswXS5jYWxsKHAuZXhwb3J0cyxmdW5jdGlvbihyKXt2YXIgbj1lW2ldWzFdW3JdO3JldHVybiBvKG58fHIpfSxwLHAuZXhwb3J0cyxyLGUsbix0KX1yZXR1cm4gbltpXS5leHBvcnRzfWZvcih2YXIgdT1cImZ1bmN0aW9uXCI9PXR5cGVvZiByZXF1aXJlJiZyZXF1aXJlLGk9MDtpPHQubGVuZ3RoO2krKylvKHRbaV0pO3JldHVybiBvfXJldHVybiByfSkoKSIsIlxuZnVuY3Rpb24gZHJhd0NpcmNsZShjaXJjbGUpIHtcbiAgICBjb25zdCBjYW52YXMgPSBkb2N1bWVudC5nZXRFbGVtZW50QnlJZCgnZHJhd2luZycpO1xuICAgIGNvbnN0IGN0eCA9IGNhbnZhcy5nZXRDb250ZXh0KCcyZCcpO1xuXG4gICAgY29uc3QgeyBpbWd4LCBpbWd5LCBjZW50ZXIsIHJhZGl1cywgY29sb3IgfSA9IGNpcmNsZTtcblxuICAgIGNvbnN0IHNjYWxlX3ggPSBjYW52YXMud2lkdGggLyBpbWd4O1xuICAgIGNvbnN0IHNjYWxlX3kgPSBjYW52YXMuaGVpZ2h0IC8gaW1neTtcbiAgICBjdHguZmlsbFN0eWxlID0gYHJnYmEoJHtjb2xvclswXX0sJHtjb2xvclsxXX0sJHtjb2xvclsyXX0sJHtjb2xvclszXS8yNTV9YDtcbiAgICBjdHguYmVnaW5QYXRoKCk7XG4gICAgY3R4LmVsbGlwc2UoXG4gICAgICAgIGNlbnRlclswXSAqIHNjYWxlX3gsXG4gICAgICAgIGNlbnRlclsxXSAqIHNjYWxlX3ksXG4gICAgICAgIHJhZGl1cyAqIHNjYWxlX3gsXG4gICAgICAgIHJhZGl1cyAqIHNjYWxlX3ksXG4gICAgICAgIDAsXG4gICAgICAgIDAsXG4gICAgICAgIDIgKiBNYXRoLlBJXG4gICAgKTtcbiAgICBjdHguZmlsbCgpO1xufVxuXG5mdW5jdGlvbiBpbml0V2Vic29ja2V0KHVyaSkge1xuICAgIGNvbnN0IGNhbnZhcyA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKCdkcmF3aW5nJyk7XG4gICAgY29uc3QgY3R4ID0gY2FudmFzLmdldENvbnRleHQoJzJkJyk7XG4gICAgY29uc3QgZXBvY2hCdG4gPSBkb2N1bWVudC5nZXRFbGVtZW50QnlJZCgnZXBvY2gnKTtcblxuICAgIGNvbnNvbGUubG9nKHVyaSk7XG4gICAgY29uc3Qgd3MgPSBuZXcgV2ViU29ja2V0KHVyaSk7XG5cbiAgICB3cy5vbm9wZW4gPSAoKSA9PiB7Y29uc29sZS5sb2coXCJXZWJzb2NrZXQgb3BlbmVkXCIpO307XG4gICAgd3Mub25tZXNzYWdlID0gKG1lc3NhZ2UpID0+IHtcbiAgICAgICAgY29uc29sZS5sb2coJ3JlY2VpdmVkIGZyb20gd3M6JywgSlNPTi5wYXJzZShtZXNzYWdlLmRhdGEpKTtcbiAgICAgICAgY29uc3QgZGF0YSA9IEpTT04ucGFyc2UobWVzc2FnZS5kYXRhKTtcbiAgICAgICAgY29uc3QgdHlwZSA9IE9iamVjdC5rZXlzKGRhdGEpWzBdO1xuICAgICAgICBjb25zdCBwYXlsb2FkID0gZGF0YVt0eXBlXTtcbiAgICAgICAgc3dpdGNoICh0eXBlKSB7XG4gICAgICAgICAgICBjYXNlIFwiQ2lyY2xlXCI6XG4gICAgICAgICAgICAgICAgZHJhd0NpcmNsZShwYXlsb2FkKTtcbiAgICAgICAgICAgICAgICBicmVhaztcbiAgICAgICAgICAgIGNhc2UgXCJSb29tUGF0aFwiOlxuICAgICAgICAgICAgICAgIGNvbnN0IGxpbmtfZWxlbSA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKFwicm9vbS1saW5rXCIpO1xuICAgICAgICAgICAgICAgIGNvbnN0IGxpbmtfdGV4dF9lbGVtID0gZG9jdW1lbnQuZ2V0RWxlbWVudEJ5SWQoXCJyb29tLWxpbmstdGV4dFwiKTtcblxuICAgICAgICAgICAgICAgIGNvbnN0IGxpbmsgPSBsb2NhdGlvbi5ob3N0ICsgcGF5bG9hZDtcblxuICAgICAgICAgICAgICAgIGxpbmtfZWxlbS5ocmVmID0gbGluaztcbiAgICAgICAgICAgICAgICBsaW5rX3RleHRfZWxlbS5pbm5lckhUTUwgPSBsaW5rO1xuICAgICAgICAgICAgICAgIGJyZWFrO1xuICAgICAgICAgICAgY2FzZSBcIk5ld0ltYWdlXCI6XG4gICAgICAgICAgICAgICAgY29uc3QgW3dpZHRoLCBoZWlnaHRdID0gcGF5bG9hZC5kaW1lbnNpb25zO1xuICAgICAgICAgICAgICAgIGNhbnZhcy53aWR0aCA9IHdpZHRoO1xuICAgICAgICAgICAgICAgIGNhbnZhcy5oZWlnaHQgPSBoZWlnaHQ7XG4gICAgICAgICAgICAgICAgYnJlYWs7XG4gICAgICAgICAgICBjYXNlIFwiUGxheWVyTGlzdFwiOlxuICAgICAgICAgICAgICAgIGNvbnNvbGUubG9nKHBheWxvYWQpO1xuICAgICAgICAgICAgICAgIGZvciAobGV0IHBsYXllciBvZiBwYXlsb2FkKSB7XG4gICAgICAgICAgICAgICAgICAgIGNvbnNvbGUubG9nKHBsYXllci5pZCk7XG4gICAgICAgICAgICAgICAgfVxuICAgICAgICAgICAgICAgIGJyZWFrO1xuICAgICAgICAgICAgZGVmYXVsdDpcbiAgICAgICAgICAgICAgICBjb25zb2xlLmVycm9yKGBUeXBlICR7dHlwZX0gbm90IHJlY29nbml6ZWRgKTtcbiAgICAgICAgICAgICAgICBicmVhaztcbiAgICAgICAgfVxuICAgIH07XG5cbiAgICByZXR1cm4gd3M7XG59XG5cbmZ1bmN0aW9uIHNlbmRXc0V2ZW50KHdzLCB0eXBlLCBwYXlsb2FkKSB7XG4gICAgY29uc3QgbWVzc2FnZSA9IEpTT04uc3RyaW5naWZ5KHsgW3R5cGVdOiBwYXlsb2FkIH0pO1xuICAgIHdzLnNlbmQobWVzc2FnZSk7XG59XG5cbm1vZHVsZS5leHBvcnRzID0ge1xuICAgIGRyYXdDaXJjbGUsIFxuICAgIGluaXRXZWJzb2NrZXQsXG4gICAgc2VuZFdzRXZlbnQsXG59O1xuIiwiY29uc3QgeyBkcmF3Q2lyY2xlLCBpbml0V2Vic29ja2V0LCBzZW5kV3NFdmVudCB9ID0gcmVxdWlyZSgnLi9jb21tb24uanMnKTtcbmNvbnN0IHsgVGVzdFN0cnVjdCB9ID0gd2FzbV9iaW5kZ2VuO1xuXG5mdW5jdGlvbiByZWFkU2luZ2xlRmlsZShlKSB7XG4gICAgbGV0IGZpbGUgPSBlLnRhcmdldC5maWxlc1swXTtcbiAgICBpZiAoIWZpbGUpIHtcbiAgICAgICAgcmV0dXJuO1xuICAgIH1cblxuICAgIGZpbGUuYXJyYXlCdWZmZXIoKS50aGVuKGJ1ZmZlciA9PiB7XG4gICAgICAgIHdvcmtlci5wb3N0TWVzc2FnZSh7IHR5cGU6IFwiaW5pdC9idWZmZXJcIiwgcGF5bG9hZDogYnVmZmVyIH0pO1xuICAgIH0pO1xufVxuXG5hc3luYyBmdW5jdGlvbiBpbml0V2ViV29ya2VyKHdzKSB7XG4gICAgY29uc3QgY2FudmFzID0gZG9jdW1lbnQuZ2V0RWxlbWVudEJ5SWQoJ2RyYXdpbmcnKTtcbiAgICBjb25zdCBlcG9jaEJ0biA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKCdlcG9jaCcpO1xuXG4gICAgYXdhaXQgd2FzbV9iaW5kZ2VuKCdqcy9wa2cvd2FzbV9iZy53YXNtJyk7XG5cbiAgICBsZXQgd29ya2VyID0gbmV3IFdvcmtlcihcImpzL3dvcmtlci5qc1wiKTtcbiAgICB3b3JrZXIub25tZXNzYWdlID0gZnVuY3Rpb24gKGUpIHtcbiAgICAgICAgY29uc3QgeyB0eXBlLCBwYXlsb2FkIH0gPSBlLmRhdGE7XG4gICAgICAgIGNvbnNvbGUubG9nKFwicHJvY2Vzc2luZyB3b3JrZXIgZXZlbnRcIiwgdHlwZSk7XG4gICAgICAgIHN3aXRjaCAodHlwZSkge1xuICAgICAgICAgICAgY2FzZSBcInJlYWR5XCI6XG4gICAgICAgICAgICAgICAgY29uc3QgdXJsID0gXCIvaW1hZ2VzL21vb24uanBlZ1wiO1xuICAgICAgICAgICAgICAgIHdvcmtlci5wb3N0TWVzc2FnZSh7IHR5cGU6IFwiaW5pdC91cmxcIiwgcGF5bG9hZDogdXJsIH0pO1xuICAgICAgICAgICAgICAgIGJyZWFrO1xuICAgICAgICAgICAgY2FzZSBcImluaXQvZG9uZVwiOlxuICAgICAgICAgICAgICAgIGNvbnN0IFt3aWR0aCwgaGVpZ2h0XSA9IHBheWxvYWQ7XG4gICAgICAgICAgICAgICAgc2VuZFdzRXZlbnQod3MsIFwiTmV3SW1hZ2VcIiwgeyBkaW1lbnNpb25zOiBbd2lkdGgsIGhlaWdodF0gfSk7XG4gICAgICAgICAgICAgICAgY2FudmFzLndpZHRoID0gd2lkdGg7XG4gICAgICAgICAgICAgICAgY2FudmFzLmhlaWdodCA9IGhlaWdodDtcbiAgICAgICAgICAgICAgICBlcG9jaEJ0bi5kaXNhYmxlZCA9IGZhbHNlO1xuICAgICAgICAgICAgICAgIGJyZWFrO1xuICAgICAgICAgICAgY2FzZSBcImVwb2NoL2RvbmVcIjpcbiAgICAgICAgICAgICAgICBpZiAocGF5bG9hZCkge1xuICAgICAgICAgICAgICAgICAgICBjb25zdCB7IGNpcmNsZSwgaW1hZ2VfZGF0YSB9ID0gcGF5bG9hZDtcbiAgICAgICAgICAgICAgICAgICAgLy8gVHVybiB0eXBlZCBhcnJheXMgKEludDMyQXJyYXksIGV0Yy4pIGludG8gbm9ybWFsIEpTIGFycmF5cy5cbiAgICAgICAgICAgICAgICAgICAgY2lyY2xlLmNlbnRlciA9IEFycmF5LmZyb20oY2lyY2xlLmNlbnRlcik7XG4gICAgICAgICAgICAgICAgICAgIGNpcmNsZS5jb2xvciA9IEFycmF5LmZyb20oY2lyY2xlLmNvbG9yKTtcblxuICAgICAgICAgICAgICAgICAgICBjb25zdCBtZXNzYWdlID0gSlNPTi5zdHJpbmdpZnkoe1wiQ2lyY2xlXCI6IGNpcmNsZX0pO1xuICAgICAgICAgICAgICAgICAgICBzZW5kV3NFdmVudCh3cywgXCJDaXJjbGVcIiwgY2lyY2xlKTtcbiAgICAgICAgICAgICAgICB9IGVsc2Uge1xuICAgICAgICAgICAgICAgICAgICBjb25zb2xlLmxvZyhcIk5vIGNpcmNsZSBmb3VuZFwiKTtcbiAgICAgICAgICAgICAgICB9XG4gICAgICAgICAgICAgICAgd29ya2VyLnBvc3RNZXNzYWdlKHsgdHlwZTogXCJlcG9jaFwiLCBwYXlsb2FkOiB7IG51bV9nZW5zOiAyNSwgZ2VuX3NpemU6IDEwMCB9IH0pO1xuICAgICAgICAgICAgICAgIGJyZWFrO1xuICAgICAgICAgICAgZGVmYXVsdDpcbiAgICAgICAgICAgICAgICBjb25zb2xlLmVycm9yKGBhY3Rpb24gdHlwZSAnJHt0eXBlfScgbm90IHJlY29nbml6ZWRgKTtcbiAgICAgICAgICAgICAgICBicmVhaztcbiAgICAgICAgfVxuICAgIH1cblxuICAgIHJldHVybiB3b3JrZXI7XG59XG5cbmFzeW5jIGZ1bmN0aW9uIHJ1bigpIHtcbiAgICBhd2FpdCB3YXNtX2JpbmRnZW4oJ2pzL3BrZy93YXNtX2JnLndhc20nKTtcblxuICAgIGNvbnN0IGNhbnZhcyA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKCdkcmF3aW5nJyk7XG4gICAgY29uc3QgY3R4ID0gY2FudmFzLmdldENvbnRleHQoJzJkJyk7XG5cbiAgICBjb25zdCBlcG9jaEJ0biA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKCdlcG9jaCcpO1xuXG4gICAgY29uc3QgdXJpID0gJ3dzOi8vJyArIGxvY2F0aW9uLmhvc3RuYW1lICsgJzozMDAxJyArICcvcm9vbSc7XG4gICAgY29uc3Qgd3MgPSBpbml0V2Vic29ja2V0KHVyaSk7XG5cbiAgICBkb2N1bWVudC5nZXRFbGVtZW50QnlJZCgnZmlsZS1pbnB1dCcpXG4gICAgICAgIC5hZGRFdmVudExpc3RlbmVyKCdjaGFuZ2UnLCByZWFkU2luZ2xlRmlsZSwgZmFsc2UpO1xuXG4gICAgbGV0IHdvcmtlciA9IGF3YWl0IGluaXRXZWJXb3JrZXIod3MpO1xuXG4gICAgZXBvY2hCdG4uYWRkRXZlbnRMaXN0ZW5lcihcImNsaWNrXCIsICgpID0+IHtcbiAgICAgICAgY29uc29sZS5sb2coXCJzdGFydGluZyBlcG9jaFwiKTtcbiAgICAgICAgd29ya2VyLnBvc3RNZXNzYWdlKHsgdHlwZTogXCJlcG9jaFwiLCBwYXlsb2FkOiB7IG51bV9nZW5zOiAyNSwgZ2VuX3NpemU6IDEwMCB9IH0pO1xuICAgICAgICAvL2NvbnNvbGUubG9nKHN0cnVjdC50cnlfZXBvY2goMTAwLCA1MCkpO1xuICAgICAgICAvL3N0cnVjdC5kcmF3KGN0eCk7XG4gICAgfSk7XG59XG5cbnJ1bigpO1xuIl19
