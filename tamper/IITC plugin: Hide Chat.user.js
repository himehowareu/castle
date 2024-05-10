// ==UserScript==
// @id             iitc-plugin-hide-chat
// @name           IITC plugin: Hide Chat
// @category       Tweaks
// @version        0.1.0
// @description    Hides the chat.
// @include        https://*.ingress.com/intel*
// @include        http://*.ingress.com/intel*
// @match          https://*.ingress.com/intel*
// @match          http://*.ingress.com/intel*
// @include        https://*.ingress.com/mission/*
// @include        http://*.ingress.com/mission/*
// @match          https://*.ingress.com/mission/*
// @match          http://*.ingress.com/mission/*
// @grant          none
// ==/UserScript==

// Doesn't actually use any IITC features.

{
    const run = () => setTimeout(() => Array.from(document.querySelectorAll('#chatcontrols, #chat, #chatinput')).forEach(element => element.style.display = 'none'), 200)
    if (['complete', 'loaded', 'interactive'].includes(document.readyState)) run()
    else document.addEventListener('DOMContentLoaded', run)
}