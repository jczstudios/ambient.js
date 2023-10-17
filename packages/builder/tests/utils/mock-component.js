function init() {
    console.log("from init");
}


function exec(messageSource, messageName, messageData) {
    console.log("from exec!");
}

export const guest = {
    init,
    exec
}