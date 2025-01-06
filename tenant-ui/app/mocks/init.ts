async function enableMocking() {
  if (process.env.NODE_ENV !== "development") {
    return;
  }

  const { worker } = await import("./browser");

  return worker.start({
    serviceWorker: {
      url: "/ui/mockServiceWorker.js",
      options: {},
    },
  });
}

export default enableMocking;
