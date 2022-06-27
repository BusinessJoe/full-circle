function fileToArrayBuffer(file) {
    return new Promise((resolve, reject) => {
        let fileReader = new FileReader();

        fileReader.onerror = (event) => {
            reject(event.target.error);
        };

        fileReader.onload = (event) => {
            resolve(event.target.result);
        };

        fileReader.readAsArrayBuffer(file);
    });
}

function arrayBufferToBase64( buffer ) {
    var binary = '';
    var bytes = new Uint8Array( buffer );
    var len = bytes.byteLength;
    for (var i = 0; i < len; i++) {
        binary += String.fromCharCode( bytes[ i ] );
    }
    return window.btoa( binary );
}

module.exports = { fileToArrayBuffer, arrayBufferToBase64 };
