// js/buildtime/index.ts
var runtimeError = (method) => new Error(
  `macroforge/buildtime.${method}: @buildtime APIs are only available inside @buildtime declarations evaluated by macroforge. If you're seeing this at runtime, the macroforge plugin is not installed or not running on this file.`
);
var buildtime = {
  fs: {
    readText(_path) {
      throw runtimeError("fs.readText");
    },
    readJson(_path) {
      throw runtimeError("fs.readJson");
    },
    exists(_path) {
      throw runtimeError("fs.exists");
    },
    listDir(_path) {
      throw runtimeError("fs.listDir");
    }
  },
  crypto: {
    sha256(_input) {
      throw runtimeError("crypto.sha256");
    },
    sha512(_input) {
      throw runtimeError("crypto.sha512");
    }
  },
  time: {
    now() {
      throw runtimeError("time.now");
    },
    unix() {
      throw runtimeError("time.unix");
    },
    iso() {
      throw runtimeError("time.iso");
    }
  },
  env: new Proxy(
    {},
    {
      get(_target, _prop) {
        throw runtimeError("env");
      }
    }
  ),
  flags: {
    has(_flag) {
      throw runtimeError("flags.has");
    },
    get(_flag) {
      throw runtimeError("flags.get");
    }
  },
  location: new Proxy({}, {
    get(_target, _prop) {
      throw runtimeError("location");
    }
  })
};
var $buildtime = buildtime;
export {
  $buildtime,
  buildtime
};
