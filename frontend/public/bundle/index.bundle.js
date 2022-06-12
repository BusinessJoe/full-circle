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
        console.log("processing event", type);
        switch (type) {
            case "ready":
                const url = "/images/moon.jpeg";
                worker.postMessage({ type: "init/url", payload: url });
                break;
            case "init/done":
                const [width, height] = payload;
                console.log(width, height);
                ws.send(JSON.stringify({
                    topic: "new-image",
                    payload: {
                        dimensions: [width, height]
                    }
                }));
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

                    const message = JSON.stringify({topic: "circle", payload: circle});
                    console.log(circle);
                    console.log(message);
                    ws.send(message);
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
//# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIm5vZGVfbW9kdWxlcy9icm93c2VyLXBhY2svX3ByZWx1ZGUuanMiLCJwdWJsaWMvanMvY29tbW9uLmpzIiwicHVibGljL2pzL2luZGV4LmpzIl0sIm5hbWVzIjpbXSwibWFwcGluZ3MiOiJBQUFBO0FDQUE7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTs7QUNoRUE7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBIiwiZmlsZSI6ImdlbmVyYXRlZC5qcyIsInNvdXJjZVJvb3QiOiIiLCJzb3VyY2VzQ29udGVudCI6WyIoZnVuY3Rpb24oKXtmdW5jdGlvbiByKGUsbix0KXtmdW5jdGlvbiBvKGksZil7aWYoIW5baV0pe2lmKCFlW2ldKXt2YXIgYz1cImZ1bmN0aW9uXCI9PXR5cGVvZiByZXF1aXJlJiZyZXF1aXJlO2lmKCFmJiZjKXJldHVybiBjKGksITApO2lmKHUpcmV0dXJuIHUoaSwhMCk7dmFyIGE9bmV3IEVycm9yKFwiQ2Fubm90IGZpbmQgbW9kdWxlICdcIitpK1wiJ1wiKTt0aHJvdyBhLmNvZGU9XCJNT0RVTEVfTk9UX0ZPVU5EXCIsYX12YXIgcD1uW2ldPXtleHBvcnRzOnt9fTtlW2ldWzBdLmNhbGwocC5leHBvcnRzLGZ1bmN0aW9uKHIpe3ZhciBuPWVbaV1bMV1bcl07cmV0dXJuIG8obnx8cil9LHAscC5leHBvcnRzLHIsZSxuLHQpfXJldHVybiBuW2ldLmV4cG9ydHN9Zm9yKHZhciB1PVwiZnVuY3Rpb25cIj09dHlwZW9mIHJlcXVpcmUmJnJlcXVpcmUsaT0wO2k8dC5sZW5ndGg7aSsrKW8odFtpXSk7cmV0dXJuIG99cmV0dXJuIHJ9KSgpIiwiXG5mdW5jdGlvbiBkcmF3Q2lyY2xlKGNpcmNsZSkge1xuICAgIGNvbnN0IGNhbnZhcyA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKCdkcmF3aW5nJyk7XG4gICAgY29uc3QgY3R4ID0gY2FudmFzLmdldENvbnRleHQoJzJkJyk7XG5cbiAgICBjb25zdCB7IGltZ3gsIGltZ3ksIGNlbnRlciwgcmFkaXVzLCBjb2xvciB9ID0gY2lyY2xlO1xuXG4gICAgY29uc3Qgc2NhbGVfeCA9IGNhbnZhcy53aWR0aCAvIGltZ3g7XG4gICAgY29uc3Qgc2NhbGVfeSA9IGNhbnZhcy5oZWlnaHQgLyBpbWd5O1xuICAgIGN0eC5maWxsU3R5bGUgPSBgcmdiYSgke2NvbG9yWzBdfSwke2NvbG9yWzFdfSwke2NvbG9yWzJdfSwke2NvbG9yWzNdLzI1NX1gO1xuICAgIGN0eC5iZWdpblBhdGgoKTtcbiAgICBjdHguZWxsaXBzZShcbiAgICAgICAgY2VudGVyWzBdICogc2NhbGVfeCxcbiAgICAgICAgY2VudGVyWzFdICogc2NhbGVfeSxcbiAgICAgICAgcmFkaXVzICogc2NhbGVfeCxcbiAgICAgICAgcmFkaXVzICogc2NhbGVfeSxcbiAgICAgICAgMCxcbiAgICAgICAgMCxcbiAgICAgICAgMiAqIE1hdGguUElcbiAgICApO1xuICAgIGN0eC5maWxsKCk7XG59XG5cbmZ1bmN0aW9uIGluaXRXZWJzb2NrZXQodXJpKSB7XG4gICAgY29uc3QgY2FudmFzID0gZG9jdW1lbnQuZ2V0RWxlbWVudEJ5SWQoJ2RyYXdpbmcnKTtcbiAgICBjb25zdCBjdHggPSBjYW52YXMuZ2V0Q29udGV4dCgnMmQnKTtcbiAgICBjb25zdCBlcG9jaEJ0biA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKCdlcG9jaCcpO1xuXG4gICAgY29uc29sZS5sb2codXJpKTtcbiAgICBjb25zdCB3cyA9IG5ldyBXZWJTb2NrZXQodXJpKTtcblxuICAgIHdzLm9ub3BlbiA9ICgpID0+IHtjb25zb2xlLmxvZyhcIldlYnNvY2tldCBvcGVuZWRcIik7fTtcbiAgICB3cy5vbm1lc3NhZ2UgPSAobWVzc2FnZSkgPT4ge1xuICAgICAgICBjb25zb2xlLmxvZyhKU09OLnBhcnNlKG1lc3NhZ2UuZGF0YSkpO1xuICAgICAgICBjb25zdCB7IHRvcGljLCBwYXlsb2FkIH0gPSBKU09OLnBhcnNlKG1lc3NhZ2UuZGF0YSk7XG4gICAgICAgIHN3aXRjaCAodG9waWMpIHtcbiAgICAgICAgICAgIGNhc2UgXCJjaXJjbGVcIjpcbiAgICAgICAgICAgICAgICBkcmF3Q2lyY2xlKHBheWxvYWQpO1xuICAgICAgICAgICAgICAgIGJyZWFrO1xuICAgICAgICAgICAgY2FzZSBcInJvb20tbGlua1wiOlxuICAgICAgICAgICAgICAgIGNvbnN0IGxpbmtfZWxlbSA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKFwicm9vbS1saW5rXCIpO1xuICAgICAgICAgICAgICAgIGNvbnN0IGxpbmtfdGV4dF9lbGVtID0gZG9jdW1lbnQuZ2V0RWxlbWVudEJ5SWQoXCJyb29tLWxpbmstdGV4dFwiKTtcblxuICAgICAgICAgICAgICAgIGNvbnN0IGxpbmsgPSBsb2NhdGlvbi5ob3N0ICsgcGF5bG9hZDtcblxuICAgICAgICAgICAgICAgIGxpbmtfZWxlbS5ocmVmID0gbGluaztcbiAgICAgICAgICAgICAgICBsaW5rX3RleHRfZWxlbS5pbm5lckhUTUwgPSBsaW5rO1xuICAgICAgICAgICAgICAgIGJyZWFrO1xuICAgICAgICAgICAgY2FzZSBcIm5ldy1pbWFnZVwiOlxuICAgICAgICAgICAgICAgIGNvbnN0IFt3aWR0aCwgaGVpZ2h0XSA9IHBheWxvYWQuZGltZW5zaW9ucztcbiAgICAgICAgICAgICAgICBjYW52YXMud2lkdGggPSB3aWR0aDtcbiAgICAgICAgICAgICAgICBjYW52YXMuaGVpZ2h0ID0gaGVpZ2h0O1xuICAgICAgICAgICAgICAgIGJyZWFrO1xuICAgICAgICB9XG4gICAgICAgIGNvbnNvbGUubG9nKCdyZWNlaXZlZCcsIG1lc3NhZ2UuZGF0YSk7XG4gICAgfTtcblxuICAgIHJldHVybiB3cztcbn1cblxubW9kdWxlLmV4cG9ydHMgPSB7XG4gICAgZHJhd0NpcmNsZSwgXG4gICAgaW5pdFdlYnNvY2tldFxufTtcbiIsImNvbnN0IHtkcmF3Q2lyY2xlLCBpbml0V2Vic29ja2V0fSA9IHJlcXVpcmUoJy4vY29tbW9uLmpzJyk7XG5jb25zdCB7IFRlc3RTdHJ1Y3QgfSA9IHdhc21fYmluZGdlbjtcblxuZnVuY3Rpb24gcmVhZFNpbmdsZUZpbGUoZSkge1xuICAgIGxldCBmaWxlID0gZS50YXJnZXQuZmlsZXNbMF07XG4gICAgaWYgKCFmaWxlKSB7XG4gICAgICAgIHJldHVybjtcbiAgICB9XG5cbiAgICBmaWxlLmFycmF5QnVmZmVyKCkudGhlbihidWZmZXIgPT4ge1xuICAgICAgICB3b3JrZXIucG9zdE1lc3NhZ2UoeyB0eXBlOiBcImluaXQvYnVmZmVyXCIsIHBheWxvYWQ6IGJ1ZmZlciB9KTtcbiAgICB9KTtcbn1cblxuYXN5bmMgZnVuY3Rpb24gaW5pdFdlYldvcmtlcih3cykge1xuICAgIGNvbnN0IGNhbnZhcyA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKCdkcmF3aW5nJyk7XG4gICAgY29uc3QgZXBvY2hCdG4gPSBkb2N1bWVudC5nZXRFbGVtZW50QnlJZCgnZXBvY2gnKTtcblxuICAgIGF3YWl0IHdhc21fYmluZGdlbignanMvcGtnL3dhc21fYmcud2FzbScpO1xuXG4gICAgbGV0IHdvcmtlciA9IG5ldyBXb3JrZXIoXCJqcy93b3JrZXIuanNcIik7XG4gICAgd29ya2VyLm9ubWVzc2FnZSA9IGZ1bmN0aW9uIChlKSB7XG4gICAgICAgIGNvbnN0IHsgdHlwZSwgcGF5bG9hZCB9ID0gZS5kYXRhO1xuICAgICAgICBjb25zb2xlLmxvZyhcInByb2Nlc3NpbmcgZXZlbnRcIiwgdHlwZSk7XG4gICAgICAgIHN3aXRjaCAodHlwZSkge1xuICAgICAgICAgICAgY2FzZSBcInJlYWR5XCI6XG4gICAgICAgICAgICAgICAgY29uc3QgdXJsID0gXCIvaW1hZ2VzL21vb24uanBlZ1wiO1xuICAgICAgICAgICAgICAgIHdvcmtlci5wb3N0TWVzc2FnZSh7IHR5cGU6IFwiaW5pdC91cmxcIiwgcGF5bG9hZDogdXJsIH0pO1xuICAgICAgICAgICAgICAgIGJyZWFrO1xuICAgICAgICAgICAgY2FzZSBcImluaXQvZG9uZVwiOlxuICAgICAgICAgICAgICAgIGNvbnN0IFt3aWR0aCwgaGVpZ2h0XSA9IHBheWxvYWQ7XG4gICAgICAgICAgICAgICAgY29uc29sZS5sb2cod2lkdGgsIGhlaWdodCk7XG4gICAgICAgICAgICAgICAgd3Muc2VuZChKU09OLnN0cmluZ2lmeSh7XG4gICAgICAgICAgICAgICAgICAgIHRvcGljOiBcIm5ldy1pbWFnZVwiLFxuICAgICAgICAgICAgICAgICAgICBwYXlsb2FkOiB7XG4gICAgICAgICAgICAgICAgICAgICAgICBkaW1lbnNpb25zOiBbd2lkdGgsIGhlaWdodF1cbiAgICAgICAgICAgICAgICAgICAgfVxuICAgICAgICAgICAgICAgIH0pKTtcbiAgICAgICAgICAgICAgICBjYW52YXMud2lkdGggPSB3aWR0aDtcbiAgICAgICAgICAgICAgICBjYW52YXMuaGVpZ2h0ID0gaGVpZ2h0O1xuICAgICAgICAgICAgICAgIGVwb2NoQnRuLmRpc2FibGVkID0gZmFsc2U7XG4gICAgICAgICAgICAgICAgYnJlYWs7XG4gICAgICAgICAgICBjYXNlIFwiZXBvY2gvZG9uZVwiOlxuICAgICAgICAgICAgICAgIGlmIChwYXlsb2FkKSB7XG4gICAgICAgICAgICAgICAgICAgIGNvbnN0IHsgY2lyY2xlLCBpbWFnZV9kYXRhIH0gPSBwYXlsb2FkO1xuICAgICAgICAgICAgICAgICAgICAvLyBUdXJuIHR5cGVkIGFycmF5cyAoSW50MzJBcnJheSwgZXRjLikgaW50byBub3JtYWwgSlMgYXJyYXlzLlxuICAgICAgICAgICAgICAgICAgICBjaXJjbGUuY2VudGVyID0gQXJyYXkuZnJvbShjaXJjbGUuY2VudGVyKTtcbiAgICAgICAgICAgICAgICAgICAgY2lyY2xlLmNvbG9yID0gQXJyYXkuZnJvbShjaXJjbGUuY29sb3IpO1xuXG4gICAgICAgICAgICAgICAgICAgIGNvbnN0IG1lc3NhZ2UgPSBKU09OLnN0cmluZ2lmeSh7dG9waWM6IFwiY2lyY2xlXCIsIHBheWxvYWQ6IGNpcmNsZX0pO1xuICAgICAgICAgICAgICAgICAgICBjb25zb2xlLmxvZyhjaXJjbGUpO1xuICAgICAgICAgICAgICAgICAgICBjb25zb2xlLmxvZyhtZXNzYWdlKTtcbiAgICAgICAgICAgICAgICAgICAgd3Muc2VuZChtZXNzYWdlKTtcbiAgICAgICAgICAgICAgICB9IGVsc2Uge1xuICAgICAgICAgICAgICAgICAgICBjb25zb2xlLmxvZyhcIk5vIGNpcmNsZSBmb3VuZFwiKTtcbiAgICAgICAgICAgICAgICB9XG4gICAgICAgICAgICAgICAgd29ya2VyLnBvc3RNZXNzYWdlKHsgdHlwZTogXCJlcG9jaFwiLCBwYXlsb2FkOiB7IG51bV9nZW5zOiAyNSwgZ2VuX3NpemU6IDEwMCB9IH0pO1xuICAgICAgICAgICAgICAgIGJyZWFrO1xuICAgICAgICAgICAgZGVmYXVsdDpcbiAgICAgICAgICAgICAgICBjb25zb2xlLmVycm9yKGBhY3Rpb24gdHlwZSAnJHt0eXBlfScgbm90IHJlY29nbml6ZWRgKTtcbiAgICAgICAgICAgICAgICBicmVhaztcbiAgICAgICAgfVxuICAgIH1cblxuICAgIHJldHVybiB3b3JrZXI7XG59XG5cbmFzeW5jIGZ1bmN0aW9uIHJ1bigpIHtcbiAgICBhd2FpdCB3YXNtX2JpbmRnZW4oJ2pzL3BrZy93YXNtX2JnLndhc20nKTtcblxuICAgIGNvbnN0IGNhbnZhcyA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKCdkcmF3aW5nJyk7XG4gICAgY29uc3QgY3R4ID0gY2FudmFzLmdldENvbnRleHQoJzJkJyk7XG5cbiAgICBjb25zdCBlcG9jaEJ0biA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKCdlcG9jaCcpO1xuXG4gICAgY29uc3QgdXJpID0gJ3dzOi8vJyArIGxvY2F0aW9uLmhvc3RuYW1lICsgJzozMDAxJyArICcvcm9vbSc7XG4gICAgY29uc3Qgd3MgPSBpbml0V2Vic29ja2V0KHVyaSk7XG5cbiAgICBkb2N1bWVudC5nZXRFbGVtZW50QnlJZCgnZmlsZS1pbnB1dCcpXG4gICAgICAgIC5hZGRFdmVudExpc3RlbmVyKCdjaGFuZ2UnLCByZWFkU2luZ2xlRmlsZSwgZmFsc2UpO1xuXG4gICAgbGV0IHdvcmtlciA9IGF3YWl0IGluaXRXZWJXb3JrZXIod3MpO1xuXG4gICAgZXBvY2hCdG4uYWRkRXZlbnRMaXN0ZW5lcihcImNsaWNrXCIsICgpID0+IHtcbiAgICAgICAgY29uc29sZS5sb2coXCJzdGFydGluZyBlcG9jaFwiKTtcbiAgICAgICAgd29ya2VyLnBvc3RNZXNzYWdlKHsgdHlwZTogXCJlcG9jaFwiLCBwYXlsb2FkOiB7IG51bV9nZW5zOiAyNSwgZ2VuX3NpemU6IDEwMCB9IH0pO1xuICAgICAgICAvL2NvbnNvbGUubG9nKHN0cnVjdC50cnlfZXBvY2goMTAwLCA1MCkpO1xuICAgICAgICAvL3N0cnVjdC5kcmF3KGN0eCk7XG4gICAgfSk7XG59XG5cbnJ1bigpO1xuIl19
