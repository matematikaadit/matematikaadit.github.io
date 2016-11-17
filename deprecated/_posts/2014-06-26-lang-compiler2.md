---
layout: "post"
title: "Languages and Compilers II"
tags: "haskell"
---

Lanjutan dari [Languages and Compilers][tc1].

----

Notasi dari **sequence** di matematika yang kita definisikan kemarin mirip
dengan notasi **list** di Haskell. Bedanya, notasi list di haskell lebih
*precise* ketimbang *sequence*. Ketika bicara tentang bahasa dan grammar,
biasanya perbedaan antara simbol atau *sequence* tunggal dibiarkan implisit.
Namun tidak begitu di Haskell. Karena di Haskell `[a]` dan `a` merupakan dua
objek yang berbeda.

Biasanya huruf alfabet bungsu seperti *a, b, c, ...* dan seterusnya mewakili
simbol tunggal, sedangkan huruf alfabet buncit seperti *z, y, x, w, ...*
mewaikili *sequence*. Penulisan *a* bisa bermakna simbol tunggal *a* atau
*sequence* *aε*, tergantung konteksnya.

*Concatenation* dari simbol dengan *sequence* seperti *az* serta *sequence*
dengan *sequence* seperti *yz* bisa dipahami lewat alfabet yang digunakan.
Tapi di Haskell, dua hal tersebut dibedakan oleh dua operator yang berbeda.
Yang pertama ditangani oleh operator `(:)` sedangkan yang kedua oleh
operator `(++)`.

## Grammar

Palindrom lewat induksi

- *Empty string* ε adalah palindrom
- String yang terdiri dari hanya satu karakter *a, b, dan c* juga palindrom
- Jika *P* adalah palindrom, maka string yang didapatkan dari pengimbuhan
  karakter *a, b, dan c* di depan dan di belakang *P* juga palindrom.
  Yakni string:
  - aPa
  - bPb
  - cPc

Induksi tersebut bisa dituliskan dalam *production rule* (atau
*production* saja) sebagai berikut:

- P → ε 
- P → a
- P → b
- P → c
- P → aPa
- P → bPb
- P → cPc

Di contoh ini himpunan nonterminalnya adalah { P }, sedangkan himpunan
terminalnya adalah {a, b, c}.

----

## Latihan

* Definisikan *words* secara rekursif dengan memanfaatkan fungsi seperti
  *takeWhile*

      import Prelude hiding (words)
      import Data.Char (isSpace)

      words :: String -> [String]
      words xs = case dropWhile isSpace xs of
                   "" -> []
                   ys -> w : words zs
                         where (w, zs) = break isSpace ys

* Definisikan *words* dengan menggunakana *foldr*

      import Prelude hiding (words)                
      import Data.Char (isSpace)                   
                                                  
      words :: String -> [String]
      words = uncurry combine . foldr accumulator ([],"")
          where
          accumulator c (a, w)
              | isSpace c = (combine a w, "")
              | otherwise = (a,c:w)
          combine a ""  = a
          combine a w   = w:a

[tc1]: {% post_url 2014-06-25-lang-compiler %}
