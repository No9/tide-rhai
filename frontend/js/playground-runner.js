// FIXME: We could have used worker-loader but it currently breaks with WASM.
//        workerize-loader has the issue fixed. We sould want to switch back
//        to worker-loader once the issue is fixed.
//        Blocked on: https://github.com/webpack-contrib/worker-loader/pull/175
import MyWorker from "workerize-loader!./worker.js";

let workerLoader = (function () {
    /**
     * @type Worker?
     */
    let worker = null;
    /**
     * @type Promise<Worker>?
     */
    let workerPromise = null;

    let rejectWorkerLoad = null;

    function ensureWorker() {
        if (workerPromise === null) {
            workerPromise = new Promise((resolve, reject) => {
                worker = new MyWorker();
                rejectWorkerLoad = reject;
                worker.onerror = ev => {
                    workerPromise = null;
                    rejectWorkerLoad = null;
                    console.log("An error occured in the Worker when loading:", ev);
                    reject("An error occured in the Worker when loading: " + ev.message);
                };
                const msgListener = ev => {
                    if (ev.data.req === "_ready") {
                        worker.removeEventListener("message", msgListener);
                        worker.onerror = ev => {
                            worker = null;
                            workerPromise = null;
                            console.error("An error occured in the Worker:", ev);
                        };
                        rejectWorkerLoad = null;
                        resolve(worker);
                    }
                };
                worker.addEventListener("message", msgListener);
            });
        }
        return workerPromise;
    }

    function terminateLoadedWorker() {
        if (isWorkerLoaded()) {
            worker.terminate();
            worker = null;
            workerPromise = null;
        }
    }

    function terminateLoadingWorker() {
        if (isWorkerLoading()) {
            worker.terminate();
            worker = null;
            workerPromise = null;
            const r = rejectWorkerLoad;
            rejectWorkerLoad = null;
            r("Loading of Worker stopped.");
        }
    }

    function terminateWorker() {
        if (isWorkerLoaded()) {
            terminateLoadedWorker();
        } else if (isWorkerLoading()) {
            terminateLoadingWorker();
        }
    }

    function isWorkerLoaded() {
        return worker !== null && rejectWorkerLoad === null;
    }

    function isWorkerLoading() {
        return worker !== null && rejectWorkerLoad !== null;
    }

    return {
        ensureWorker,
        terminateWorker,
    }
})();

let runScriptMessageListener = null;
let runScriptPromiseReject = null;

/**
 * @callback AppendOutputCallback
 * @param {string} line
 */

/**
 * 
 * @param {string} script
 * @param {AppendOutputCallback} appendOutput
 * @param {(Number) => void} updateOps
 * @returns {Promise<void>}
 */
function runScript(script, appendOutput, updateOps, message) {
    if (runScriptMessageListener) {
        return Promise.reject("Another script is running.");
    }
    return new Promise((resolve, reject) => {
        appendOutput(`Saving and running script...`);
        
        var savemessage = { "content" : script };
        fetch('/save', {
            method: 'post',
            body: JSON.stringify(savemessage)
            
        }).then(function(response) {
            return response.json();
        }).then(function(data) {
            let messageObj = JSON.parse(message.replace(/\s/g, ''));
            appendOutput(`Upload Finished ${message} at ${new Date().toISOString()}`);
            appendOutput("Saved:" + JSON.stringify(data));
            fetch('/', {
            method: 'post',
            body: message,
            headers: {
                "Content-Type" : messageObj.datacontenttype,
                "ce-specversion" : messageObj.specversion,
                "ce-id": messageObj.id,
                "ce-type": messageObj.type,
                "ce-source": messageObj.source,
            }
            }).then(function(response) {
                return response.json();
            }).then(function(data) {
                appendOutput(`Execute Finished at ${new Date().toISOString()}`);
                appendOutput(JSON.stringify(data));
                runScriptMessageListener = null;
                runScriptPromiseReject = null;
                resolve();    
            });
        });
    //     workerLoader.ensureWorker().then(worker => {
    //         updateOps(0);
    //         appendOutput(`Running script at ${new Date().toISOString()}\n`);
    //         worker.addEventListener("message", runScriptMessageListener = ev => {
    //             if (ev.data.req === "runScript/output") {
    //                 appendOutput(ev.data.output);
    //             } else if (ev.data.req === "runScript/end") {
    //                 appendOutput(`Finished at ${new Date().toISOString()}`);
    //                 worker.removeEventListener("message", runScriptMessageListener);
    //                 runScriptMessageListener = null;
    //                 runScriptPromiseReject = null;
    //                 resolve();
    //             } else if (ev.data.req === "runScript/updateOps") {
    //                 updateOps(ev.data.ops);
    //             }
    //         })
    //         runScriptPromiseReject = reject;
    //         worker.postMessage({ req: "runScript", script });
    //     }).catch(e => {
    //         reject("Cannot load Worker: " + e);
    //     });
    });
}

function stopScript() {
    workerLoader.terminateWorker();
    if (runScriptPromiseReject) {
        runScriptPromiseReject("Script execution stopped.");
        runScriptMessageListener = null;
        runScriptPromiseReject = null;
    }
}

export { runScript, stopScript };
