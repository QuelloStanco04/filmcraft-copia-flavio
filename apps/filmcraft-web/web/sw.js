// Minimal service worker: makes the page installable (Chrome wants a fetch handler). It does not
// cache anything, so a new build is always picked up.
self.addEventListener("install", () => self.skipWaiting());
self.addEventListener("activate", (e) => e.waitUntil(self.clients.claim()));
self.addEventListener("fetch", () => {});
