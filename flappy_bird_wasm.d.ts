/* tslint:disable */
/* eslint-disable */
export enum GameState {
  Menu = 0,
  Playing = 1,
  Paused = 2,
  GameOver = 3,
}
export class Game {
  free(): void;
  constructor(canvas: HTMLCanvasElement);
  load_assets(): Promise<void>;
  update(): void;
  render(): void;
  flap(): void;
  toggle_pause(): void;
  start_game(): void;
  quit_game(): void;
  get_state(): GameState;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_game_free: (a: number, b: number) => void;
  readonly game_new: (a: number, b: number) => void;
  readonly game_load_assets: (a: number) => number;
  readonly game_update: (a: number) => void;
  readonly game_render: (a: number) => void;
  readonly game_flap: (a: number) => void;
  readonly game_toggle_pause: (a: number) => void;
  readonly game_start_game: (a: number) => void;
  readonly game_quit_game: (a: number) => void;
  readonly game_get_state: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_export_1: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h62c25a7f3d840362: (a: number, b: number, c: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h72fe6758d1220524: (a: number, b: number, c: number, d: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
