---
layout: "post"
title: ". versus $"
tags: "haskell"
---

Pak Dengklek punya kode Haskell seperti ini:

    numRev :: Int -> Int
    numRev n = read (reverse (show n))

Tapi karena tidak suka dengan banyaknya tanda kurung di kode tersebut, ia
merubahnya menjadi seperti ini.

    numRev :: Int -> Int
    numRev n = read $ reverse $ show $ n

Pak Ganesh yang kebetulan berada di sebelah Pak Dengklek menyuarakan protes
dan menyarankan Pak Dengklek untuk menggantinya jadi seperti ini.

    numRev :: Int -> Int
    numRev n = read . reverse . show $ n

Nah, loh. Emang apa bedanya, tanya Pak Dengklek. Cuman ganti simbol `$`
dengan `.` aja kan.

Oooh... jangan salah. Menggunakan `(.)` a.k.a. *composition* menunjukkan
bahwa program kalian lebih *composable* ketimbang menggunakan operator
`($)`. Kan salah satu keunggulan pemrograman fungsional adalah kalian bisa
dengan mudah dan gratis mengkomposisikan fungsi-fungsi yang kalian
definisikan. Kalo bisa di *compose*, kenapa tidak?

Itu sebabnya di Haskell lebih banyak yang suka versi kode yang disaranin Pak
Ganesh tadi. Bahkan, beberapa orang memilih untuk bikin kode tersebut jadi
*pointless* seperti ini:

    numRev :: Int -> Int
    numRev = read . reverse . show

Yang tentunya gak bisa dilakukan kalau pake `($)`.
