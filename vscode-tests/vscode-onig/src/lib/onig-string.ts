import { IOnigBinding, Pointer } from "./types";
import UtfString from "./utf-string";

const onigBinding: IOnigBinding | null = null;

export class OnigString {
  private static LAST_ID = 0;
  private static _sharedPtr: Pointer = 0; // a pointer to a string of 10000 bytes
  private static _sharedPtrInUse = false;

  public readonly id = (++OnigString.LAST_ID);
  private readonly _onigBinding: IOnigBinding;
  public readonly content: string;
  public readonly utf16Length: number;
  public readonly utf8Length: number;
  public readonly utf16OffsetToUtf8: Uint32Array | null;
  public readonly utf8OffsetToUtf16: Uint32Array | null;
  public readonly ptr: Pointer;

  constructor(str: string) {
    if (!onigBinding) {
      throw new Error(`Must invoke loadWASM first.`);
    }
    this._onigBinding = onigBinding;
    this.content = str;
    const utfString = new UtfString(str);
    this.utf16Length = utfString.utf16Length;
    this.utf8Length = utfString.utf8Length;
    this.utf16OffsetToUtf8 = utfString.utf16OffsetToUtf8;
    this.utf8OffsetToUtf16 = utfString.utf8OffsetToUtf16;

    if (this.utf8Length < 10000 && !OnigString._sharedPtrInUse) {
      if (!OnigString._sharedPtr) {
        OnigString._sharedPtr = onigBinding._malloc(10000);
      }
      OnigString._sharedPtrInUse = true;
      onigBinding.HEAPU8.set(utfString.utf8Value, OnigString._sharedPtr);
      this.ptr = OnigString._sharedPtr;
    } else {
      this.ptr = utfString.createString(onigBinding);
    }
  }

  public convertUtf8OffsetToUtf16(utf8Offset: number): number {
    if (this.utf8OffsetToUtf16) {
      if (utf8Offset < 0) {
        return 0;
      }
      if (utf8Offset > this.utf8Length) {
        return this.utf16Length;
      }
      return this.utf8OffsetToUtf16[utf8Offset];
    }
    return utf8Offset;
  }

  public convertUtf16OffsetToUtf8(utf16Offset: number): number {
    if (this.utf16OffsetToUtf8) {
      if (utf16Offset < 0) {
        return 0;
      }
      if (utf16Offset > this.utf16Length) {
        return this.utf8Length;
      }
      return this.utf16OffsetToUtf8[utf16Offset];
    }
    return utf16Offset;
  }

  public dispose(): void {
    if (this.ptr === OnigString._sharedPtr) {
      OnigString._sharedPtrInUse = false;
    } else {
      this._onigBinding._free(this.ptr);
    }
  }
}
