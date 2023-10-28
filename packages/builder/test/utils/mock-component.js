import {
    dropInputStream,
    dropOutputStream,
    flush,
    read,
    subscribeToInputStream,
    write,
  } from 'wasi:io/streams';

function init() {
    
}


function exec(messageSource, messageName, messageData) {
    // console.log("from exec!");
}

export const guest = {
    init,
    exec
}