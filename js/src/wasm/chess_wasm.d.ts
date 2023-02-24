/* tslint:disable */
/* eslint-disable */
/**
*/
export class Chess {
  free(): void;
/**
* @param {string} fen
*/
  constructor(fen: string);
/**
* @param {string} fen
*/
  load(fen: string): void;
/**
* @returns {Array<any>}
*/
  board(): Array<any>;
/**
* @returns {string}
*/
  fen(): string;
/**
* @returns {Array<any>}
*/
  moves(): Array<any>;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_chess_free: (a: number) => void;
  readonly chess_new: (a: number, b: number) => number;
  readonly chess_load: (a: number, b: number, c: number) => void;
  readonly chess_board: (a: number) => number;
  readonly chess_fen: (a: number, b: number) => void;
  readonly chess_moves: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
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
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
