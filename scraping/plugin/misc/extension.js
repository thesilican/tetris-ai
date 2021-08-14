// ==UserScript==
// @name         Tetr.IO Scraper
// @namespace    http://tampermonkey.net/
// @version      0.1
// @description  try to take over the world!
// @author       You
// @match        https://tetr.io/*
// @icon         https://www.google.com/s2/favicons?domain=thesilican.com
// @grant        none
// ==/UserScript==

(function () {
  "use strict";
  const script = document.createElement("script");
  script.src = "http://localhost:1234/index.js";
  document.head.prepend(script);
})();
