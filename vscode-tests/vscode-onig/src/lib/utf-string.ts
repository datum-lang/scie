import {IOnigBinding, Pointer} from "./types";

export default class UtfString {
  private static _utf8ByteLength(str: string): number {
    let result = 0;
    for (let i = 0, len = str.length; i < len; i++) {
      const charCode = str.charCodeAt(i);

      let codepoint = charCode;
      let wasSurrogatePair = false;

      if (charCode >= 0xd800 && charCode <= 0xdbff) {
        // Hit a high surrogate, try to look for a matching low surrogate
        if (i + 1 < len) {
          const nextCharCode = str.charCodeAt(i + 1);
          if (nextCharCode >= 0xdc00 && nextCharCode <= 0xdfff) {
            // Found the matching low surrogate
            codepoint = (((charCode - 0xd800) << 10) + 0x10000) | (nextCharCode - 0xdc00);
            wasSurrogatePair = true;
          }
        }
      }

      if (codepoint <= 0x7f) {
        result += 1;
      } else if (codepoint <= 0x7ff) {
        result += 2;
      } else if (codepoint <= 0xffff) {
        result += 3;
      } else {
        result += 4;
      }

      if (wasSurrogatePair) {
        i++;
      }
    }

    return result;
  }

  public readonly utf16Length: number;
  public readonly utf8Length: number;
  public readonly utf16Value: string;
  public readonly utf8Value: Uint8Array;
  public readonly utf16OffsetToUtf8: Uint32Array | null;
  public readonly utf8OffsetToUtf16: Uint32Array | null;

  constructor(str: string) {
    const utf16Length = str.length;
    const utf8Length = UtfString._utf8ByteLength(str);
    const computeIndicesMapping = (utf8Length !== utf16Length);
    const utf16OffsetToUtf8 = computeIndicesMapping ? new Uint32Array(utf16Length + 1) : null!;
    if (computeIndicesMapping) {
      utf16OffsetToUtf8[utf16Length] = utf8Length;
    }
    const utf8OffsetToUtf16 = computeIndicesMapping ? new Uint32Array(utf8Length + 1) : null!;
    if (computeIndicesMapping) {
      utf8OffsetToUtf16[utf8Length] = utf16Length;
    }
    const utf8Value = new Uint8Array(utf8Length);

    let i8 = 0;
    console.log("utf16Length: " + utf16Length);
    for (let i16 = 0; i16 < utf16Length; i16++) {
      const charCode = str.charCodeAt(i16);
      console.log(charCode);

      let codePoint = charCode;
      let wasSurrogatePair = false;

      if (charCode >= 0xd800 && charCode <= 0xdbff) {
        // Hit a high surrogate, try to look for a matching low surrogate
        if (i16 + 1 < utf16Length) {
          const nextCharCode = str.charCodeAt(i16 + 1);
          if (nextCharCode >= 0xdc00 && nextCharCode <= 0xdfff) {
            // Found the matching low surrogate
            codePoint = (((charCode - 0xd800) << 10) + 0x10000) | (nextCharCode - 0xdc00);
            wasSurrogatePair = true;
          }
        }
      }

      if (computeIndicesMapping) {
        utf16OffsetToUtf8[i16] = i8;
        if (wasSurrogatePair) {
          utf16OffsetToUtf8[i16 + 1] = i8;
        }

        if (codePoint <= 0x7f) {
          utf8OffsetToUtf16[i8 + 0] = i16;
        } else if (codePoint <= 0x7ff) {
          utf8OffsetToUtf16[i8 + 0] = i16;
          utf8OffsetToUtf16[i8 + 1] = i16;
        } else if (codePoint <= 0xffff) {
          utf8OffsetToUtf16[i8 + 0] = i16;
          utf8OffsetToUtf16[i8 + 1] = i16;
          utf8OffsetToUtf16[i8 + 2] = i16;
        } else {
          utf8OffsetToUtf16[i8 + 0] = i16;
          utf8OffsetToUtf16[i8 + 1] = i16;
          utf8OffsetToUtf16[i8 + 2] = i16;
          utf8OffsetToUtf16[i8 + 3] = i16;
        }
      }

      if (codePoint <= 0x7f) {
        utf8Value[i8++] = codePoint;
      } else if (codePoint <= 0x7ff) {
        utf8Value[i8++] = 0b11000000 | ((codePoint & 0b00000000000000000000011111000000) >>> 6);
        utf8Value[i8++] = 0b10000000 | ((codePoint & 0b00000000000000000000000000111111) >>> 0);
      } else if (codePoint <= 0xffff) {
        utf8Value[i8++] = 0b11100000 | ((codePoint & 0b00000000000000001111000000000000) >>> 12);
        utf8Value[i8++] = 0b10000000 | ((codePoint & 0b00000000000000000000111111000000) >>> 6);
        utf8Value[i8++] = 0b10000000 | ((codePoint & 0b00000000000000000000000000111111) >>> 0);
      } else {
        utf8Value[i8++] = 0b11110000 | ((codePoint & 0b00000000000111000000000000000000) >>> 18);
        utf8Value[i8++] = 0b10000000 | ((codePoint & 0b00000000000000111111000000000000) >>> 12);
        utf8Value[i8++] = 0b10000000 | ((codePoint & 0b00000000000000000000111111000000) >>> 6);
        utf8Value[i8++] = 0b10000000 | ((codePoint & 0b00000000000000000000000000111111) >>> 0);
      }

      if (wasSurrogatePair) {
        i16++;
      }
    }

    this.utf16Length = utf16Length;
    this.utf8Length = utf8Length;
    this.utf16Value = str;
    this.utf8Value = utf8Value;
    this.utf16OffsetToUtf8 = utf16OffsetToUtf8;
    this.utf8OffsetToUtf16 = utf8OffsetToUtf16;
  }


  public createString(onigBinding: IOnigBinding): Pointer {
    const result = onigBinding._malloc(this.utf8Length);
    onigBinding.HEAPU8.set(this.utf8Value, result);
    return result;
  }
}
