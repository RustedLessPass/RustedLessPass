var cacheName = "rustedlesspass";
var filesToCache = [
  "./",
  "index.html",
  "manifest.json",
  "service_worker.js",
  "index.css",
  "pico.orange.min.css",
  "rustedlesspass-web.js",
  "rustedlesspass-web_bg.wasm",
];

// Add all files in the "assets" directory
var assetsFiles = [
  "./assets/fontawesome/css/all.css",
  "./assets/fontawesome/css/all.min.css",
  "./assets/fontawesome/css/brands.css",
  "./assets/fontawesome/css/brands.min.css",
  "./assets/fontawesome/css/fontawesome.css",
  "./assets/fontawesome/css/fontawesome.min.css",
  "./assets/fontawesome/css/regular.css",
  "./assets/fontawesome/css/regular.min.css",
  "./assets/fontawesome/css/solid.css",
  "./assets/fontawesome/css/solid.min.css",
  "./assets/fontawesome/css/svg-with-js.css",
  "./assets/fontawesome/css/svg-with-js.min.css",
  "./assets/fontawesome/css/v4-font-face.css",
  "./assets/fontawesome/css/v4-font-face.min.css",
  "./assets/fontawesome/css/v4-shims.css",
  "./assets/fontawesome/css/v4-shims.min.css",
  "./assets/fontawesome/css/v5-font-face.css",
  "./assets/fontawesome/css/v5-font-face.min.css",
  "./assets/fontawesome/webfonts/fa-brands-400.ttf",
  "./assets/fontawesome/webfonts/fa-brands-400.woff2",
  "./assets/fontawesome/webfonts/fa-regular-400.ttf",
  "./assets/fontawesome/webfonts/fa-regular-400.woff2",
  "./assets/fontawesome/webfonts/fa-solid-900.ttf",
  "./assets/fontawesome/webfonts/fa-solid-900.woff2",
  "./assets/fontawesome/webfonts/fa-v4compatibility.ttf",
  "./assets/fontawesome/webfonts/fa-v4compatibility.woff2",
  "./assets/icons/maskable_icon_x48.png",
  "./assets/icons/maskable_icon_x96.png",
  "./assets/icons/maskable_icon_x128.png",
  "./assets/icons/maskable_icon_x192.png",
  "./assets/icons/maskable_icon_x384.png",
  "./assets/icons/maskable_icon_x512.png",
  "./assets/icons/maskable_icon_x512.icns",
  "./assets/icons/maskable_icon_x512.ico",
  "./assets/minimal-theme-switcher.js",
  "./assets/pico.orange.min.css",
];

filesToCache.push(...assetsFiles);

/* Start the service worker and cache all of the app's content */
self.addEventListener("install", function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

/* Serve cached content when offline */
self.addEventListener("fetch", function (e) {
  e.respondWith(
    caches.match(e.request).then(function (response) {
      return response || fetch(e.request);
    })
  );
});
