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

Oooh... jangan salah. Komposisi fungsi a.k.a `(.)` itu lebih ...
*composable*. Iyaaa. beneran. *Composition style* nunjukkin bahwa fungsi
kalian adalah unit komposisi yang bisa kalain *mix and match* untuk
mendapatkan apa yang kalian inginkan. Namanya juga functional programming.

Di sisi lain, `($)` itu operator untuk aplikasi fungsi dengan operator
*precedence* paling rendah. Itu sebabnya bisa dipakai juga ngilangin tanda
kurung dari kode yang banyak kurungnya. Kayak spasi yang mewujudkan diri.
Namun terkadang simbol dollar ini justru menghalangi jalan kita para
komposisionist :). Itu sebabnya di Haskell lebih banyak yang suka versi kode
yang disaranin Pak Ganesh tadi. Kata mereka "Gue gak butuh banyak dollar
buat kode ini, enyah kau sana!!!". Emang agak lebay sih...

Sebagai bonus, kalian juga bisa hapus sepenuhnya simbol dollar tersebut dari
fungsi tadi pake *pointless style* berikut:

    numRev :: Int -> Int
    numRev = read . reverse . show

Lebih indah kan (RED: pendapat pribadi).

*Disadur dari pembicaraan di channel #haskell-beginners di freenode*

