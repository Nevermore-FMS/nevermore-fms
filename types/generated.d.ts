// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.

// deno-lint-ignore-file no-explicit-any

/// <reference no-default-lib="true" />
/// <reference lib="esnext" />

declare interface Console {
  assert(condition?: boolean, ...data: any[]): void;
  clear(): void;
  count(label?: string): void;
  countReset(label?: string): void;
  debug(...data: any[]): void;
  dir(item?: any, options?: any): void;
  dirxml(...data: any[]): void;
  error(...data: any[]): void;
  group(...data: any[]): void;
  groupCollapsed(...data: any[]): void;
  groupEnd(): void;
  info(...data: any[]): void;
  log(...data: any[]): void;
  table(tabularData?: any, properties?: string[]): void;
  time(label?: string): void;
  timeEnd(label?: string): void;
  timeLog(label?: string, ...data: any[]): void;
  trace(...data: any[]): void;
  warn(...data: any[]): void;
}
// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.

// deno-lint-ignore-file no-explicit-any

/// <reference no-default-lib="true" />
/// <reference lib="esnext" />

declare class URLSearchParams {
  constructor(
    init?: string[][] | Record<string, string> | string | URLSearchParams,
  );
  static toString(): string;

  /** Appends a specified key/value pair as a new search parameter.
   *
   * ```ts
   * let searchParams = new URLSearchParams();
   * searchParams.append('name', 'first');
   * searchParams.append('name', 'second');
   * ```
   */
  append(name: string, value: string): void;

  /** Deletes the given search parameter and its associated value,
   * from the list of all search parameters.
   *
   * ```ts
   * let searchParams = new URLSearchParams([['name', 'value']]);
   * searchParams.delete('name');
   * ```
   */
  delete(name: string): void;

  /** Returns all the values associated with a given search parameter
   * as an array.
   *
   * ```ts
   * searchParams.getAll('name');
   * ```
   */
  getAll(name: string): string[];

  /** Returns the first value associated to the given search parameter.
   *
   * ```ts
   * searchParams.get('name');
   * ```
   */
  get(name: string): string | null;

  /** Returns a Boolean that indicates whether a parameter with the
   * specified name exists.
   *
   * ```ts
   * searchParams.has('name');
   * ```
   */
  has(name: string): boolean;

  /** Sets the value associated with a given search parameter to the
   * given value. If there were several matching values, this method
   * deletes the others. If the search parameter doesn't exist, this
   * method creates it.
   *
   * ```ts
   * searchParams.set('name', 'value');
   * ```
   */
  set(name: string, value: string): void;

  /** Sort all key/value pairs contained in this object in place and
   * return undefined. The sort order is according to Unicode code
   * points of the keys.
   *
   * ```ts
   * searchParams.sort();
   * ```
   */
  sort(): void;

  /** Calls a function for each element contained in this object in
   * place and return undefined. Optionally accepts an object to use
   * as this when executing callback as second argument.
   *
   * ```ts
   * const params = new URLSearchParams([["a", "b"], ["c", "d"]]);
   * params.forEach((value, key, parent) => {
   *   console.log(value, key, parent);
   * });
   * ```
   */
  forEach(
    callbackfn: (value: string, key: string, parent: this) => void,
    thisArg?: any,
  ): void;

  /** Returns an iterator allowing to go through all keys contained
   * in this object.
   *
   * ```ts
   * const params = new URLSearchParams([["a", "b"], ["c", "d"]]);
   * for (const key of params.keys()) {
   *   console.log(key);
   * }
   * ```
   */
  keys(): IterableIterator<string>;

  /** Returns an iterator allowing to go through all values contained
   * in this object.
   *
   * ```ts
   * const params = new URLSearchParams([["a", "b"], ["c", "d"]]);
   * for (const value of params.values()) {
   *   console.log(value);
   * }
   * ```
   */
  values(): IterableIterator<string>;

  /** Returns an iterator allowing to go through all key/value
   * pairs contained in this object.
   *
   * ```ts
   * const params = new URLSearchParams([["a", "b"], ["c", "d"]]);
   * for (const [key, value] of params.entries()) {
   *   console.log(key, value);
   * }
   * ```
   */
  entries(): IterableIterator<[string, string]>;

  /** Returns an iterator allowing to go through all key/value
   * pairs contained in this object.
   *
   * ```ts
   * const params = new URLSearchParams([["a", "b"], ["c", "d"]]);
   * for (const [key, value] of params) {
   *   console.log(key, value);
   * }
   * ```
   */
  [Symbol.iterator](): IterableIterator<[string, string]>;

  /** Returns a query string suitable for use in a URL.
   *
   * ```ts
   * searchParams.toString();
   * ```
   */
  toString(): string;
}

/** The URL interface represents an object providing static methods used for creating object URLs. */
declare class URL {
  constructor(url: string, base?: string | URL);
  static createObjectURL(blob: Blob): string;
  static revokeObjectURL(url: string): void;

  hash: string;
  host: string;
  hostname: string;
  href: string;
  toString(): string;
  readonly origin: string;
  password: string;
  pathname: string;
  port: string;
  protocol: string;
  search: string;
  readonly searchParams: URLSearchParams;
  username: string;
  toJSON(): string;
}

declare interface URLPatternInit {
  protocol?: string;
  username?: string;
  password?: string;
  hostname?: string;
  port?: string;
  pathname?: string;
  search?: string;
  hash?: string;
  baseURL?: string;
}

declare type URLPatternInput = string | URLPatternInit;

declare interface URLPatternComponentResult {
  input: string;
  groups: Record<string, string>;
}

/** `URLPatternResult` is the object returned from `URLPattern.match`. */
declare interface URLPatternResult {
  /** The inputs provided when matching. */
  inputs: [URLPatternInit] | [URLPatternInit, string];

  /** The matched result for the `protocol` matcher. */
  protocol: URLPatternComponentResult;
  /** The matched result for the `username` matcher. */
  username: URLPatternComponentResult;
  /** The matched result for the `password` matcher. */
  password: URLPatternComponentResult;
  /** The matched result for the `hostname` matcher. */
  hostname: URLPatternComponentResult;
  /** The matched result for the `port` matcher. */
  port: URLPatternComponentResult;
  /** The matched result for the `pathname` matcher. */
  pathname: URLPatternComponentResult;
  /** The matched result for the `search` matcher. */
  search: URLPatternComponentResult;
  /** The matched result for the `hash` matcher. */
  hash: URLPatternComponentResult;
}

/**
 * The URLPattern API provides a web platform primitive for matching URLs based
 * on a convenient pattern syntax.
 *
 * The syntax is based on path-to-regexp. Wildcards, named capture groups,
 * regular groups, and group modifiers are all supported.
 *
 * ```ts
 * // Specify the pattern as structured data.
 * const pattern = new URLPattern({ pathname: "/users/:user" });
 * const match = pattern.match("/users/joe");
 * console.log(match.pathname.groups.user); // joe
 * ```
 *
 * ```ts
 * // Specify a fully qualified string pattern.
 * const pattern = new URLPattern("https://example.com/books/:id");
 * console.log(pattern.test("https://example.com/books/123")); // true
 * console.log(pattern.test("https://deno.land/books/123")); // false
 * ```
 *
 * ```ts
 * // Specify a relative string pattern with a base URL.
 * const pattern = new URLPattern("/:article", "https://blog.example.com");
 * console.log(pattern.test("https://blog.example.com/article")); // true
 * console.log(pattern.test("https://blog.example.com/article/123")); // false
 * ```
 */
declare class URLPattern {
  constructor(input: URLPatternInput, baseURL?: string);

  /**
   * Test if the given input matches the stored pattern.
   *
   * The input can either be provided as a url string (with an optional base),
   * or as individual components in the form of an object.
   *
   * ```ts
   * const pattern = new URLPattern("https://example.com/books/:id");
   *
   * // Test a url string.
   * console.log(pattern.test("https://example.com/books/123")); // true
   *
   * // Test a relative url with a base.
   * console.log(pattern.test("/books/123", "https://example.com")); // true
   *
   * // Test an object of url components.
   * console.log(pattern.test({ pathname: "/books/123" })); // true
   * ```
   */
  test(input: URLPatternInput, baseURL?: string): boolean;

  /**
   * Match the given input against the stored pattern.
   *
   * The input can either be provided as a url string (with an optional base),
   * or as individual components in the form of an object.
   *
   * ```ts
   * const pattern = new URLPattern("https://example.com/books/:id");
   *
   * // Match a url string.
   * let match = pattern.match("https://example.com/books/123");
   * console.log(match.pathname.groups.id); // 123
   *
   * // Match a relative url with a base.
   * match = pattern.match("/books/123", "https://example.com");
   * console.log(match.pathname.groups.id); // 123
   *
   * // Match an object of url components.
   * match = pattern.match({ pathname: "/books/123" });
   * console.log(match.pathname.groups.id); // 123
   * ```
   */
  exec(input: URLPatternInput, baseURL?: string): URLPatternResult | null;

  /** The pattern string for the `protocol`. */
  readonly protocol: string;
  /** The pattern string for the `username`. */
  readonly username: string;
  /** The pattern string for the `password`. */
  readonly password: string;
  /** The pattern string for the `hostname`. */
  readonly hostname: string;
  /** The pattern string for the `port`. */
  readonly port: string;
  /** The pattern string for the `pathname`. */
  readonly pathname: string;
  /** The pattern string for the `search`. */
  readonly search: string;
  /** The pattern string for the `hash`. */
  readonly hash: string;
}
// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.

// deno-lint-ignore-file no-explicit-any

/// <reference no-default-lib="true" />
/// <reference lib="esnext" />

declare class DOMException extends Error {
  constructor(message?: string, name?: string);
  readonly name: string;
  readonly message: string;
  readonly code: number;
}

interface EventInit {
  bubbles?: boolean;
  cancelable?: boolean;
  composed?: boolean;
}

/** An event which takes place in the DOM. */
declare class Event {
  constructor(type: string, eventInitDict?: EventInit);
  /** Returns true or false depending on how event was initialized. True if
   * event goes through its target's ancestors in reverse tree order, and
   * false otherwise. */
  readonly bubbles: boolean;
  cancelBubble: boolean;
  /** Returns true or false depending on how event was initialized. Its return
   * value does not always carry meaning, but true can indicate that part of the
   * operation during which event was dispatched, can be canceled by invoking
   * the preventDefault() method. */
  readonly cancelable: boolean;
  /** Returns true or false depending on how event was initialized. True if
   * event invokes listeners past a ShadowRoot node that is the root of its
   * target, and false otherwise. */
  readonly composed: boolean;
  /** Returns the object whose event listener's callback is currently being
   * invoked. */
  readonly currentTarget: EventTarget | null;
  /** Returns true if preventDefault() was invoked successfully to indicate
   * cancellation, and false otherwise. */
  readonly defaultPrevented: boolean;
  /** Returns the event's phase, which is one of NONE, CAPTURING_PHASE,
   * AT_TARGET, and BUBBLING_PHASE. */
  readonly eventPhase: number;
  /** Returns true if event was dispatched by the user agent, and false
   * otherwise. */
  readonly isTrusted: boolean;
  /** Returns the object to which event is dispatched (its target). */
  readonly target: EventTarget | null;
  /** Returns the event's timestamp as the number of milliseconds measured
   * relative to the time origin. */
  readonly timeStamp: number;
  /** Returns the type of event, e.g. "click", "hashchange", or "submit". */
  readonly type: string;
  /** Returns the invocation target objects of event's path (objects on which
   * listeners will be invoked), except for any nodes in shadow trees of which
   * the shadow root's mode is "closed" that are not reachable from event's
   * currentTarget. */
  composedPath(): EventTarget[];
  /** If invoked when the cancelable attribute value is true, and while
   * executing a listener for the event with passive set to false, signals to
   * the operation that caused event to be dispatched that it needs to be
   * canceled. */
  preventDefault(): void;
  /** Invoking this method prevents event from reaching any registered event
   * listeners after the current one finishes running and, when dispatched in a
   * tree, also prevents event from reaching any other objects. */
  stopImmediatePropagation(): void;
  /** When dispatched in a tree, invoking this method prevents event from
   * reaching any objects other than the current object. */
  stopPropagation(): void;
  readonly AT_TARGET: number;
  readonly BUBBLING_PHASE: number;
  readonly CAPTURING_PHASE: number;
  readonly NONE: number;
  static readonly AT_TARGET: number;
  static readonly BUBBLING_PHASE: number;
  static readonly CAPTURING_PHASE: number;
  static readonly NONE: number;
}

/**
 * EventTarget is a DOM interface implemented by objects that can receive events
 * and may have listeners for them.
 */
declare class EventTarget {
  /** Appends an event listener for events whose type attribute value is type.
   * The callback argument sets the callback that will be invoked when the event
   * is dispatched.
   *
   * The options argument sets listener-specific options. For compatibility this
   * can be a boolean, in which case the method behaves exactly as if the value
   * was specified as options's capture.
   *
   * When set to true, options's capture prevents callback from being invoked
   * when the event's eventPhase attribute value is BUBBLING_PHASE. When false
   * (or not present), callback will not be invoked when event's eventPhase
   * attribute value is CAPTURING_PHASE. Either way, callback will be invoked if
   * event's eventPhase attribute value is AT_TARGET.
   *
   * When set to true, options's passive indicates that the callback will not
   * cancel the event by invoking preventDefault(). This is used to enable
   * performance optimizations described in § 2.8 Observing event listeners.
   *
   * When set to true, options's once indicates that the callback will only be
   * invoked once after which the event listener will be removed.
   *
   * The event listener is appended to target's event listener list and is not
   * appended if it has the same type, callback, and capture. */
  addEventListener(
    type: string,
    listener: EventListenerOrEventListenerObject | null,
    options?: boolean | AddEventListenerOptions,
  ): void;
  /** Dispatches a synthetic event event to target and returns true if either
   * event's cancelable attribute value is false or its preventDefault() method
   * was not invoked, and false otherwise. */
  dispatchEvent(event: Event): boolean;
  /** Removes the event listener in target's event listener list with the same
   * type, callback, and options. */
  removeEventListener(
    type: string,
    callback: EventListenerOrEventListenerObject | null,
    options?: EventListenerOptions | boolean,
  ): void;
}

interface EventListener {
  (evt: Event): void | Promise<void>;
}

interface EventListenerObject {
  handleEvent(evt: Event): void | Promise<void>;
}

declare type EventListenerOrEventListenerObject =
  | EventListener
  | EventListenerObject;

interface AddEventListenerOptions extends EventListenerOptions {
  once?: boolean;
  passive?: boolean;
}

interface EventListenerOptions {
  capture?: boolean;
}

interface ProgressEventInit extends EventInit {
  lengthComputable?: boolean;
  loaded?: number;
  total?: number;
}

/** Events measuring progress of an underlying process, like an HTTP request
 * (for an XMLHttpRequest, or the loading of the underlying resource of an
 * <img>, <audio>, <video>, <style> or <link>). */
declare class ProgressEvent<T extends EventTarget = EventTarget> extends Event {
  constructor(type: string, eventInitDict?: ProgressEventInit);
  readonly lengthComputable: boolean;
  readonly loaded: number;
  readonly target: T | null;
  readonly total: number;
}

/** Decodes a string of data which has been encoded using base-64 encoding.
 *
 *     console.log(atob("aGVsbG8gd29ybGQ=")); // outputs 'hello world'
 */
declare function atob(s: string): string;

/** Creates a base-64 ASCII encoded string from the input string.
 *
 *     console.log(btoa("hello world"));  // outputs "aGVsbG8gd29ybGQ="
 */
declare function btoa(s: string): string;

declare interface TextDecoderOptions {
  fatal?: boolean;
  ignoreBOM?: boolean;
}

declare interface TextDecodeOptions {
  stream?: boolean;
}

declare class TextDecoder {
  constructor(label?: string, options?: TextDecoderOptions);

  /** Returns encoding's name, lowercased. */
  readonly encoding: string;
  /** Returns `true` if error mode is "fatal", and `false` otherwise. */
  readonly fatal: boolean;
  /** Returns `true` if ignore BOM flag is set, and `false` otherwise. */
  readonly ignoreBOM = false;

  /** Returns the result of running encoding's decoder. */
  decode(input?: BufferSource, options?: TextDecodeOptions): string;
}

declare interface TextEncoderEncodeIntoResult {
  read: number;
  written: number;
}

declare class TextEncoder {
  /** Returns "utf-8". */
  readonly encoding: "utf-8";
  /** Returns the result of running UTF-8's encoder. */
  encode(input?: string): Uint8Array;
  encodeInto(input: string, dest: Uint8Array): TextEncoderEncodeIntoResult;
}

declare class TextDecoderStream {
  /** Returns encoding's name, lowercased. */
  readonly encoding: string;
  /** Returns `true` if error mode is "fatal", and `false` otherwise. */
  readonly fatal: boolean;
  /** Returns `true` if ignore BOM flag is set, and `false` otherwise. */
  readonly ignoreBOM = false;
  constructor(label?: string, options?: TextDecoderOptions);
  readonly readable: ReadableStream<string>;
  readonly writable: WritableStream<BufferSource>;
  readonly [Symbol.toStringTag]: string;
}

declare class TextEncoderStream {
  /** Returns "utf-8". */
  readonly encoding: "utf-8";
  readonly readable: ReadableStream<Uint8Array>;
  readonly writable: WritableStream<string>;
  readonly [Symbol.toStringTag]: string;
}

/** A controller object that allows you to abort one or more DOM requests as and
 * when desired. */
declare class AbortController {
  /** Returns the AbortSignal object associated with this object. */
  readonly signal: AbortSignal;
  /** Invoking this method will set this object's AbortSignal's aborted flag and
   * signal to any observers that the associated activity is to be aborted. */
  abort(): void;
}

interface AbortSignalEventMap {
  abort: Event;
}

/** A signal object that allows you to communicate with a DOM request (such as a
 * Fetch) and abort it if required via an AbortController object. */
interface AbortSignal extends EventTarget {
  /** Returns true if this AbortSignal's AbortController has signaled to abort,
   * and false otherwise. */
  readonly aborted: boolean;
  onabort: ((this: AbortSignal, ev: Event) => any) | null;
  addEventListener<K extends keyof AbortSignalEventMap>(
    type: K,
    listener: (this: AbortSignal, ev: AbortSignalEventMap[K]) => any,
    options?: boolean | AddEventListenerOptions,
  ): void;
  addEventListener(
    type: string,
    listener: EventListenerOrEventListenerObject,
    options?: boolean | AddEventListenerOptions,
  ): void;
  removeEventListener<K extends keyof AbortSignalEventMap>(
    type: K,
    listener: (this: AbortSignal, ev: AbortSignalEventMap[K]) => any,
    options?: boolean | EventListenerOptions,
  ): void;
  removeEventListener(
    type: string,
    listener: EventListenerOrEventListenerObject,
    options?: boolean | EventListenerOptions,
  ): void;
}

declare var AbortSignal: {
  prototype: AbortSignal;
  new (): AbortSignal;
};

interface FileReaderEventMap {
  "abort": ProgressEvent<FileReader>;
  "error": ProgressEvent<FileReader>;
  "load": ProgressEvent<FileReader>;
  "loadend": ProgressEvent<FileReader>;
  "loadstart": ProgressEvent<FileReader>;
  "progress": ProgressEvent<FileReader>;
}

/** Lets web applications asynchronously read the contents of files (or raw data buffers) stored on the user's computer, using File or Blob objects to specify the file or data to read. */
interface FileReader extends EventTarget {
  readonly error: DOMException | null;
  onabort: ((this: FileReader, ev: ProgressEvent<FileReader>) => any) | null;
  onerror: ((this: FileReader, ev: ProgressEvent<FileReader>) => any) | null;
  onload: ((this: FileReader, ev: ProgressEvent<FileReader>) => any) | null;
  onloadend: ((this: FileReader, ev: ProgressEvent<FileReader>) => any) | null;
  onloadstart:
    | ((this: FileReader, ev: ProgressEvent<FileReader>) => any)
    | null;
  onprogress: ((this: FileReader, ev: ProgressEvent<FileReader>) => any) | null;
  readonly readyState: number;
  readonly result: string | ArrayBuffer | null;
  abort(): void;
  readAsArrayBuffer(blob: Blob): void;
  readAsBinaryString(blob: Blob): void;
  readAsDataURL(blob: Blob): void;
  readAsText(blob: Blob, encoding?: string): void;
  readonly DONE: number;
  readonly EMPTY: number;
  readonly LOADING: number;
  addEventListener<K extends keyof FileReaderEventMap>(
    type: K,
    listener: (this: FileReader, ev: FileReaderEventMap[K]) => any,
    options?: boolean | AddEventListenerOptions,
  ): void;
  addEventListener(
    type: string,
    listener: EventListenerOrEventListenerObject,
    options?: boolean | AddEventListenerOptions,
  ): void;
  removeEventListener<K extends keyof FileReaderEventMap>(
    type: K,
    listener: (this: FileReader, ev: FileReaderEventMap[K]) => any,
    options?: boolean | EventListenerOptions,
  ): void;
  removeEventListener(
    type: string,
    listener: EventListenerOrEventListenerObject,
    options?: boolean | EventListenerOptions,
  ): void;
}

declare var FileReader: {
  prototype: FileReader;
  new (): FileReader;
  readonly DONE: number;
  readonly EMPTY: number;
  readonly LOADING: number;
};

type BlobPart = BufferSource | Blob | string;

interface BlobPropertyBag {
  type?: string;
  endings?: "transparent" | "native";
}

/** A file-like object of immutable, raw data. Blobs represent data that isn't necessarily in a JavaScript-native format. The File interface is based on Blob, inheriting blob functionality and expanding it to support files on the user's system. */
declare class Blob {
  constructor(blobParts?: BlobPart[], options?: BlobPropertyBag);

  readonly size: number;
  readonly type: string;
  arrayBuffer(): Promise<ArrayBuffer>;
  slice(start?: number, end?: number, contentType?: string): Blob;
  stream(): ReadableStream<Uint8Array>;
  text(): Promise<string>;
}

interface FilePropertyBag extends BlobPropertyBag {
  lastModified?: number;
}

/** Provides information about files and allows JavaScript in a web page to
 * access their content. */
declare class File extends Blob {
  constructor(
    fileBits: BlobPart[],
    fileName: string,
    options?: FilePropertyBag,
  );

  readonly lastModified: number;
  readonly name: string;
}

interface ReadableStreamReadDoneResult<T> {
  done: true;
  value?: T;
}

interface ReadableStreamReadValueResult<T> {
  done: false;
  value: T;
}

type ReadableStreamReadResult<T> =
  | ReadableStreamReadValueResult<T>
  | ReadableStreamReadDoneResult<T>;

interface ReadableStreamDefaultReader<R = any> {
  readonly closed: Promise<void>;
  cancel(reason?: any): Promise<void>;
  read(): Promise<ReadableStreamReadResult<R>>;
  releaseLock(): void;
}

declare var ReadableStreamDefaultReader: {
  prototype: ReadableStreamDefaultReader;
  new <R>(stream: ReadableStream<R>): ReadableStreamDefaultReader<R>;
};

interface ReadableStreamReader<R = any> {
  cancel(): Promise<void>;
  read(): Promise<ReadableStreamReadResult<R>>;
  releaseLock(): void;
}

declare var ReadableStreamReader: {
  prototype: ReadableStreamReader;
  new (): ReadableStreamReader;
};

interface ReadableByteStreamControllerCallback {
  (controller: ReadableByteStreamController): void | PromiseLike<void>;
}

interface UnderlyingByteSource {
  autoAllocateChunkSize?: number;
  cancel?: ReadableStreamErrorCallback;
  pull?: ReadableByteStreamControllerCallback;
  start?: ReadableByteStreamControllerCallback;
  type: "bytes";
}

interface UnderlyingSink<W = any> {
  abort?: WritableStreamErrorCallback;
  close?: WritableStreamDefaultControllerCloseCallback;
  start?: WritableStreamDefaultControllerStartCallback;
  type?: undefined;
  write?: WritableStreamDefaultControllerWriteCallback<W>;
}

interface UnderlyingSource<R = any> {
  cancel?: ReadableStreamErrorCallback;
  pull?: ReadableStreamDefaultControllerCallback<R>;
  start?: ReadableStreamDefaultControllerCallback<R>;
  type?: undefined;
}

interface ReadableStreamErrorCallback {
  (reason: any): void | PromiseLike<void>;
}

interface ReadableStreamDefaultControllerCallback<R> {
  (controller: ReadableStreamDefaultController<R>): void | PromiseLike<void>;
}

interface ReadableStreamDefaultController<R = any> {
  readonly desiredSize: number | null;
  close(): void;
  enqueue(chunk: R): void;
  error(error?: any): void;
}

declare var ReadableStreamDefaultController: {
  prototype: ReadableStreamDefaultController;
  new (): ReadableStreamDefaultController;
};

interface ReadableByteStreamController {
  readonly byobRequest: undefined;
  readonly desiredSize: number | null;
  close(): void;
  enqueue(chunk: ArrayBufferView): void;
  error(error?: any): void;
}

declare var ReadableByteStreamController: {
  prototype: ReadableByteStreamController;
  new (): ReadableByteStreamController;
};

interface PipeOptions {
  preventAbort?: boolean;
  preventCancel?: boolean;
  preventClose?: boolean;
  signal?: AbortSignal;
}

interface QueuingStrategySizeCallback<T = any> {
  (chunk: T): number;
}

interface QueuingStrategy<T = any> {
  highWaterMark?: number;
  size?: QueuingStrategySizeCallback<T>;
}

/** This Streams API interface provides a built-in byte length queuing strategy
 * that can be used when constructing streams. */
declare class CountQueuingStrategy implements QueuingStrategy {
  constructor(options: { highWaterMark: number });
  highWaterMark: number;
  size(chunk: any): 1;
}

declare class ByteLengthQueuingStrategy
  implements QueuingStrategy<ArrayBufferView> {
  constructor(options: { highWaterMark: number });
  highWaterMark: number;
  size(chunk: ArrayBufferView): number;
}

/** This Streams API interface represents a readable stream of byte data. The
 * Fetch API offers a concrete instance of a ReadableStream through the body
 * property of a Response object. */
interface ReadableStream<R = any> {
  readonly locked: boolean;
  cancel(reason?: any): Promise<void>;
  /**
   * @deprecated This is no longer part of the Streams standard and the async
   *             iterable should be obtained by just using the stream as an
   *             async iterator.
   */
  getIterator(options?: { preventCancel?: boolean }): AsyncIterableIterator<R>;
  getReader(): ReadableStreamDefaultReader<R>;
  pipeThrough<T>(
    { writable, readable }: {
      writable: WritableStream<R>;
      readable: ReadableStream<T>;
    },
    options?: PipeOptions,
  ): ReadableStream<T>;
  pipeTo(dest: WritableStream<R>, options?: PipeOptions): Promise<void>;
  tee(): [ReadableStream<R>, ReadableStream<R>];
  [Symbol.asyncIterator](options?: {
    preventCancel?: boolean;
  }): AsyncIterableIterator<R>;
}

declare var ReadableStream: {
  prototype: ReadableStream;
  new (
    underlyingSource: UnderlyingByteSource,
    strategy?: { highWaterMark?: number; size?: undefined },
  ): ReadableStream<Uint8Array>;
  new <R = any>(
    underlyingSource?: UnderlyingSource<R>,
    strategy?: QueuingStrategy<R>,
  ): ReadableStream<R>;
};

interface WritableStreamDefaultControllerCloseCallback {
  (): void | PromiseLike<void>;
}

interface WritableStreamDefaultControllerStartCallback {
  (controller: WritableStreamDefaultController): void | PromiseLike<void>;
}

interface WritableStreamDefaultControllerWriteCallback<W> {
  (chunk: W, controller: WritableStreamDefaultController):
    | void
    | PromiseLike<
      void
    >;
}

interface WritableStreamErrorCallback {
  (reason: any): void | PromiseLike<void>;
}

/** This Streams API interface provides a standard abstraction for writing
 * streaming data to a destination, known as a sink. This object comes with
 * built-in backpressure and queuing. */
interface WritableStream<W = any> {
  readonly locked: boolean;
  abort(reason?: any): Promise<void>;
  getWriter(): WritableStreamDefaultWriter<W>;
}

declare var WritableStream: {
  prototype: WritableStream;
  new <W = any>(
    underlyingSink?: UnderlyingSink<W>,
    strategy?: QueuingStrategy<W>,
  ): WritableStream<W>;
};

/** This Streams API interface represents a controller allowing control of a
 * WritableStream's state. When constructing a WritableStream, the underlying
 * sink is given a corresponding WritableStreamDefaultController instance to
 * manipulate. */
interface WritableStreamDefaultController {
  error(error?: any): void;
}

/** This Streams API interface is the object returned by
 * WritableStream.getWriter() and once created locks the < writer to the
 * WritableStream ensuring that no other streams can write to the underlying
 * sink. */
interface WritableStreamDefaultWriter<W = any> {
  readonly closed: Promise<void>;
  readonly desiredSize: number | null;
  readonly ready: Promise<void>;
  abort(reason?: any): Promise<void>;
  close(): Promise<void>;
  releaseLock(): void;
  write(chunk: W): Promise<void>;
}

declare var WritableStreamDefaultWriter: {
  prototype: WritableStreamDefaultWriter;
  new (): WritableStreamDefaultWriter;
};

interface TransformStream<I = any, O = any> {
  readonly readable: ReadableStream<O>;
  readonly writable: WritableStream<I>;
}

declare var TransformStream: {
  prototype: TransformStream;
  new <I = any, O = any>(
    transformer?: Transformer<I, O>,
    writableStrategy?: QueuingStrategy<I>,
    readableStrategy?: QueuingStrategy<O>,
  ): TransformStream<I, O>;
};

interface TransformStreamDefaultController<O = any> {
  readonly desiredSize: number | null;
  enqueue(chunk: O): void;
  error(reason?: any): void;
  terminate(): void;
}

interface Transformer<I = any, O = any> {
  flush?: TransformStreamDefaultControllerCallback<O>;
  readableType?: undefined;
  start?: TransformStreamDefaultControllerCallback<O>;
  transform?: TransformStreamDefaultControllerTransformCallback<I, O>;
  writableType?: undefined;
}

interface TransformStreamDefaultControllerCallback<O> {
  (controller: TransformStreamDefaultController<O>): void | PromiseLike<void>;
}

interface TransformStreamDefaultControllerTransformCallback<I, O> {
  (
    chunk: I,
    controller: TransformStreamDefaultController<O>,
  ): void | PromiseLike<void>;
}

interface MessageEventInit<T = any> extends EventInit {
  data?: T;
  origin?: string;
  lastEventId?: string;
}

declare class MessageEvent<T = any> extends Event {
  /**
   * Returns the data of the message.
   */
  readonly data: T;
  /**
   * Returns the last event ID string, for server-sent events.
   */
  readonly lastEventId: string;
  /**
   * Returns transfered ports.
   */
  readonly ports: ReadonlyArray<MessagePort>;
  constructor(type: string, eventInitDict?: MessageEventInit);
}

type Transferable = ArrayBuffer | MessagePort;

/**
 * @deprecated
 *
 * This type has been renamed to StructuredSerializeOptions. Use that type for
 * new code.
 */
type PostMessageOptions = StructuredSerializeOptions;

interface StructuredSerializeOptions {
  transfer?: Transferable[];
}

/** The MessageChannel interface of the Channel Messaging API allows us to
 * create a new message channel and send data through it via its two MessagePort
 * properties. */
declare class MessageChannel {
  constructor();
  readonly port1: MessagePort;
  readonly port2: MessagePort;
}

interface MessagePortEventMap {
  "message": MessageEvent;
  "messageerror": MessageEvent;
}

/** The MessagePort interface of the Channel Messaging API represents one of the
 * two ports of a MessageChannel, allowing messages to be sent from one port and
 * listening out for them arriving at the other. */
declare class MessagePort extends EventTarget {
  onmessage: ((this: MessagePort, ev: MessageEvent) => any) | null;
  onmessageerror: ((this: MessagePort, ev: MessageEvent) => any) | null;
  /**
   * Disconnects the port, so that it is no longer active.
   */
  close(): void;
  /**
   * Posts a message through the channel. Objects listed in transfer are
   * transferred, not just cloned, meaning that they are no longer usable on the
   * sending side.
   *
   * Throws a "DataCloneError" DOMException if transfer contains duplicate
   * objects or port, or if message could not be cloned.
   */
  postMessage(message: any, transfer: Transferable[]): void;
  postMessage(message: any, options?: StructuredSerializeOptions): void;
  /**
   * Begins dispatching messages received on the port. This is implictly called
   * when assiging a value to `this.onmessage`.
   */
  start(): void;
  addEventListener<K extends keyof MessagePortEventMap>(
    type: K,
    listener: (this: MessagePort, ev: MessagePortEventMap[K]) => any,
    options?: boolean | AddEventListenerOptions,
  ): void;
  addEventListener(
    type: string,
    listener: EventListenerOrEventListenerObject,
    options?: boolean | AddEventListenerOptions,
  ): void;
  removeEventListener<K extends keyof MessagePortEventMap>(
    type: K,
    listener: (this: MessagePort, ev: MessagePortEventMap[K]) => any,
    options?: boolean | EventListenerOptions,
  ): void;
  removeEventListener(
    type: string,
    listener: EventListenerOrEventListenerObject,
    options?: boolean | EventListenerOptions,
  ): void;
}

declare function structuredClone(
  value: any,
  options?: StructuredSerializeOptions,
): any;
// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.

// deno-lint-ignore-file no-explicit-any

/// <reference no-default-lib="true" />
/// <reference lib="esnext" />

interface DomIterable<K, V> {
  keys(): IterableIterator<K>;
  values(): IterableIterator<V>;
  entries(): IterableIterator<[K, V]>;
  [Symbol.iterator](): IterableIterator<[K, V]>;
  forEach(
    callback: (value: V, key: K, parent: this) => void,
    thisArg?: any,
  ): void;
}

type FormDataEntryValue = File | string;

/** Provides a way to easily construct a set of key/value pairs representing
 * form fields and their values, which can then be easily sent using the
 * XMLHttpRequest.send() method. It uses the same format a form would use if the
 * encoding type were set to "multipart/form-data". */
declare class FormData implements DomIterable<string, FormDataEntryValue> {
  // TODO(ry) FormData constructor is non-standard.
  // new(form?: HTMLFormElement): FormData;
  constructor();

  append(name: string, value: string | Blob, fileName?: string): void;
  delete(name: string): void;
  get(name: string): FormDataEntryValue | null;
  getAll(name: string): FormDataEntryValue[];
  has(name: string): boolean;
  set(name: string, value: string | Blob, fileName?: string): void;
  keys(): IterableIterator<string>;
  values(): IterableIterator<string>;
  entries(): IterableIterator<[string, FormDataEntryValue]>;
  [Symbol.iterator](): IterableIterator<[string, FormDataEntryValue]>;
  forEach(
    callback: (value: FormDataEntryValue, key: string, parent: this) => void,
    thisArg?: any,
  ): void;
}

interface Body {
  /** A simple getter used to expose a `ReadableStream` of the body contents. */
  readonly body: ReadableStream<Uint8Array> | null;
  /** Stores a `Boolean` that declares whether the body has been used in a
   * response yet.
   */
  readonly bodyUsed: boolean;
  /** Takes a `Response` stream and reads it to completion. It returns a promise
   * that resolves with an `ArrayBuffer`.
   */
  arrayBuffer(): Promise<ArrayBuffer>;
  /** Takes a `Response` stream and reads it to completion. It returns a promise
   * that resolves with a `Blob`.
   */
  blob(): Promise<Blob>;
  /** Takes a `Response` stream and reads it to completion. It returns a promise
   * that resolves with a `FormData` object.
   */
  formData(): Promise<FormData>;
  /** Takes a `Response` stream and reads it to completion. It returns a promise
   * that resolves with the result of parsing the body text as JSON.
   */
  json(): Promise<any>;
  /** Takes a `Response` stream and reads it to completion. It returns a promise
   * that resolves with a `USVString` (text).
   */
  text(): Promise<string>;
}

type HeadersInit = Headers | string[][] | Record<string, string>;

/** This Fetch API interface allows you to perform various actions on HTTP
 * request and response headers. These actions include retrieving, setting,
 * adding to, and removing. A Headers object has an associated header list,
 * which is initially empty and consists of zero or more name and value pairs.
 * You can add to this using methods like append() (see Examples). In all
 * methods of this interface, header names are matched by case-insensitive byte
 * sequence. */
interface Headers {
  append(name: string, value: string): void;
  delete(name: string): void;
  get(name: string): string | null;
  has(name: string): boolean;
  set(name: string, value: string): void;
  forEach(
    callbackfn: (value: string, key: string, parent: Headers) => void,
    thisArg?: any,
  ): void;
}

declare class Headers implements DomIterable<string, string> {
  constructor(init?: HeadersInit);

  /** Appends a new value onto an existing header inside a `Headers` object, or
   * adds the header if it does not already exist.
   */
  append(name: string, value: string): void;
  /** Deletes a header from a `Headers` object. */
  delete(name: string): void;
  /** Returns an iterator allowing to go through all key/value pairs
   * contained in this Headers object. The both the key and value of each pairs
   * are ByteString objects.
   */
  entries(): IterableIterator<[string, string]>;
  /** Returns a `ByteString` sequence of all the values of a header within a
   * `Headers` object with a given name.
   */
  get(name: string): string | null;
  /** Returns a boolean stating whether a `Headers` object contains a certain
   * header.
   */
  has(name: string): boolean;
  /** Returns an iterator allowing to go through all keys contained in
   * this Headers object. The keys are ByteString objects.
   */
  keys(): IterableIterator<string>;
  /** Sets a new value for an existing header inside a Headers object, or adds
   * the header if it does not already exist.
   */
  set(name: string, value: string): void;
  /** Returns an iterator allowing to go through all values contained in
   * this Headers object. The values are ByteString objects.
   */
  values(): IterableIterator<string>;
  forEach(
    callbackfn: (value: string, key: string, parent: this) => void,
    thisArg?: any,
  ): void;
  /** The Symbol.iterator well-known symbol specifies the default
   * iterator for this Headers object
   */
  [Symbol.iterator](): IterableIterator<[string, string]>;
}

type RequestInfo = Request | string;
type RequestCache =
  | "default"
  | "force-cache"
  | "no-cache"
  | "no-store"
  | "only-if-cached"
  | "reload";
type RequestCredentials = "include" | "omit" | "same-origin";
type RequestMode = "cors" | "navigate" | "no-cors" | "same-origin";
type RequestRedirect = "error" | "follow" | "manual";
type ReferrerPolicy =
  | ""
  | "no-referrer"
  | "no-referrer-when-downgrade"
  | "origin"
  | "origin-when-cross-origin"
  | "same-origin"
  | "strict-origin"
  | "strict-origin-when-cross-origin"
  | "unsafe-url";
type BodyInit =
  | Blob
  | BufferSource
  | FormData
  | URLSearchParams
  | ReadableStream<Uint8Array>
  | string;
type RequestDestination =
  | ""
  | "audio"
  | "audioworklet"
  | "document"
  | "embed"
  | "font"
  | "image"
  | "manifest"
  | "object"
  | "paintworklet"
  | "report"
  | "script"
  | "sharedworker"
  | "style"
  | "track"
  | "video"
  | "worker"
  | "xslt";

interface RequestInit {
  /**
   * A BodyInit object or null to set request's body.
   */
  body?: BodyInit | null;
  /**
   * A string indicating how the request will interact with the browser's cache
   * to set request's cache.
   */
  cache?: RequestCache;
  /**
   * A string indicating whether credentials will be sent with the request
   * always, never, or only when sent to a same-origin URL. Sets request's
   * credentials.
   */
  credentials?: RequestCredentials;
  /**
   * A Headers object, an object literal, or an array of two-item arrays to set
   * request's headers.
   */
  headers?: HeadersInit;
  /**
   * A cryptographic hash of the resource to be fetched by request. Sets
   * request's integrity.
   */
  integrity?: string;
  /**
   * A boolean to set request's keepalive.
   */
  keepalive?: boolean;
  /**
   * A string to set request's method.
   */
  method?: string;
  /**
   * A string to indicate whether the request will use CORS, or will be
   * restricted to same-origin URLs. Sets request's mode.
   */
  mode?: RequestMode;
  /**
   * A string indicating whether request follows redirects, results in an error
   * upon encountering a redirect, or returns the redirect (in an opaque
   * fashion). Sets request's redirect.
   */
  redirect?: RequestRedirect;
  /**
   * A string whose value is a same-origin URL, "about:client", or the empty
   * string, to set request's referrer.
   */
  referrer?: string;
  /**
   * A referrer policy to set request's referrerPolicy.
   */
  referrerPolicy?: ReferrerPolicy;
  /**
   * An AbortSignal to set request's signal.
   */
  signal?: AbortSignal | null;
  /**
   * Can only be null. Used to disassociate request from any Window.
   */
  window?: any;
}

/** This Fetch API interface represents a resource request. */
declare class Request implements Body {
  constructor(input: RequestInfo, init?: RequestInit);

  /**
   * Returns the cache mode associated with request, which is a string
   * indicating how the request will interact with the browser's cache when
   * fetching.
   */
  readonly cache: RequestCache;
  /**
   * Returns the credentials mode associated with request, which is a string
   * indicating whether credentials will be sent with the request always, never,
   * or only when sent to a same-origin URL.
   */
  readonly credentials: RequestCredentials;
  /**
   * Returns the kind of resource requested by request, e.g., "document" or "script".
   */
  readonly destination: RequestDestination;
  /**
   * Returns a Headers object consisting of the headers associated with request.
   * Note that headers added in the network layer by the user agent will not be
   * accounted for in this object, e.g., the "Host" header.
   */
  readonly headers: Headers;
  /**
   * Returns request's subresource integrity metadata, which is a cryptographic
   * hash of the resource being fetched. Its value consists of multiple hashes
   * separated by whitespace. [SRI]
   */
  readonly integrity: string;
  /**
   * Returns a boolean indicating whether or not request is for a history
   * navigation (a.k.a. back-forward navigation).
   */
  readonly isHistoryNavigation: boolean;
  /**
   * Returns a boolean indicating whether or not request is for a reload
   * navigation.
   */
  readonly isReloadNavigation: boolean;
  /**
   * Returns a boolean indicating whether or not request can outlive the global
   * in which it was created.
   */
  readonly keepalive: boolean;
  /**
   * Returns request's HTTP method, which is "GET" by default.
   */
  readonly method: string;
  /**
   * Returns the mode associated with request, which is a string indicating
   * whether the request will use CORS, or will be restricted to same-origin
   * URLs.
   */
  readonly mode: RequestMode;
  /**
   * Returns the redirect mode associated with request, which is a string
   * indicating how redirects for the request will be handled during fetching. A
   * request will follow redirects by default.
   */
  readonly redirect: RequestRedirect;
  /**
   * Returns the referrer of request. Its value can be a same-origin URL if
   * explicitly set in init, the empty string to indicate no referrer, and
   * "about:client" when defaulting to the global's default. This is used during
   * fetching to determine the value of the `Referer` header of the request
   * being made.
   */
  readonly referrer: string;
  /**
   * Returns the referrer policy associated with request. This is used during
   * fetching to compute the value of the request's referrer.
   */
  readonly referrerPolicy: ReferrerPolicy;
  /**
   * Returns the signal associated with request, which is an AbortSignal object
   * indicating whether or not request has been aborted, and its abort event
   * handler.
   */
  readonly signal: AbortSignal;
  /**
   * Returns the URL of request as a string.
   */
  readonly url: string;
  clone(): Request;

  /** A simple getter used to expose a `ReadableStream` of the body contents. */
  readonly body: ReadableStream<Uint8Array> | null;
  /** Stores a `Boolean` that declares whether the body has been used in a
   * response yet.
   */
  readonly bodyUsed: boolean;
  /** Takes a `Response` stream and reads it to completion. It returns a promise
   * that resolves with an `ArrayBuffer`.
   */
  arrayBuffer(): Promise<ArrayBuffer>;
  /** Takes a `Response` stream and reads it to completion. It returns a promise
   * that resolves with a `Blob`.
   */
  blob(): Promise<Blob>;
  /** Takes a `Response` stream and reads it to completion. It returns a promise
   * that resolves with a `FormData` object.
   */
  formData(): Promise<FormData>;
  /** Takes a `Response` stream and reads it to completion. It returns a promise
   * that resolves with the result of parsing the body text as JSON.
   */
  json(): Promise<any>;
  /** Takes a `Response` stream and reads it to completion. It returns a promise
   * that resolves with a `USVString` (text).
   */
  text(): Promise<string>;
}

interface ResponseInit {
  headers?: HeadersInit;
  status?: number;
  statusText?: string;
}

type ResponseType =
  | "basic"
  | "cors"
  | "default"
  | "error"
  | "opaque"
  | "opaqueredirect";

/** This Fetch API interface represents the response to a request. */
declare class Response implements Body {
  constructor(body?: BodyInit | null, init?: ResponseInit);
  static error(): Response;
  static redirect(url: string, status?: number): Response;

  readonly headers: Headers;
  readonly ok: boolean;
  readonly redirected: boolean;
  readonly status: number;
  readonly statusText: string;
  readonly trailer: Promise<Headers>;
  readonly type: ResponseType;
  readonly url: string;
  clone(): Response;

  /** A simple getter used to expose a `ReadableStream` of the body contents. */
  readonly body: ReadableStream<Uint8Array> | null;
  /** Stores a `Boolean` that declares whether the body has been used in a
   * response yet.
   */
  readonly bodyUsed: boolean;
  /** Takes a `Response` stream and reads it to completion. It returns a promise
   * that resolves with an `ArrayBuffer`.
   */
  arrayBuffer(): Promise<ArrayBuffer>;
  /** Takes a `Response` stream and reads it to completion. It returns a promise
   * that resolves with a `Blob`.
   */
  blob(): Promise<Blob>;
  /** Takes a `Response` stream and reads it to completion. It returns a promise
   * that resolves with a `FormData` object.
   */
  formData(): Promise<FormData>;
  /** Takes a `Response` stream and reads it to completion. It returns a promise
   * that resolves with the result of parsing the body text as JSON.
   */
  json(): Promise<any>;
  /** Takes a `Response` stream and reads it to completion. It returns a promise
   * that resolves with a `USVString` (text).
   */
  text(): Promise<string>;
}

/** Fetch a resource from the network. It returns a Promise that resolves to the
 * Response to that request, whether it is successful or not.
 *
 *     const response = await fetch("http://my.json.host/data.json");
 *     console.log(response.status);  // e.g. 200
 *     console.log(response.statusText); // e.g. "OK"
 *     const jsonData = await response.json();
 */
declare function fetch(
  input: Request | URL | string,
  init?: RequestInit,
): Promise<Response>;
// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.

// deno-lint-ignore-file no-explicit-any

/// <reference no-default-lib="true" />
/// <reference lib="esnext" />

interface CloseEventInit extends EventInit {
  code?: number;
  reason?: string;
  wasClean?: boolean;
}

declare class CloseEvent extends Event {
  constructor(type: string, eventInitDict?: CloseEventInit);
  /**
   * Returns the WebSocket connection close code provided by the server.
   */
  readonly code: number;
  /**
   * Returns the WebSocket connection close reason provided by the server.
   */
  readonly reason: string;
  /**
   * Returns true if the connection closed cleanly; false otherwise.
   */
  readonly wasClean: boolean;
}

interface WebSocketEventMap {
  close: CloseEvent;
  error: Event;
  message: MessageEvent;
  open: Event;
}

/** Provides the API for creating and managing a WebSocket connection to a server, as well as for sending and receiving data on the connection. */
declare class WebSocket extends EventTarget {
  constructor(url: string, protocols?: string | string[]);

  static readonly CLOSED: number;
  static readonly CLOSING: number;
  static readonly CONNECTING: number;
  static readonly OPEN: number;

  /**
   * Returns a string that indicates how binary data from the WebSocket object is exposed to scripts:
   *
   * Can be set, to change how binary data is returned. The default is "blob".
   */
  binaryType: BinaryType;
  /**
   * Returns the number of bytes of application data (UTF-8 text and binary data) that have been queued using send() but not yet been transmitted to the network.
   *
   * If the WebSocket connection is closed, this attribute's value will only increase with each call to the send() method. (The number does not reset to zero once the connection closes.)
   */
  readonly bufferedAmount: number;
  /**
   * Returns the extensions selected by the server, if any.
   */
  readonly extensions: string;
  onclose: ((this: WebSocket, ev: CloseEvent) => any) | null;
  onerror: ((this: WebSocket, ev: Event | ErrorEvent) => any) | null;
  onmessage: ((this: WebSocket, ev: MessageEvent) => any) | null;
  onopen: ((this: WebSocket, ev: Event) => any) | null;
  /**
   * Returns the subprotocol selected by the server, if any. It can be used in conjunction with the array form of the constructor's second argument to perform subprotocol negotiation.
   */
  readonly protocol: string;
  /**
   * Returns the state of the WebSocket object's connection. It can have the values described below.
   */
  readonly readyState: number;
  /**
   * Returns the URL that was used to establish the WebSocket connection.
   */
  readonly url: string;
  /**
   * Closes the WebSocket connection, optionally using code as the the WebSocket connection close code and reason as the the WebSocket connection close reason.
   */
  close(code?: number, reason?: string): void;
  /**
   * Transmits data using the WebSocket connection. data can be a string, a Blob, an ArrayBuffer, or an ArrayBufferView.
   */
  send(data: string | ArrayBufferLike | Blob | ArrayBufferView): void;
  readonly CLOSED: number;
  readonly CLOSING: number;
  readonly CONNECTING: number;
  readonly OPEN: number;
  addEventListener<K extends keyof WebSocketEventMap>(
    type: K,
    listener: (this: WebSocket, ev: WebSocketEventMap[K]) => any,
    options?: boolean | AddEventListenerOptions,
  ): void;
  addEventListener(
    type: string,
    listener: EventListenerOrEventListenerObject,
    options?: boolean | AddEventListenerOptions,
  ): void;
  removeEventListener<K extends keyof WebSocketEventMap>(
    type: K,
    listener: (this: WebSocket, ev: WebSocketEventMap[K]) => any,
    options?: boolean | EventListenerOptions,
  ): void;
  removeEventListener(
    type: string,
    listener: EventListenerOrEventListenerObject,
    options?: boolean | EventListenerOptions,
  ): void;
}

type BinaryType = "arraybuffer" | "blob";
// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.

/// <reference no-default-lib="true" />
/// <reference lib="esnext" />

declare var crypto: Crypto;

interface Algorithm {
  name: string;
}

interface KeyAlgorithm {
  name: string;
}

type AlgorithmIdentifier = string | Algorithm;
type HashAlgorithmIdentifier = AlgorithmIdentifier;
type KeyType = "private" | "public" | "secret";
type KeyUsage =
  | "decrypt"
  | "deriveBits"
  | "deriveKey"
  | "encrypt"
  | "sign"
  | "unwrapKey"
  | "verify"
  | "wrapKey";
type KeyFormat = "jwk" | "pkcs8" | "raw" | "spki";
type NamedCurve = string;

interface RsaOtherPrimesInfo {
  d?: string;
  r?: string;
  t?: string;
}

interface JsonWebKey {
  alg?: string;
  crv?: string;
  d?: string;
  dp?: string;
  dq?: string;
  e?: string;
  ext?: boolean;
  k?: string;
  // deno-lint-ignore camelcase
  key_ops?: string[];
  kty?: string;
  n?: string;
  oth?: RsaOtherPrimesInfo[];
  p?: string;
  q?: string;
  qi?: string;
  use?: string;
  x?: string;
  y?: string;
}

interface HmacKeyGenParams extends Algorithm {
  hash: HashAlgorithmIdentifier;
  length?: number;
}

interface EcKeyGenParams extends Algorithm {
  namedCurve: NamedCurve;
}

interface EcdsaParams extends Algorithm {
  hash: HashAlgorithmIdentifier;
}

interface RsaHashedImportParams extends Algorithm {
  hash: HashAlgorithmIdentifier;
}

interface RsaHashedKeyGenParams extends RsaKeyGenParams {
  hash: HashAlgorithmIdentifier;
}

interface RsaKeyGenParams extends Algorithm {
  modulusLength: number;
  publicExponent: Uint8Array;
}

interface RsaPssParams extends Algorithm {
  saltLength: number;
}

interface RsaOaepParams extends Algorithm {
  label?: Uint8Array;
}

interface HmacImportParams extends Algorithm {
  hash: HashAlgorithmIdentifier;
  length?: number;
}

interface EcKeyAlgorithm extends KeyAlgorithm {
  namedCurve: NamedCurve;
}

interface HmacKeyAlgorithm extends KeyAlgorithm {
  hash: KeyAlgorithm;
  length: number;
}

interface RsaHashedKeyAlgorithm extends RsaKeyAlgorithm {
  hash: KeyAlgorithm;
}

interface RsaKeyAlgorithm extends KeyAlgorithm {
  modulusLength: number;
  publicExponent: Uint8Array;
}

interface HkdfParams extends Algorithm {
  hash: HashAlgorithmIdentifier;
  info: BufferSource;
  salt: BufferSource;
}

interface Pbkdf2Params extends Algorithm {
  hash: HashAlgorithmIdentifier;
  iterations: number;
  salt: BufferSource;
}

/** The CryptoKey dictionary of the Web Crypto API represents a cryptographic key. */
interface CryptoKey {
  readonly algorithm: KeyAlgorithm;
  readonly extractable: boolean;
  readonly type: KeyType;
  readonly usages: KeyUsage[];
}

declare var CryptoKey: {
  prototype: CryptoKey;
  new (): CryptoKey;
};

/** The CryptoKeyPair dictionary of the Web Crypto API represents a key pair for an asymmetric cryptography algorithm, also known as a public-key algorithm. */
interface CryptoKeyPair {
  privateKey: CryptoKey;
  publicKey: CryptoKey;
}

declare var CryptoKeyPair: {
  prototype: CryptoKeyPair;
  new (): CryptoKeyPair;
};

/** This Web Crypto API interface provides a number of low-level cryptographic functions. It is accessed via the Crypto.subtle properties available in a window context (via Window.crypto). */
interface SubtleCrypto {
  generateKey(
    algorithm: RsaHashedKeyGenParams | EcKeyGenParams,
    extractable: boolean,
    keyUsages: KeyUsage[],
  ): Promise<CryptoKeyPair>;
  generateKey(
    algorithm: HmacKeyGenParams,
    extractable: boolean,
    keyUsages: KeyUsage[],
  ): Promise<CryptoKey>;
  generateKey(
    algorithm: AlgorithmIdentifier,
    extractable: boolean,
    keyUsages: KeyUsage[],
  ): Promise<CryptoKeyPair | CryptoKey>;
  importKey(
    format: "jwk",
    keyData: JsonWebKey,
    algorithm: AlgorithmIdentifier | HmacImportParams,
    extractable: boolean,
    keyUsages: KeyUsage[],
  ): Promise<CryptoKey>;
  importKey(
    format: Exclude<KeyFormat, "jwk">,
    keyData: BufferSource,
    algorithm: AlgorithmIdentifier | HmacImportParams | RsaHashedImportParams,
    extractable: boolean,
    keyUsages: KeyUsage[],
  ): Promise<CryptoKey>;
  exportKey(format: "jwk", key: CryptoKey): Promise<JsonWebKey>;
  exportKey(
    format: Exclude<KeyFormat, "jwk">,
    key: CryptoKey,
  ): Promise<ArrayBuffer>;
  sign(
    algorithm: AlgorithmIdentifier | RsaPssParams | EcdsaParams,
    key: CryptoKey,
    data: BufferSource,
  ): Promise<ArrayBuffer>;
  verify(
    algorithm: AlgorithmIdentifier | RsaPssParams | EcdsaParams,
    key: CryptoKey,
    signature: BufferSource,
    data: BufferSource,
  ): Promise<boolean>;
  digest(
    algorithm: AlgorithmIdentifier,
    data: BufferSource,
  ): Promise<ArrayBuffer>;
  encrypt(
    algorithm: AlgorithmIdentifier | RsaOaepParams,
    key: CryptoKey,
    data: BufferSource,
  ): Promise<ArrayBuffer>;
  decrypt(
    algorithm: AlgorithmIdentifier | RsaOaepParams,
    key: CryptoKey,
    data: BufferSource,
  ): Promise<ArrayBuffer>;
  deriveBits(
    algorithm: AlgorithmIdentifier | HkdfParams | Pbkdf2Params,
    baseKey: CryptoKey,
    length: number,
  ): Promise<ArrayBuffer>;
}

declare interface Crypto {
  readonly subtle: SubtleCrypto;
  getRandomValues<
    T extends
      | Int8Array
      | Int16Array
      | Int32Array
      | Uint8Array
      | Uint16Array
      | Uint32Array
      | Uint8ClampedArray
      | Float32Array
      | Float64Array
      | DataView
      | null,
  >(
    array: T,
  ): T;
  randomUUID(): string;
}

declare var SubtleCrypto: {
  prototype: SubtleCrypto;
  new (): SubtleCrypto;
};
// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.

// deno-lint-ignore-file no-explicit-any

/// <reference no-default-lib="true" />
/// <reference lib="esnext" />

interface BroadcastChannelEventMap {
  "message": MessageEvent;
  "messageerror": MessageEvent;
}

interface BroadcastChannel extends EventTarget {
  /**
   * Returns the channel name (as passed to the constructor).
   */
  readonly name: string;
  onmessage: ((this: BroadcastChannel, ev: MessageEvent) => any) | null;
  onmessageerror: ((this: BroadcastChannel, ev: MessageEvent) => any) | null;
  /**
   * Closes the BroadcastChannel object, opening it up to garbage collection.
   */
  close(): void;
  /**
   * Sends the given message to other BroadcastChannel objects set up for
   * this channel. Messages can be structured objects, e.g. nested objects
   * and arrays.
   */
  postMessage(message: any): void;
  addEventListener<K extends keyof BroadcastChannelEventMap>(
    type: K,
    listener: (this: BroadcastChannel, ev: BroadcastChannelEventMap[K]) => any,
    options?: boolean | AddEventListenerOptions,
  ): void;
  addEventListener(
    type: string,
    listener: EventListenerOrEventListenerObject,
    options?: boolean | AddEventListenerOptions,
  ): void;
  removeEventListener<K extends keyof BroadcastChannelEventMap>(
    type: K,
    listener: (this: BroadcastChannel, ev: BroadcastChannelEventMap[K]) => any,
    options?: boolean | EventListenerOptions,
  ): void;
  removeEventListener(
    type: string,
    listener: EventListenerOrEventListenerObject,
    options?: boolean | EventListenerOptions,
  ): void;
}

declare var BroadcastChannel: {
  prototype: BroadcastChannel;
  new (name: string): BroadcastChannel;
};
// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.

/// <reference no-default-lib="true" />
/// <reference lib="esnext" />

declare namespace Deno {
  export interface NetAddr {
    transport: "tcp" | "udp";
    hostname: string;
    port: number;
  }

  export interface UnixAddr {
    transport: "unix" | "unixpacket";
    path: string;
  }

  export type Addr = NetAddr | UnixAddr;

  /** A generic network listener for stream-oriented protocols. */
  export interface Listener extends AsyncIterable<Conn> {
    /** Waits for and resolves to the next connection to the `Listener`. */
    accept(): Promise<Conn>;
    /** Close closes the listener. Any pending accept promises will be rejected
     * with errors. */
    close(): void;
    /** Return the address of the `Listener`. */
    readonly addr: Addr;

    /** Return the rid of the `Listener`. */
    readonly rid: number;

    [Symbol.asyncIterator](): AsyncIterableIterator<Conn>;
  }

  export interface Conn extends Reader, Writer, Closer {
    /** The local address of the connection. */
    readonly localAddr: Addr;
    /** The remote address of the connection. */
    readonly remoteAddr: Addr;
    /** The resource ID of the connection. */
    readonly rid: number;
    /** Shuts down (`shutdown(2)`) the write side of the connection. Most
     * callers should just use `close()`. */
    closeWrite(): Promise<void>;
  }

  export interface ListenOptions {
    /** The port to listen on. */
    port: number;
    /** A literal IP address or host name that can be resolved to an IP address.
     * If not specified, defaults to `0.0.0.0`.
     *
     * __Note about `0.0.0.0`__ While listening `0.0.0.0` works on all platforms,
     * the browsers on Windows don't work with the address `0.0.0.0`.
     * You should show the message like `server running on localhost:8080` instead of
     * `server running on 0.0.0.0:8080` if your program supports Windows. */
    hostname?: string;
  }

  /** Listen announces on the local transport address.
   *
   * ```ts
   * const listener1 = Deno.listen({ port: 80 })
   * const listener2 = Deno.listen({ hostname: "192.0.2.1", port: 80 })
   * const listener3 = Deno.listen({ hostname: "[2001:db8::1]", port: 80 });
   * const listener4 = Deno.listen({ hostname: "golang.org", port: 80, transport: "tcp" });
   * ```
   *
   * Requires `allow-net` permission. */
  export function listen(
    options: ListenOptions & { transport?: "tcp" },
  ): Listener;

  export interface ListenTlsOptions extends ListenOptions {
    /** Path to a file containing a PEM formatted CA certificate. Requires
     * `--allow-read`. */
    certFile: string;
    /** Server public key file. Requires `--allow-read`.*/
    keyFile: string;

    transport?: "tcp";
  }

  /** Listen announces on the local transport address over TLS (transport layer
   * security).
   *
   * ```ts
   * const lstnr = Deno.listenTls({ port: 443, certFile: "./server.crt", keyFile: "./server.key" });
   * ```
   *
   * Requires `allow-net` permission. */
  export function listenTls(options: ListenTlsOptions): Listener;

  export interface ConnectOptions {
    /** The port to connect to. */
    port: number;
    /** A literal IP address or host name that can be resolved to an IP address.
     * If not specified, defaults to `127.0.0.1`. */
    hostname?: string;
    transport?: "tcp";
  }

  /**
   * Connects to the hostname (default is "127.0.0.1") and port on the named
   * transport (default is "tcp"), and resolves to the connection (`Conn`).
   *
   * ```ts
   * const conn1 = await Deno.connect({ port: 80 });
   * const conn2 = await Deno.connect({ hostname: "192.0.2.1", port: 80 });
   * const conn3 = await Deno.connect({ hostname: "[2001:db8::1]", port: 80 });
   * const conn4 = await Deno.connect({ hostname: "golang.org", port: 80, transport: "tcp" });
   * ```
   *
   * Requires `allow-net` permission for "tcp". */
  export function connect(options: ConnectOptions): Promise<Conn>;

  export interface ConnectTlsOptions {
    /** The port to connect to. */
    port: number;
    /** A literal IP address or host name that can be resolved to an IP address.
     * If not specified, defaults to `127.0.0.1`. */
    hostname?: string;
    /** Server certificate file. */
    certFile?: string;
  }

  /** Establishes a secure connection over TLS (transport layer security) using
   * an optional cert file, hostname (default is "127.0.0.1") and port.  The
   * cert file is optional and if not included Mozilla's root certificates will
   * be used (see also https://github.com/ctz/webpki-roots for specifics)
   *
   * ```ts
   * const conn1 = await Deno.connectTls({ port: 80 });
   * const conn2 = await Deno.connectTls({ certFile: "./certs/my_custom_root_CA.pem", hostname: "192.0.2.1", port: 80 });
   * const conn3 = await Deno.connectTls({ hostname: "[2001:db8::1]", port: 80 });
   * const conn4 = await Deno.connectTls({ certFile: "./certs/my_custom_root_CA.pem", hostname: "golang.org", port: 80});
   * ```
   *
   * Requires `allow-net` permission.
   */
  export function connectTls(options: ConnectTlsOptions): Promise<Conn>;

  /** Shutdown socket send operations.
   *
   * Matches behavior of POSIX shutdown(3).
   *
   * ```ts
   * const listener = Deno.listen({ port: 80 });
   * const conn = await listener.accept();
   * Deno.shutdown(conn.rid);
   * ```
   */
  export function shutdown(rid: number): Promise<void>;
}
// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.

// Documentation partially adapted from [MDN](https://developer.mozilla.org/),
// by Mozilla Contributors, which is licensed under CC-BY-SA 2.5.

/// <reference no-default-lib="true" />
/// <reference lib="esnext" />
/// <reference lib="deno.console" />
/// <reference lib="deno.url" />
/// <reference lib="deno.web" />
/// <reference lib="deno.fetch" />
/// <reference lib="deno.websocket" />
/// <reference lib="deno.crypto" />
/// <reference lib="deno.broadcast_channel" />

declare namespace WebAssembly {
  /**
   * The `WebAssembly.CompileError` object indicates an error during WebAssembly decoding or validation.
   *
   * [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/CompileError)
   */
  export class CompileError extends Error {
    /** Creates a new `WebAssembly.CompileError` object. */
    constructor();
  }

  /**
   * A `WebAssembly.Global` object represents a global variable instance, accessible from
   * both JavaScript and importable/exportable across one or more `WebAssembly.Module`
   * instances. This allows dynamic linking of multiple modules.
   *
   * [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/Global)
   */
  export class Global {
    /** Creates a new `Global` object. */
    constructor(descriptor: GlobalDescriptor, v?: any);

    /**
     * The value contained inside the global variable — this can be used to directly set
     * and get the global's value.
     */
    value: any;

    /** Old-style method that returns the value contained inside the global variable. */
    valueOf(): any;
  }

  /**
   * A `WebAssembly.Instance` object is a stateful, executable instance of a `WebAssembly.Module`.
   * Instance objects contain all the Exported WebAssembly functions that allow calling into
   * WebAssembly code from JavaScript.
   *
   * [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/Instance)
   */
  export class Instance {
    /** Creates a new Instance object. */
    constructor(module: Module, importObject?: Imports);

    /**
     * Returns an object containing as its members all the functions exported from the
     * WebAssembly module instance, to allow them to be accessed and used by JavaScript.
     * Read-only.
     */
    readonly exports: Exports;
  }

  /**
   * The `WebAssembly.LinkError` object indicates an error during module instantiation
   * (besides traps from the start function).
   *
   * [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/LinkError)
   */
  export class LinkError extends Error {
    /** Creates a new WebAssembly.LinkError object. */
    constructor();
  }

  /**
   * The `WebAssembly.Memory` object is a resizable `ArrayBuffer` or `SharedArrayBuffer` that
   * holds the raw bytes of memory accessed by a WebAssembly Instance.
   *
   * A memory created by JavaScript or in WebAssembly code will be accessible and mutable
   * from both JavaScript and WebAssembly.
   *
   * [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/Memory)
   */
  export class Memory {
    /** Creates a new `Memory` object. */
    constructor(descriptor: MemoryDescriptor);

    /** An accessor property that returns the buffer contained in the memory. */
    readonly buffer: ArrayBuffer | SharedArrayBuffer;

    /**
     * Increases the size of the memory instance by a specified number of WebAssembly
     * pages (each one is 64KB in size).
     */
    grow(delta: number): number;
  }

  /**
   * A `WebAssembly.Module` object contains stateless WebAssembly code that has already been compiled
   * by the browser — this can be efficiently shared with Workers, and instantiated multiple times.
   *
   * [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/Module)
   */
  export class Module {
    /** Creates a new `Module` object. */
    constructor(bytes: BufferSource);

    /**
     * Given a `Module` and string, returns a copy of the contents of all custom sections in the
     * module with the given string name.
     * */
    static customSections(
      moduleObject: Module,
      sectionName: string,
    ): ArrayBuffer[];

    /** Given a `Module`, returns an array containing descriptions of all the declared exports. */
    static exports(moduleObject: Module): ModuleExportDescriptor[];

    /** Given a `Module`, returns an array containing descriptions of all the declared imports. */
    static imports(moduleObject: Module): ModuleImportDescriptor[];
  }

  /**
   * The `WebAssembly.RuntimeError` object is the error type that is thrown whenever WebAssembly
   * specifies a trap.
   *
   * [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/RuntimeError)
   */
  export class RuntimeError extends Error {
    /** Creates a new `WebAssembly.RuntimeError` object. */
    constructor();
  }

  /**
   * The `WebAssembly.Table()` object is a JavaScript wrapper object — an array-like structure
   * representing a WebAssembly Table, which stores function references. A table created by
   * JavaScript or in WebAssembly code will be accessible and mutable from both JavaScript
   * and WebAssembly.
   *
   * [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/Table)
   */
  export class Table {
    /** Creates a new `Table` object. */
    constructor(descriptor: TableDescriptor);

    /** Returns the length of the table, i.e. the number of elements. */
    readonly length: number;

    /** Accessor function — gets the element stored at a given index. */
    get(index: number): Function | null;

    /** Increases the size of the `Table` instance by a specified number of elements. */
    grow(delta: number): number;

    /** Sets an element stored at a given index to a given value. */
    set(index: number, value: Function | null): void;
  }

  /** The `GlobalDescriptor` describes the options you can pass to `new WebAssembly.Global()`. */
  export interface GlobalDescriptor {
    mutable?: boolean;
    value: ValueType;
  }

  /** The `MemoryDescriptor` describes the options you can pass to `new WebAssembly.Memory()`. */
  export interface MemoryDescriptor {
    initial: number;
    maximum?: number;
    shared?: boolean;
  }

  /** A `ModuleExportDescriptor` is the description of a declared export in a `WebAssembly.Module`. */
  export interface ModuleExportDescriptor {
    kind: ImportExportKind;
    name: string;
  }

  /** A `ModuleImportDescriptor` is the description of a declared import in a `WebAssembly.Module`. */
  export interface ModuleImportDescriptor {
    kind: ImportExportKind;
    module: string;
    name: string;
  }

  /** The `TableDescriptor` describes the options you can pass to `new WebAssembly.Table()`. */
  export interface TableDescriptor {
    element: TableKind;
    initial: number;
    maximum?: number;
  }

  /** The value returned from `WebAssembly.instantiate`. */
  export interface WebAssemblyInstantiatedSource {
    /* A `WebAssembly.Instance` object that contains all the exported WebAssembly functions. */
    instance: Instance;

    /**
     * A `WebAssembly.Module` object representing the compiled WebAssembly module.
     * This `Module` can be instantiated again, or shared via postMessage().
     */
    module: Module;
  }

  export type ImportExportKind = "function" | "global" | "memory" | "table";
  export type TableKind = "anyfunc";
  export type ValueType = "f32" | "f64" | "i32" | "i64";
  export type ExportValue = Function | Global | Memory | Table;
  export type Exports = Record<string, ExportValue>;
  export type ImportValue = ExportValue | number;
  export type ModuleImports = Record<string, ImportValue>;
  export type Imports = Record<string, ModuleImports>;

  /**
   * The `WebAssembly.compile()` function compiles WebAssembly binary code into a
   * `WebAssembly.Module` object. This function is useful if it is necessary to compile
   * a module before it can be instantiated (otherwise, the `WebAssembly.instantiate()`
   * function should be used).
   *
   * [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/compile)
   */
  export function compile(bytes: BufferSource): Promise<Module>;

  /**
   * The WebAssembly.instantiate() function allows you to compile and instantiate
   * WebAssembly code.
   *
   * This overload takes the WebAssembly binary code, in the form of a typed
   * array or ArrayBuffer, and performs both compilation and instantiation in one step.
   * The returned Promise resolves to both a compiled WebAssembly.Module and its first
   * WebAssembly.Instance.
   *
   * [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/instantiate)
   */
  export function instantiate(
    bytes: BufferSource,
    importObject?: Imports,
  ): Promise<WebAssemblyInstantiatedSource>;

  /**
   * The WebAssembly.instantiate() function allows you to compile and instantiate
   * WebAssembly code.
   *
   * This overload takes an already-compiled WebAssembly.Module and returns
   * a Promise that resolves to an Instance of that Module. This overload is useful
   * if the Module has already been compiled.
   *
   * [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/instantiate)
   */
  export function instantiate(
    moduleObject: Module,
    importObject?: Imports,
  ): Promise<Instance>;

  /**
   * The `WebAssembly.validate()` function validates a given typed array of
   * WebAssembly binary code, returning whether the bytes form a valid wasm
   * module (`true`) or not (`false`).
   *
   * [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WebAssembly/validate)
   */
  export function validate(bytes: BufferSource): boolean;
}

/** Sets a timer which executes a function once after the timer expires. Returns
 * an id which may be used to cancel the timeout.
 *
 *     setTimeout(() => { console.log('hello'); }, 500);
 */
declare function setTimeout(
  /** callback function to execute when timer expires */
  cb: (...args: any[]) => void,
  /** delay in ms */
  delay?: number,
  /** arguments passed to callback function */
  ...args: any[]
): number;

/** Repeatedly calls a function , with a fixed time delay between each call.
 *
 *     // Outputs 'hello' to the console every 500ms
 *     setInterval(() => { console.log('hello'); }, 500);
 */
declare function setInterval(
  /** callback function to execute when timer expires */
  cb: (...args: any[]) => void,
  /** delay in ms */
  delay?: number,
  /** arguments passed to callback function */
  ...args: any[]
): number;

/** Cancels a timed, repeating action which was previously started by a call
 * to `setInterval()`
 *
 *     const id = setInterval(() => {console.log('hello');}, 500);
 *     ...
 *     clearInterval(id);
 */
declare function clearInterval(id?: number): void;

/** Cancels a scheduled action initiated by `setTimeout()`
 *
 *     const id = setTimeout(() => {console.log('hello');}, 500);
 *     ...
 *     clearTimeout(id);
 */
declare function clearTimeout(id?: number): void;

interface VoidFunction {
  (): void;
}

/** A microtask is a short function which is executed after the function or
 * module which created it exits and only if the JavaScript execution stack is
 * empty, but before returning control to the event loop being used to drive the
 * script's execution environment. This event loop may be either the main event
 * loop or the event loop driving a web worker.
 *
 *     queueMicrotask(() => { console.log('This event loop stack is complete'); });
 */
declare function queueMicrotask(func: VoidFunction): void;

/** Dispatches an event in the global scope, synchronously invoking any
 * registered event listeners for this event in the appropriate order. Returns
 * false if event is cancelable and at least one of the event handlers which
 * handled this event called Event.preventDefault(). Otherwise it returns true.
 *
 *     dispatchEvent(new Event('unload'));
 */
declare function dispatchEvent(event: Event): boolean;

interface DOMStringList {
  /** Returns the number of strings in strings. */
  readonly length: number;
  /** Returns true if strings contains string, and false otherwise. */
  contains(string: string): boolean;
  /** Returns the string with index index from strings. */
  item(index: number): string | null;
  [index: number]: string;
}

type BufferSource = ArrayBufferView | ArrayBuffer;

declare var console: Console;

interface ErrorEventInit extends EventInit {
  message?: string;
  filename?: string;
  lineno?: number;
  colno?: number;
  error?: any;
}

declare class ErrorEvent extends Event {
  readonly message: string;
  readonly filename: string;
  readonly lineno: number;
  readonly colno: number;
  readonly error: any;
  constructor(type: string, eventInitDict?: ErrorEventInit);
}

interface AbstractWorkerEventMap {
  "error": ErrorEvent;
}

interface WorkerEventMap extends AbstractWorkerEventMap {
  "message": MessageEvent;
  "messageerror": MessageEvent;
}

interface WorkerOptions {
  type?: "classic" | "module";
  name?: string;
}

declare class Worker extends EventTarget {
  onerror?: (e: ErrorEvent) => void;
  onmessage?: (e: MessageEvent) => void;
  onmessageerror?: (e: MessageEvent) => void;
  constructor(
    specifier: string | URL,
    options?: WorkerOptions,
  );
  postMessage(message: any, transfer: Transferable[]): void;
  postMessage(message: any, options?: PostMessageOptions): void;
  addEventListener<K extends keyof WorkerEventMap>(
    type: K,
    listener: (this: Worker, ev: WorkerEventMap[K]) => any,
    options?: boolean | AddEventListenerOptions,
  ): void;
  addEventListener(
    type: string,
    listener: EventListenerOrEventListenerObject,
    options?: boolean | AddEventListenerOptions,
  ): void;
  removeEventListener<K extends keyof WorkerEventMap>(
    type: K,
    listener: (this: Worker, ev: WorkerEventMap[K]) => any,
    options?: boolean | EventListenerOptions,
  ): void;
  removeEventListener(
    type: string,
    listener: EventListenerOrEventListenerObject,
    options?: boolean | EventListenerOptions,
  ): void;
  terminate(): void;
}

declare interface CustomEventInit<T = any> extends EventInit {
  detail?: T;
}

declare class CustomEvent<T = any> extends Event {
  constructor(typeArg: string, eventInitDict?: CustomEventInit<T>);
  /** Returns any custom data event was created with. Typically used for
   * synthetic events. */
  readonly detail: T;
}

interface ErrorConstructor {
  /** See https://v8.dev/docs/stack-trace-api#stack-trace-collection-for-custom-exceptions. */
  captureStackTrace(error: Object, constructor?: Function): void;
  // TODO(nayeemrmn): Support `Error.prepareStackTrace()`. We currently use this
  // internally in a way that makes it unavailable for users.
}/**
 * The `Nevermore` namespace defines the API used to interact with the Nevermore FMS from Worker Scripts.
 * 
 * Remember, playing with robots is dangerous, but controlling their safety system is even more dangerous.
 * Ensure you triple check, test, and verify your workers before ever using them in production.
 */
declare namespace Nevermore {

    /**
     * The `Nevermore.Field` namespace defines the API used to interact with the active field.
     * 
     * When the worker script is ran it's assumed that the current field has been fully started and 
     * all functions may be called.
     */
    namespace Field {

        export function on(event: 'tick', listener: () => Promise<void>): void;

        export function on(event: 'close', listener: () => Promise<void>): void;

        /**
         * Retrieves a teams DriverStation.
         * 
         * @param teamNumber The team number for the driver station.
         */
        export function getDriverStation(teamNumber: number): Promise<DriverStation>

        /**
         * Retrieves all connected DriverStations
         */
        export function getDriverStations(): Promise<DriverStation[]>

        /**
         * Adds a team to the alliance station map, thereby allowing it to properly connect.
         * 
         * @param teamNumber The team number of the team intended to be added.
         * @param allianceStation The alliance station of the team.
         */
        export function addTeam(teamNumber: number, allianceStation: AllianceStation): Promise<void>

        /**
         * Removes a team from the alliance station map, doesn't work after a robot is already connected.
         * 
         * @param teamNumber The team number of the team intended to be removed.
         */
        export function removeTeam(teamNumber: number): Promise<void>

        /**
         * Emergency stops all robots on the field no matter the state of any driverstations.
         * 
         * @param emergencyStopped Whether the robots should or shouldn't be emergency stopped.
         */
        export function setOverrideEmergencyStoppedAll(emergencyStopped: boolean): Promise<void>

        /**
         * Enables or disabes all robots on the field no matter the state of any driverstations.
         * 
         * @param enabled Whether the robots should or shouldn't be enabled.
         */
        export function setOverrideEnabledAll(enabled: boolean): Promise<void>

        export function getTeamAllianceStation(teamNumber: number): Promise<AllianceStation>

        export function getTeamToAllianceStationMap(): Promise<Map<number, AllianceStation>>

        export enum AllianceStation {
            RED1,
            RED2,
            RED3,
            BLUE1,
            BLUE2,
            BLUE3,
            NONE
        }

        export enum DriverStationStatus {
            GOOD,
            BAD,
            WAITING
        }

        export enum Mode {
            TELEOP,
            TEST,
            AUTONOMOUS
        }

        export interface DriverStationState {
            emergencyStop: boolean,
            enable: boolean,
            mode: Mode,
            teamNumber: number,
            allianceStation: AllianceStation,
            status: DriverStationStatus,
            sequenceNumber: number,
            timeToDisplay: number,
            matchNumber: number,
            eventName: string
        }

        export interface DriverStationConfirmedState {
            isEmergencyStopped: boolean,
            robotCommunicationsActive: boolean,
            canPingRadio: boolean,
            canPingRio: boolean,
            isEnabled: boolean,
            mode: Mode,
            teamNumber: number,
            batteryVoltage: number
        }

        export class DriverStation {
            private constructor(rid: number)

            getConfirmedState(): Promise<DriverStationConfirmedState>

            getState(): Promise<DriverStationState>

            setState(state: DriverStationState): Promise<DriverStationState>

            isInCorrectStation(): Promise<boolean>

            isInMatch(): Promise<boolean>

            getAddress(): Promise<string>

            isClosed(): Promise<boolean>
        }
    }
}declare namespace Nevermore {
    /**
     * The `Nevermore.Database` namespace defines the functions involved with storing and retrieving persistent data using SQL.
     */
    namespace Database {
        /**
         * Retrieves the SQLDatabase assigned for this Plugin. The database is isolated based upon the name its defined with.
         * 
         * This function caches the result until `SQLDatabase.close` is called, so once a database is created it always will return the same database.
         * 
         * @param name The name of the database you want to retrieve.
         */
        export function get(name: string): Promise<SQLDatabase>

        export type ParameterImpl = object | number | string | null

        /**
         * Defines a JSON compatible parameter.
         */
        export type Parameter = ParameterImpl | ParameterImpl[]

        /**
         * An SQL Database backed by SQLite.
         */
        export class SQLDatabase {
            private constructor(rid: number)

            /**
             * Runs an SQL statement without returning anything.
             * 
             * @param stmt The SQL statement.
             * @param params The parameters for the statement.
             */
            run(stmt: string, params: Parameter[]): Promise<void>

            /**
             * Runs an SQL statement and returns the first row.
             * 
             * @param stmt The SQL statement.
             * @param params The parameters for the statement.
             */
            get<T>(stmt: string, params: Parameter[]): Promise<T>

            /**
             * Runs an SQL statement and returns all rows.
             * 
             * @param stmt The SQL statement.
             * @param params The parameters for the statement.
             */
            all<T>(stmt: string, params: Parameter[]): Promise<T[]>

            /**
             * Closes the SQLite database.
             */
            close(): void
        }
    }
}declare namespace Nevermore {
    /**
     * The `Nevermore.PubSub` namespace defines the API used to send messages to and from the frontend.
     */
    namespace PubSub {
        export type PubSubMessageImpl = object | boolean | number | string | null

        /**
         * Message represents and JS object capable of being turned into JSON.
         */
        export type PubSubMessage = PubSubMessageImpl | PubSubMessageImpl[]

        /**
         * A callback for a subscriber returning a JS Object.
         */
        export type PubSubCallback = (message: PubSubMessage) => Promise<void>;

        /**
         * Publishes the specified message to the specified topic.
         * 
         * Can be accessed on the frontend.
         * 
         * @param topic The topic being published to.
         * @param message The message to publish.
         */
        export function publish(topic: string, message: PubSubMessage): Promise<void>;

        /**
         * Subscribes to the specified topic and calls the callback when it receives a message.
         * 
         * @param topic The topic being subscribed to.
         * @param callback The callback to be called with the message as a param.
         */
        export function subscribe(topic: string, callback: PubSubCallback): void;

        /**
         * Unsubscribes from the specified topic and calls the callback when it receives a message.
         * 
         * @param topic The topic being unsubscribed from.
         * @param callback The callback being unsubscribed from.
         */
        export function unsubscribe(topic: string): Promise<void>;
    }
}declare namespace Nevermore {
    /** The `Nevermore.Network` namespace includes functions and types to register a network configurator to the FMS.
     * 
     * Javascript Example:
     * ```js
     * Nevermore.Network.registerConfigurator({
     *      name: "ubiquiti-edgerouter-x-full",
     *      readme: "# Ubqiuiti EdgeRouter Network Stack\nThis is the configuration used by the Field at AMRoC Tampa Bay.",
     *      author: "EAO",
     *      url: "https://edgarallanohms.com",
     *      email: "frcteam5276@gmail.com",
     *      supportedHardware: ["Ubiquiti Controller", "EdgeRouter X"]
     *  }, {
     *      scan: async function(isFactory) {
     *          if (isFactory) {
     *              // Run Factory Scanning Code.
     *              return Nevermore.Network.SUCCESS
     *          } else {
     *              // Run Configured Scanning Code. 
     *              return Nevermore.Network.SUCCESS
     *          }
     *      },
     *      initialConfiguration: async function(password) {
     *          return Nevermore.Network.ERROR("Not implemented")
     *      },
     *      matchConfiguration: async function(allianceStationConfiguration) {
     *          return Nevermore.Network.ERROR("Not implemented")
     *      }
     *  });
     * ```
     */
    namespace Network {
        /**
         * Defines a successful callback from a configurator function.
         */
        export const SUCCESS: Error

        /**
         * Defines an Error/Success returned from a configurator function.
         */
        export type Error = string | null

        /**
         * Defines the info needed to register a configurator.
         * 
         * This info is displayed to end users to help them identify it.
         */
        export interface ConfiguratorInfo {
            name: string,
            readme: string,
            author: string,
            url: string,
            email: string,
            supportedHardware: string[],
            timeout: number
        }

        /**
         * The configuration for an AllianceStation on the Network.
         */
        export interface AllianceStationConfiguration {
            ssid: string,
            password: string
        }

        /**
         * The configurations for each AllianceStation on the Network.
         */
        export interface AllianceStationToConfiguration {
            red1: AllianceStationConfiguration,
            red2: AllianceStationConfiguration,
            red3: AllianceStationConfiguration,
            blue1: AllianceStationConfiguration,
            blue2: AllianceStationConfiguration,
            blue3: AllianceStationConfiguration
        }

        /**
         * The callbacks for a configurator.
         */
        export interface ConfiguratorCallbacks {
            scan: () => Promise<Error>,
            initialConfiguration: () => Promise<Error>,
            matchConfiguration: (allianceStationConfiguration: AllianceStationToConfiguration) => Promise<Error>
        }

        /**
         * Registers a new configurator with the FMS.
         * 
         * @param info The information defining the configurator.
         * @param callbacks The callbacks of the configurator.
         */
        export function registerConfigurator(info: ConfiguratorInfo, callbacks: ConfiguratorCallbacks): void

        /**
         * Creates a new error for a callback.
         * 
         * @param message The message of the error.
         */
        export function ERROR(message: string): Error
    }
}// Copyright 2018-2021 the Deno authors. All rights reserved. MIT license.

/// <reference no-default-lib="true" />
/// <reference lib="esnext" />
/// <reference lib="deno.net" />
  
  declare namespace Deno {
  
    export interface Reader {
      /** Reads up to `p.byteLength` bytes into `p`. It resolves to the number of
       * bytes read (`0` < `n` <= `p.byteLength`) and rejects if any error
       * encountered. Even if `read()` resolves to `n` < `p.byteLength`, it may
       * use all of `p` as scratch space during the call. If some data is
       * available but not `p.byteLength` bytes, `read()` conventionally resolves
       * to what is available instead of waiting for more.
       *
       * When `read()` encounters end-of-file condition, it resolves to EOF
       * (`null`).
       *
       * When `read()` encounters an error, it rejects with an error.
       *
       * Callers should always process the `n` > `0` bytes returned before
       * considering the EOF (`null`). Doing so correctly handles I/O errors that
       * happen after reading some bytes and also both of the allowed EOF
       * behaviors.
       *
       * Implementations should not retain a reference to `p`.
       *
       * Use iter() from https://deno.land/std/io/util.ts to turn a Reader into an
       * AsyncIterator.
       */
      read(p: Uint8Array): Promise<number | null>;
    }
  
    export interface ReaderSync {
      /** Reads up to `p.byteLength` bytes into `p`. It resolves to the number
       * of bytes read (`0` < `n` <= `p.byteLength`) and rejects if any error
       * encountered. Even if `readSync()` returns `n` < `p.byteLength`, it may use
       * all of `p` as scratch space during the call. If some data is available
       * but not `p.byteLength` bytes, `readSync()` conventionally returns what is
       * available instead of waiting for more.
       *
       * When `readSync()` encounters end-of-file condition, it returns EOF
       * (`null`).
       *
       * When `readSync()` encounters an error, it throws with an error.
       *
       * Callers should always process the `n` > `0` bytes returned before
       * considering the EOF (`null`). Doing so correctly handles I/O errors that happen
       * after reading some bytes and also both of the allowed EOF behaviors.
       *
       * Implementations should not retain a reference to `p`.
       *
       * Use iterSync() from https://deno.land/std/io/util.ts to turn a ReaderSync
       * into an Iterator.
       */
      readSync(p: Uint8Array): number | null;
    }
  
    export interface Writer {
      /** Writes `p.byteLength` bytes from `p` to the underlying data stream. It
       * resolves to the number of bytes written from `p` (`0` <= `n` <=
       * `p.byteLength`) or reject with the error encountered that caused the
       * write to stop early. `write()` must reject with a non-null error if
       * would resolve to `n` < `p.byteLength`. `write()` must not modify the
       * slice data, even temporarily.
       *
       * Implementations should not retain a reference to `p`.
       */
      write(p: Uint8Array): Promise<number>;
    }
  
    export interface WriterSync {
      /** Writes `p.byteLength` bytes from `p` to the underlying data
       * stream. It returns the number of bytes written from `p` (`0` <= `n`
       * <= `p.byteLength`) and any error encountered that caused the write to
       * stop early. `writeSync()` must throw a non-null error if it returns `n` <
       * `p.byteLength`. `writeSync()` must not modify the slice data, even
       * temporarily.
       *
       * Implementations should not retain a reference to `p`.
       */
      writeSync(p: Uint8Array): number;
    }
  
    export interface Closer {
      close(): void;
    }
}declare namespace Nevermore {
    /**
     * The `Nevermore.SSH` namespace defines the functions involved with connecting to a remote SSH server.
     */
    namespace SSH {
        /**
         * Connects to a SSH server address, returning a client.
         * 
         * @param address The address to connect to with SSH.
         */
        export function connect(address: string): Promise<Client>

        interface ExecResponse {
            data: Uint8Array,
            exitStatus: number
        }

        /**
         * An SSH Client
         */
        export class Client {
            private constructor(rid: number)

            /**
             * Authenticates the Client with a username and password.
             * 
             * @param username The username used for auth.
             * @param password The password used for auth.
             */
            authenticateWithPassword(username: string, password: string): Promise<void>

            /**
             * Executes a command on it's own SSH channel and returns stdout and the exit status.
             * 
             * 
             * @param wantsReply Should actually reply? Or immediately return.
             * @param command The actual command that is ran.
             */
            exec(wantsReply: boolean, command: string): Promise<ExecResponse>

            /**
             * Closes the SSH connection.
             */
            close(): void
        }
    }
}