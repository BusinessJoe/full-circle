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

module.exports = { fileToArrayBuffer };
