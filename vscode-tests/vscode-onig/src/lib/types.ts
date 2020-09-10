export type Pointer = number;

export interface IOnigBinding {
  HEAPU8: Uint8Array;
  HEAPU32: Uint32Array;

  _malloc(count: number): Pointer;
  _free(ptr: Pointer): void;
  UTF8ToString(ptr: Pointer): string;

  _getLastOnigError(): Pointer;
  _createOnigScanner(strPtrsPtr: Pointer, strLenPtr: Pointer, count: number): Pointer;
  _freeOnigScanner(ptr: Pointer): void;
  _findNextOnigScannerMatch(scanner: Pointer, strCacheId: number, strData: Pointer, strLength: number, position: number): number;
  _findNextOnigScannerMatchDbg(scanner: Pointer, strCacheId: number, strData: Pointer, strLength: number, position: number): number;
}

