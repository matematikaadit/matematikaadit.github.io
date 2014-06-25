---
layout: "post"
title: "Languages and Compilers"
tags: "haskell"
---

Rangkuman sementara dari hasil baca [Lecture Notes][lnotes] kuliah *Talen n
Compillers* a.k.a *Languages and Compilers* dari Utrecht University. Lihat
juga [*slides*-nya][lslides] yang merupakan versi ramping dari *lecture
notes* tadi. Selain itu juga saya sertakan pembahasan masalah **P-0** yang
diberikan di [Course Assigments Practicals][lpracticals]

------

## Rangkuman

Definisi-definisi.

**sequence**

Diberikan himpunan *X*. **sequence** diatas himpunan *X*, disebut *X*\*,
didefinisikan secara rekursif sebagai berikut:

- ε adalah *sequence*, disebut *sequence* kosong,
- jika *z* adalah *sequence* dan *a* adalah anggota dari X, maka *az* juga
  merupakan *sequence*.

**alfabet**

Himpunan hingga simbol-simbol

**bahasa**

Himpunan bagian dari *T*\*, untuk suatu alfabet *T*.

**kalimat**

Juga disebut **kata**, adalah elemen dari **bahasa**.

Contoh:

Himpunan `{ kuliah, praktik, latihan, ujian }` adalah bahasa diatas alfabet
`{ a,h,i,j,k,l,n,p,r,t,u }`, salah satu kalimat/kata di bahasa tersebut
adalah `kuliah`.

----

## Practices

- Definisi *unwords* menggunakan rekursi

      import Prelude hiding (unwords)

      unwords :: [String] -> String
      unwords []     = []
      unwords [x]    = x
      unwords (x:xs) = x ++ " " ++ unwords xs

- Definisi *unwords* menggunakan *foldr*

      import Prelude hiding (unwords)

      unwords :: [String] -> String
      unwords []  = ""
      unwords xs  = foldr (\w s -> w ++ " " ++ s) (last xs) (init xs)

Problem selanjutnya di *practice* ini akan dibahas di lain kesempatan.


[lnotes]: http://www.cs.uu.nl/wiki/TC/CourseMaterials#Lecture_Notes
[lslides]: http://www.cs.uu.nl/wiki/TC/CourseMaterials#Slides
[lpracticals]: http://www.cs.uu.nl/wiki/TC/CourseAssignments#Practicals
