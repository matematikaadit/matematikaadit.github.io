---
layout: post
title: "Instalasi Hdevtools untuk GHC 7.8.2"
---

Saat tulisan ini ditulis, paket [hdevtools][hdevtools] yang diunggah di
hackage masih belum mendukung GHC versi 7.8.2. Penyebabnya karena perubahan
internal GHC yang mengakibatkan build hdevtools gagal. Sudah ada
[issue][issue] yang dibuat di github dan juga [pull request][pull] yang
menyelesaikan issue ini.  Namun pull request tersebut masih belum di merge
ke master dan masih open di github. Berikut error yang mungkin akan anda
dapati ketika anda melakukan `cabal install hdevtools`:

        Not in scope: data constructor ‘GHC.MatchGroup’
        Perhaps you meant ‘GHC.DocGroup’ (imported from GHC)
    
    src/Info.hs:164:5:
        Not in scope: ‘Pretty.showDocWith’
        Perhaps you meant ‘Pretty.showDoc’ (imported from Pretty)
    
    src/Info.hs:229:12:
        Not in scope:
          type constructor or class ‘PprTyThing.PrintExplicitForalls’
    cabal: Error: some packages failed to install:
    hdevtools-0.1.0.5 failed during the building phase. The exception was:
    ExitFailure 1

Untuk mengatasi error tersebut. Kita bisa install hdevtools dari source
dengan sebelumnya menambahkan patch dari pull request yang saya maksud
diatas.

Pertama, clone repo hdevtools di github. Lalu `cd` kedalam folder baru repo
hdevtools lokal tersebut.

    $ git clone https://github.com/bitc/hdevtools.git
    $ cd hdevtools

Nah, ini trik rahasia pull request di github. Kalian bisa fetch dan merge
pull request kita tadi dengan mengetikkan perintah ini:

    $ git pull origin pull/28/head

Pull request untuk repo di github ada di ref `pull/N/head`. `N` disini
menunjukkan nomor pull request yang dimaksud. Nah, pull request yang
ngeresolve issue kita tadi ini adalah pullrequest #28. Itu sebabnya kita
tambahkan nama ref `pull/28/head` kedalam perintah `git pull origin`. Kalau
anda gak pengen langsung merge, anda bisa tambahkan nama `ref` local tempat
pullreq tadi ditampung. Contohnya sebagai berikut:

    $ git fetch origin pull/28/head:remotes/origin/pr/28

Lalu kalian bisa gunakan perintah berikut untuk checkout pullrequest itu:

    $ git checkout pr/28

Kalau udah puas liat-liatnya, lakukan merge dengan mengetikkan:

    $ git checkout master
    $ git merge pr/28

Untuk ngebuild dan install hdevtools gunakan perintah:

    $ cabal install

NOTE: kalau pengen disanbox dulu ketikkan perintah `cabal sandbox init`
sebelum mengetikkan perintah diatas.

Sekarang hdevtools akan terinstall tanpa masalah.

[issue]: https://github.com/bitc/hdevtools/issues/31
[pull]: https://github.com/bitc/hdevtools/pull/28
[hdevtools]: https://github.com/bitc/hdevtools/
