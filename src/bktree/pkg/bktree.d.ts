/* tslint:disable */
/* eslint-disable */
/**
*/
export function main_js(): void;
/**
*/
export class BKTreeWrapper {
  free(): void;
/**
* @returns {BKTreeWrapper}
*/
  static new(): BKTreeWrapper;
/**
* @param {string} value
* @param {(string)[]} row_data
*/
  insert(value: string, row_data: (string)[]): void;
/**
* @param {string} value
* @param {number} tolerance
* @returns {any}
*/
  search(value: string, tolerance: number): any;
/**
* @returns {any}
*/
  save(): any;
/**
* @param {string} data
*/
  load(data: string): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main_js: () => void;
  readonly __wbg_bktreewrapper_free: (a: number) => void;
  readonly bktreewrapper_new: () => number;
  readonly bktreewrapper_insert: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly bktreewrapper_search: (a: number, b: number, c: number, d: number) => number;
  readonly bktreewrapper_save: (a: number) => number;
  readonly bktreewrapper_load: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
