cmake_minimum_required(VERSION 3.1)
project(libonigvs C)

add_definitions("-Wall" "-g")

set(_SRCS oniguruma/src/regint.h oniguruma/src/regparse.h oniguruma/src/regenc.h oniguruma/src/st.h
        oniguruma/src/regerror.c oniguruma/src/regparse.c oniguruma/src/regext.c oniguruma/src/regcomp.c oniguruma/src/regexec.c
        oniguruma/src/reggnu.c oniguruma/src/regenc.c oniguruma/src/regsyntax.c oniguruma/src/regtrav.c oniguruma/src/regversion.c
        oniguruma/src/st.c oniguruma/src/onig_init.c
        oniguruma/src/unicode.c oniguruma/src/ascii.c oniguruma/src/utf8.c oniguruma/src/utf16_be.c oniguruma/src/utf16_le.c
        oniguruma/src/utf32_be.c oniguruma/src/utf32_le.c oniguruma/src/euc_jp.c oniguruma/src/sjis.c oniguruma/src/iso8859_1.c
        oniguruma/src/iso8859_2.c oniguruma/src/iso8859_3.c oniguruma/src/iso8859_4.c oniguruma/src/iso8859_5.c
        oniguruma/src/iso8859_6.c oniguruma/src/iso8859_7.c oniguruma/src/iso8859_8.c oniguruma/src/iso8859_9.c
        oniguruma/src/iso8859_10.c oniguruma/src/iso8859_11.c oniguruma/src/iso8859_13.c oniguruma/src/iso8859_14.c
        oniguruma/src/iso8859_15.c oniguruma/src/iso8859_16.c oniguruma/src/euc_tw.c oniguruma/src/euc_kr.c oniguruma/src/big5.c
        oniguruma/src/gb18030.c oniguruma/src/koi8_r.c oniguruma/src/cp1251.c
        oniguruma/src/euc_jp_prop.c oniguruma/src/sjis_prop.c
        oniguruma/src/unicode_unfold_key.c
        oniguruma/src/unicode_fold1_key.c oniguruma/src/unicode_fold2_key.c oniguruma/src/unicode_fold3_key.c
        config.h onigvs.c)

add_library(onigvs STATIC ${_SRCS})

install(TARGETS onigvs DESTINATION .)