// Based upon the work of the Deno Authors. All rights reserved. MIT license.
"use strict";

((window) => {
  function writable(value) {
    return {
      value,
      writable: true,
      enumerable: true,
      configurable: true,
    };
  }

  function nonEnumerable(value) {
    return {
      value,
      writable: true,
      enumerable: false,
      configurable: true,
    };
  }

  function readOnly(value) {
    return {
      value,
      enumerable: true,
      writable: false,
      configurable: true,
    };
  }

  function getterOnly(getter) {
    return {
      get: getter,
      enumerable: true,
      configurable: true,
    };
  }

  window.__bootstrap.util = {
    writable,
    nonEnumerable,
    readOnly,
    getterOnly,
  };
})(this);

((window) => {
  const core = Deno.core;
  const util = window.__bootstrap.util;
  const timers = window.__bootstrap.timers;
  const base64 = window.__bootstrap.base64;
  const encoding = window.__bootstrap.encoding;
  const Console = window.__bootstrap.console.Console;
  const crypto = window.__bootstrap.crypto;
  const url = window.__bootstrap.url;
  const headers = window.__bootstrap.headers;
  const streams = window.__bootstrap.streams;
  const webSocket = window.__bootstrap.webSocket;
  const broadcastChannel = window.__bootstrap.broadcastChannel;
  const fetch = window.__bootstrap.fetch;
  const file = window.__bootstrap.file;
  const nevermore = window.__bootstrap.nevermore;
  const net = window.__bootstrap.net;

  const windowOrWorkerGlobalScope = {
    Blob: util.nonEnumerable(file.Blob),
    ByteLengthQueuingStrategy: util.nonEnumerable(
      streams.ByteLengthQueuingStrategy
    ),
    CloseEvent: util.nonEnumerable(CloseEvent),
    CountQueuingStrategy: util.nonEnumerable(streams.CountQueuingStrategy),
    CustomEvent: util.nonEnumerable(CustomEvent),
    Deno: util.nonEnumerable(net),
    // DOMException: util.nonEnumerable(DOMException),
    ErrorEvent: util.nonEnumerable(ErrorEvent),
    Event: util.nonEnumerable(Event),
    EventTarget: util.nonEnumerable(EventTarget),
    File: util.nonEnumerable(file.File),
    Headers: util.nonEnumerable(headers.Headers),
    MessageEvent: util.nonEnumerable(MessageEvent),
    ProgressEvent: util.nonEnumerable(ProgressEvent),
    ReadableStream: util.nonEnumerable(streams.ReadableStream),
    ReadableStreamDefaultReader: util.nonEnumerable(
      streams.ReadableStreamDefaultReader
    ),
    Request: util.nonEnumerable(fetch.Request),
    Response: util.nonEnumerable(fetch.Response),
    TextDecoder: util.nonEnumerable(encoding.TextDecoder),
    TextEncoder: util.nonEnumerable(encoding.TextEncoder),
    TextDecoderStream: util.nonEnumerable(encoding.TextDecoderStream),
    TextEncoderStream: util.nonEnumerable(encoding.TextEncoderStream),
    TransformStream: util.nonEnumerable(streams.TransformStream),
    URL: util.nonEnumerable(url.URL),
    URLSearchParams: util.nonEnumerable(url.URLSearchParams),
    WebSocket: util.nonEnumerable(webSocket.WebSocket),
    BroadcastChannel: util.nonEnumerable(broadcastChannel.BroadcastChannel),
    WritableStream: util.nonEnumerable(streams.WritableStream),
    WritableStreamDefaultWriter: util.nonEnumerable(
      streams.WritableStreamDefaultWriter
    ),
    WritableStreamDefaultController: util.nonEnumerable(
      streams.WritableStreamDefaultController
    ),
    ReadableByteStreamController: util.nonEnumerable(
      streams.ReadableByteStreamController
    ),
    ReadableStreamDefaultController: util.nonEnumerable(
      streams.ReadableStreamDefaultController
    ),
    TransformStreamDefaultController: util.nonEnumerable(
      streams.TransformStreamDefaultController
    ),
    atob: util.writable(base64.atob),
    btoa: util.writable(base64.btoa),
    clearInterval: util.writable(timers.clearInterval),
    clearTimeout: util.writable(timers.clearTimeout),
    console: util.writable(
      new Console((message, level) => {
        let date = new Date();

        core.opSync("op_log", {
          message,
          level,
          dateTime: date.toLocaleString("en")
        })
      })
    ),
    crypto: util.readOnly(crypto.crypto),
    Crypto: util.nonEnumerable(crypto.Crypto),
    SubtleCrypto: util.nonEnumerable(crypto.SubtleCrypto),
    fetch: util.writable(fetch.fetch),
    setInterval: util.writable(timers.setInterval),
    setTimeout: util.writable(timers.setTimeout),
    Nevermore: util.nonEnumerable(nevermore.Nevermore),
  };

  core.setMacrotaskCallback(timers.handleTimerMacrotask);

  const consoleFromV8 = window.console;
  const wrapConsole = window.__bootstrap.console.wrapConsole;

  delete globalThis.__bootstrap;
  delete globalThis.bootstrap;

  Object.defineProperties(globalThis, windowOrWorkerGlobalScope);
  
  const consoleFromDeno = globalThis.console;
  wrapConsole(consoleFromDeno, consoleFromV8);
})(this);
