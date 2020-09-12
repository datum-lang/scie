cmake_minimum_required(VERSION 3.1)
project(libonigvs C)

add_definitions("-Wall" "-g")

set(_SRCS src/config.h)
set(_SRCS src/regint.h src/regparse.h src/regenc.h src/st.h
        src/regerror.c src/regparse.c src/regext.c src/regcomp.c src/regexec.c
        src/reggnu.c src/regenc.c src/regsyntax.c src/regtrav.c src/regversion.c
        src/st.c src/onig_init.c
        src/unicode.c src/ascii.c src/utf8.c src/utf16_be.c src/utf16_le.c
        src/utf32_be.c src/utf32_le.c src/euc_jp.c src/sjis.c src/iso8859_1.c
        src/iso8859_2.c src/iso8859_3.c src/iso8859_4.c src/iso8859_5.c
        src/iso8859_6.c src/iso8859_7.c src/iso8859_8.c src/iso8859_9.c
        src/iso8859_10.c src/iso8859_11.c src/iso8859_13.c src/iso8859_14.c
        src/iso8859_15.c src/iso8859_16.c src/euc_tw.c src/euc_kr.c src/big5.c
        src/gb18030.c src/koi8_r.c src/cp1251.c
        src/euc_jp_prop.c src/sjis_prop.c
        src/unicode_unfold_key.c
        src/unicode_fold1_key.c src/unicode_fold2_key.c src/unicode_fold3_key.c)
set(_SRCS onigvs.c)

add_library(onigvs STATIC ${_SRCS})

install(TARGETS onigvs DESTINATION .)