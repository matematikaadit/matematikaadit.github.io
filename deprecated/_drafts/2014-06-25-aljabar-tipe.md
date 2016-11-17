---
layout: "post"
title: "Kenapa Dinamakan Algebraic Data Types?"
tags: "haskell"
---

Karena Aljabar! iyaaa... Aljabar!

Eh, sebelum dilanjut, yang saya bahas ini adalah "Algebraic Data Types"-nya
Haskell. Saya asumsikan kalian udah kenal dengan hal ini. Baca [Making Our
Own Types and Typeclasses][typelyah] dari LYAH buat kalian yang masih awam.

Sekarang lanjut ke Aljabar. Ada tiga komponen utama saat kita bicara
aljabar. Mereka adalah:

- **Objek**
- **Operasi**
- **Aturan**

Yang dipelajari di sekolah: **objeknya**: `1, 2, 3, x, y, z` dan sebagainya.
**operasinya**: kali, tambah, pangkat, dan sebagainya. Sedangkan
**aturan-aturannya** seperti 1 sebagai identitas perkalian, 0 sebagai
identitas penjumlahan, komutatif penjumlahan, distributif perkalian terhadap
penjumlahan, dan sebagainya.

Oke, pak. Kalo di Algebraic Data Type komponen-komponennya tadi apa aja?
Ehm...

## Objek

Berikut "angka nol, satu, dua, tiga" dari ADT (Algebraic Data Type)

    {-# LANGUAGE EmptyDataDecls #-}
    -- biar bisa deklarasi tipe kayak dibawah ini
    data Void    -- nol
    data () = () -- satu, unit, alternatifnya: data Unit = Unit
    data Bool = True | False     -- dua
    data Ordering = LT | EQ | GT -- tiga

Banyaknya value yang mungkin dari struktur data tersebut ditunjukkan
oleh banyaknya data-konstruktor-tanpa-field yang sendirian. Mereka mewakili
"angka" atau **objek** di ADT ini.

Gampangannya, kalau data konstruktornya gak pake field apapun maka dia
mewakili angka satu. Sedangkan simbol `|` tadi mewakili operasi penjumlahan.
Nah, yang diatas jadinya gini kalau diartikan:

    Void = 0
    Unit = 1
    Bool = 1 + 1 = 2
    Ordering = 1 + 1 + 1 = 3

Tapi itu masih angka aja loh ya. Masih **Objek** aja, nah yang berikut ini
adalah **Operasinya**

## Operasi



    

[typelyah]: http://learnyouahaskell.com/making-our-own-types-and-typeclasses
