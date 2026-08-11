function requestFullscreen(element) {
    if (element.requestFullscreen) {
        return element.requestFullscreen();
    } else if (element.webkitRequestFullScreen) {
        return element.webkitRequestFullScreen();
    } else if (element.mozRequestFullScreen) {
        return element.mozRequestFullScreen();
    } else if (element.msRequestFullscreen) {
        return element.msRequestFullscreen();
    } else {
        return Promise.reject(new Error('Fullscreen not supported in request..'));
    }
}
function exitFullscreen() {
    if (document.exitFullscreen) {
        return document.exitFullscreen();
    } else if (document.webkitExitFullscreen) {
        return document.webkitExitFullscreen();
    } else if (document.mozCancelFullScreen) {
        return document.mozCancelFullScreen();
    } else if (document.msExitFullscreen) {
        return document.msExitFullscreen();
    } else {
        return Promise.reject(new Error('Fullscreen not supported in exit..'));
    }
}
function isFullscreen() {
    return !!(document.fullscreenElement || document.webkitFullscreenElement || document.mozFullScreenElement || document.msFullscreenElement);
}
async function willFullscreen(isFullscreenValue) {
    const res = await fetch(`/api/set_full_screen` , {
        method: 'PUT',
        body: isFullscreenValue ? "1" : "0"
    });
    if (res.status === 200) {
        return true;
    } else if (res.status === 202) {
        if (isFullscreen()) {
            exitFullscreen();
        } else {
            requestFullscreen(document.documentElement);
        }
        return true;
    } else {
        return false;
    }
}
async function exitWindow() {
    const res = await fetch(`/api/exit_window` , {
        method: 'PUT'
    });
    if (res.status === 200) {
        return true;
    } else {
        return false;
    }
}
