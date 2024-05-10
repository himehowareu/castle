// ==UserScript==
// @id              Easy-Ingress-Scanner-Button
// @name            IITC Plugin: Easy Ingress Scanner Button
// @category        Info
// @version         0.2
// @namespace       https://github.com/Ayaro1/IITC-Easy-Ingress-Scanner-Button/
// @updateURL       https://github.com/Ayaro1/IITC-Easy-Ingress-Scanner-Button/raw/main/EasyIngressScannerButton.user.js
// @downloadURL     https://github.com/Ayaro1/IITC-Easy-Ingress-Scanner-Button/raw/main/EasyIngressScannerButton.user.js
// @description     Adds a button option to the portal details window to directly open the selected portal in the Ingress Scanner
// @author          Ayaro
// @include         *://*.ingress.com/*
// @match           *://*.ingress.com/*
// @grant           none
// ==/UserScript==

// Wrapper function that will be stringified and injected
// into the document. Because of this, normal closure rules
// do not apply here.
function wrapper(plugin_info) {
    // Make sure that window.plugin exists. IITC defines it as a no-op function,
    // and other plugins assume the same.
    if (typeof window.plugin !== 'function') window.plugin = function() {};

    // Use own namespace for plugin
    window.plugin.easyScannerButton = function() {};

    // Name of the IITC build for first-party plugins
    plugin_info.buildName = 'easyScannerButton';

    // Datetime-derived version of the plugin
    plugin_info.dateTimeVersion = '20220627125800';

    // ID/name of the plugin
    plugin_info.pluginId = 'easyScannerButton';

    // The entry point for this plugin.
    function setup() {
      window.addHook('portalDetailsUpdated', window.plugin.easyScannerButton.addScannerButton);
    }

    window.plugin.easyScannerButton.icon = 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABIAAAASCAYAAABWzo5XAAABemlDQ1BJQ0MgUHJvZmlsZQAAKM+lkDFLw1AUhU9bpaKVDjo4OGQoCtKC1MVR61CQUkqtYNUlSZNWSNqQtIg4Orh26KLiYhX/gW7iHxAEQZ1c7OygIIKUeF5SKIgO4g333Y/z7n157wDBpqGazsAsYFbrdj6dktaK61K4gwBCXk7LqmMt5nIZ/Brv9+xk3CXEWfhbjJQ0RwUCQ+R51bLr5AVydrtuCW6Sx9WKXCKfkuM2L0i+Fbric0dw2ecPwXYhv8S3RchS2ee4YMVn8RZJrdgm2SDHTKOh9u4jXhLRqqsrrJNeOsgjjRQkKGhgCwbqSLBW6dnPc0lvLosaZ1SuFnZgc6KMCmfjVBs8VWPVqWv8DHYwhPffPXX0uaT/h8gyMPjsum8zQPgI6O677ueJ63bbQOgRuG7152st2vlCvdnXYsdAdA+4uOpryhlwSY8nnizZlj0pxAzqOvB6DowWgTF6Pbzx333f794+2g9AYRfI3AAHh8AU+6ObX6okdIVZiIabAAAACXBIWXMAAA7EAAAOxAGVKw4bAAADi0lEQVQ4T32Ua2yTZRTHf+/by7audbRsbMW5FUbYxrrEGVScQ0bwEhNNQOeHIRc/8EmCxjjilA8aVAIxcRmihmAcTqIYDcagGKZZBE38ILIgyAiZbMAuZYNdurVd2/fiedqZGEJ23jzvc57mnP/z/5/znmqvN35qM49pWna3540CfW6/oxlpm+G+m0T6JzCN+ZH0zFX/W5pQSCUN4lNJXB6bj3t30PbnNjR3msRUinTSzLK8Pe+1Rz4RT3x5dF1n5J9xVqwu474nl+Ir8NLVeQavL4+H1tcQnYhy9sQVLv8+RPEyP7alQLK5WsvqQ7am2VgWAnKTlw5tIBaNcfqLvxk4f4P69dU4XW5Of32OinuDrNkSxuVw8+GLxylZEkCT4iiC2isNB21NdxAdm+Hgxe3sevwwfT2j+AKeTJB3YR66aImOxpC7mB6NU9NYys4jG3h1VQdevyejzmmb4BBG8VicP77vZ/DSBFv3rONWJEpBkQd3jkvSLVIpk8nINIWlAT5742cudI8wGzfw+UWaSHSmRZNlaRgC5rnLzeD1KRqeqSLfn8veTccywIpKRV0ROzueJXL1Fm3b4uR6XRhCJSUglmA4LVUocUzTlmWy6G4fbzV9Rd3aECsfXU5dY1JRljq46Xizi56fBghIjKUADLlBdsVIV0WWHVO9JNyUW24MRCmvLqZ9x3FhMEOkb5qPWk6wNLyYseHpDEg21sQQXx31lCn6DUibGrF4mtisTe3aMjr3nGJf12bO9lzj3Pkh3u/ewuF3ugmvKWMmYRGPJ0WWLnmQFDXCSBBFXiKZpPLBEl7Y/TD3VBYz2D/JX78N8lhTFc3bH+DUt5cYHU4QWhFk89urqKwPMptIZ3JVjXRFy0gb5Hs8LPDncPXCGAdaT1Jc7ufIvl956vmVhOtDHDtwhkDQS3vrj0xenyU/z4nbIwU3zKw0E9Epy1foocG9m6r7Q7z3XTMLSjyk7DQtT3/JrqajGb+o1Mv+k5tYXBFgXf5efAtzpXOCIEvbGP5A2qbGxcbh0BnqH6eitojG52pYsryI1uajOJzw7ufNXOkd4Zdvehm4OE4wVIAl9cmMhyr9xtp2wZCDfMYZrTJv6uMzZHAV37YftmJIQ15+ohO3W+TIcuU4Mp1Tw6tyVQcFaL/y7mgq5trlMRkVjbJlhSp+7ncFMneYs3n/j1RsqHoR5SLxPxBlt4MA/AudzbDXyxzongAAAABJRU5ErkJggg==';

    window.plugin.easyScannerButton.addScannerButton = function() {
      easyScannerButtonURL = 'https://link.ingress.com/?link=https://intel.ingress.com/portal/' + window.selectedPortal + '&apn=com.nianticproject.ingress&isi=576505181&ibi=com.google.ingress&ifl=https://apps.apple.com/app/ingress/id576505181&ofl=https://intel.ingress.com/intel?pll=' + window.portals[window.selectedPortal]._latlng.lat + ',' + window.portals[window.selectedPortal]._latlng.lng;
      $('.linkdetails').append('<aside><a href="' + easyScannerButtonURL + '" id="Easy-Ingress-Scanner-Button"><img src="'+ window.plugin.easyScannerButton.icon +'""></a></aside>');
    }

    // Add an info property for IITC's plugin system
    setup.info = plugin_info;

    // Make sure window.bootPlugins exists and is an array
    if (!window.bootPlugins) window.bootPlugins = [];
    // Add our startup hook
    window.bootPlugins.push(setup);
    // If IITC has already booted, immediately run the 'setup' function
    if (window.iitcLoaded && typeof setup === 'function') setup();
  }

  // Create a script element to hold our content script
  var script = document.createElement('script');
  var info = {};

  // GM_info is defined by the assorted monkey-themed browser extensions
  // and holds information parsed from the script header.
  if (typeof GM_info !== 'undefined' && GM_info && GM_info.script) {
    info.script = {
      version: GM_info.script.version,
      name: GM_info.script.name,
      description: GM_info.script.description
    };
  }

  // Create a text node and our IIFE inside of it
  var textContent = document.createTextNode('(' + wrapper + ')(' + JSON.stringify(info) + ')');
  // Add some content to the script element
  script.appendChild(textContent);
  // Finally, inject it... wherever.
  (document.body || document.head || document.documentElement).appendChild(script);
