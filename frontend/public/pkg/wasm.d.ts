declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	*/
	export class RandomCircle {
	  free(): void;
	/**
	* @returns {Int32Array}
	*/
	  center: Int32Array;
	/**
	* @returns {Uint8Array}
	*/
	  color: Uint8Array;
	/**
	*/
	  imgx: number;
	/**
	*/
	  imgy: number;
	/**
	*/
	  radius: number;
	}
	/**
	*/
	export class TestStruct {
	  free(): void;
	/**
	* @param {string} url
	* @returns {Promise<TestStruct>}
	*/
	  static new_async(url: string): Promise<TestStruct>;
	/**
	* @param {ArrayBuffer} buffer
	* @returns {TestStruct}
	*/
	  static new_from_buffer(buffer: ArrayBuffer): TestStruct;
	/**
	* @returns {any}
	*/
	  get_image_data(): any;
	/**
	* @param {number} generation_size
	* @param {number} num_gens
	* @returns {RandomCircle | undefined}
	*/
	  try_epoch(generation_size: number, num_gens: number): RandomCircle | undefined;
	/**
	* @returns {number}
	*/
	  get_target_width(): number;
	/**
	* @returns {number}
	*/
	  get_target_height(): number;
	}
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_teststruct_free: (a: number) => void;
  readonly teststruct_new_async: (a: number, b: number) => number;
  readonly teststruct_new_from_buffer: (a: number) => number;
  readonly teststruct_get_image_data: (a: number, b: number) => void;
  readonly teststruct_try_epoch: (a: number, b: number, c: number) => number;
  readonly teststruct_get_target_width: (a: number) => number;
  readonly teststruct_get_target_height: (a: number) => number;
  readonly __wbg_randomcircle_free: (a: number) => void;
  readonly __wbg_get_randomcircle_imgx: (a: number) => number;
  readonly __wbg_set_randomcircle_imgx: (a: number, b: number) => void;
  readonly __wbg_get_randomcircle_imgy: (a: number) => number;
  readonly __wbg_set_randomcircle_imgy: (a: number, b: number) => void;
  readonly __wbg_get_randomcircle_radius: (a: number) => number;
  readonly __wbg_set_randomcircle_radius: (a: number, b: number) => void;
  readonly randomcircle_center: (a: number) => number;
  readonly randomcircle_set_center: (a: number, b: number, c: number) => void;
  readonly randomcircle_color: (a: number) => number;
  readonly randomcircle_set_color: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he98d7a80947c847a: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h10458065e49ae6b0: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
