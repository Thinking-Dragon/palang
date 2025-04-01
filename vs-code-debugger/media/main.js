const vscode = acquireVsCodeApi();

function runTask(kind, path, file) {
    const safePath = path.replace(/\//g, '-').toString();
    document.getElementById(`btn-run-${safePath}`).innerHTML = '<i class="p-icon--spinner u-animation--spin is-light"></i>';

    const profile = document.getElementById('profile-select').value;
    const arguments = Array.prototype.map.call(document.querySelectorAll(`.parameter-${safePath}`), element => element.value);

    vscode.postMessage({ command: 'run', kind, path, file, arguments, profile });
}
