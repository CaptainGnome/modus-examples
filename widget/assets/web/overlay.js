(function () {
  var proto = location.protocol === "https:" ? "wss:" : "ws:";
  var base = location.pathname.replace(/\/index\.html$/, "/").replace(/\/?$/, "/");
  var count = document.getElementById("count");
  var socket = null;
  function connect() {
    var ws = new WebSocket(proto + "//" + location.host + base + "ws");
    socket = ws;
    ws.onmessage = function (raw) {
      var frame;
      try { frame = JSON.parse(raw.data); } catch (err) { return; }
      if (frame.type === "plugin" && count) {
        try {
          var data = JSON.parse(frame.payload);
          if (data && typeof data.n === "number") count.textContent = String(data.n);
        } catch (err) {}
      }
    };
    ws.onclose = function () { setTimeout(connect, 1000); };
  }
  var inc = document.getElementById("inc");
  if (inc) {
    inc.onclick = function () {
      if (socket && socket.readyState === 1) {
        socket.send(JSON.stringify({ type: "plugin", payload: "{\"click\":1}" }));
      }
    };
  }
  connect();
})();
