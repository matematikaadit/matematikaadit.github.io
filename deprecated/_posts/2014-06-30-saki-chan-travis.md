---
layout: "post"
title: "Saki-chan dan Travis"
tags: "haskell"
---

Perkenalkan [saki-chan][saki-chan], teman baru kami para penghuni
**#haskell-id**. Bergabung dengan kami hari sabtu tanggal 28 kemarin.
Keturunan murni dari sesepuhnya [lambdabot][lambdabot] yang terkenal di
**#haskell** itu.

Dengan kemampuan yang sama seperti sesepuhnya, saki-chan kini jadi teman
terbaik kami. Menolong kami saat kami butuhkan. Mengingatkan kami ketika
kami lupa.  Mencatat kata-kata mutiara yang terucap.  Serta membalas mereka
yang berbuat baik.

Tapi karena saki-chan baru saja pindah, ia masih belum bisa banyak bicara
dalam bahasa kami. Rencananya kami yang baik-baik dan suka menolong ini akan
mengajarinya perlahan-lahan. Kalian yang tertarik juga boleh ikutan bantu,
saki-chan selalu terbuka untuk belajar hal baru.

Tapi cerita hari ini bukan itu. Sesuai dengan judul, kita akan bercerita
tentang perkenalan Saki-chan dan [Travis][travis].

## Travis CI

Perkenalan pertama gak seberapa baik. Saki-chan ditolak mentah-mentah. Kita
coba build saki-chan di GHC 7.8 dan GHC 7.6 dan gagal di keduanya.
Alasannya:

1. GHC 7.6 menolak install **lambdabot** karena *exhausted dependency* 
2. GHC 7.8 menolak install paket **haskell-src-exts** tanpa alasan yang
   jelas.

Problem kedua masih belum ketemu solusinya, tapi problem pertama kayaknya
tau apa penyebabnya. Tangani problem satu dulu dan biarkan problem dua
gagal. Tambah baris ini di **.travis.yml**

    matrix:
      allow_failures:
        - ghc: 7.8

Lalu coba teliti kenapa build **lambdabot** di GHC 7.6 gagal. Dugaan pertama
tentunya karena di script **.travis/build** gak nyebutin untuk nginstall
paket **lambdabot-trusted**. Di repo aslinya emang **lambadbot-trusted** gak
disebutin, tapi di repo kita, kita musti instal juga buat mbetulin bug
**mueval** kemarin. Build script yang baru jadi kayak gini:

    #!/usr/bin/env bash
    
    set -e
    
    for dir in lambdabot-core lambdabot-trusted lambdabot-*-plugins lambdabot; do
        (cd "$dir" && cabal install)
    done
    
    echo "Thank you, come again!"

Tapi ternyata masalah belum berakhir. Ada konflik antara versi network yang
diinstall. Akhirnya nambah constraint biar gak terjadi konflik tersebut.
Baris untuk install diubah jadi seperti ini:

    (cd "$dir" && cabal install --constraint="network==2.4.2.3")

Hasilnya build GHC 7.6 sukses. Masalah pertama terselesaikan.

## GHC 7.8 dan haskell-src-exts

Cerita nemuin solusinya sebenarnya panjang. Lewat beberapa debugging dan uji
coba dengan **dummy** project di github. Singkat cerita aja sih, setelah
cari-cari di internet.  Akhirnya nemu halaman ini: [Issue with happy when
building haskell-src-exts][exts-issue]

Ternyata kita kudu install **happy** (versi terbaru) agar bisa install di
GHC 7.8. Ganti lagi isi .travis.yml dengan berikut:

    language: haskell
    ghc:
      - 7.8
      - 7.6
    install: cabal install happy
    script: .travis/build
    branches:
      only:
        - master
    notifications:
      irc: "chat.freenode.net#haskell-id"

Catat bahwa kita udah hapus **allow_failures** di GHC 7.8.

Namun, penderitaan belum berakhir. Ternyata install happy masih bikin
konflik. Paken mtl yang diinstall happy konflik dengan paket mtl punya
lambdabot. Ngubah baris ini betulin konflik tersebut

    install: cabal install happy --constraint="mtl==2.1.3.1"

Sekarang akhirnya si **Travis** dan **saki-chan** bisa berteman baik.

## Referensi

- [Halaman **Saki-chan** di Travis CI][travis-saki]
- [Allow Failures Build Configuration][travis-allow]
- [Speeding the Build][travis-speed]
- [Customizing the Build][travis-custom]
- [Haskell Build Configuration][travis-haskell]
- [Lifecycle of Build][travis-lifecycle]
- [Travis fails with GHC 7.8 - Hawk Repo][travis-hawk]
- [Issue with Happy when Building haskell-src-exts][exts-issue]
- [Installing Packages with Cabal][cabal-doc-install]

[cabal-doc-install]: http://www.haskell.org/cabal/release/cabal-1.20.0.1/doc/users-guide/installing-packages.html
[travis-lifecycle]: http://docs.travis-ci.com/user/build-lifecycle/
[travis-hawk]: https://github.com/gelisam/hawk/issues/115
[travis-haskell]: http://docs.travis-ci.com/user/languages/haskell/
[travis-custom]: http://docs.travis-ci.com/user/customizing-the-build/
[travis-speed]: http://docs.travis-ci.com/user/speeding-up-the-build/
[travis-allow]: http://docs.travis-ci.com/user/build-configuration/#Rows-That-are-Allowed-To-Fail
[travis-saki]: https://travis-ci.org/matematikaadit/saki-chan
[exts-issue]: https://github.com/ghcformacosx/ghc-dot-app/issues/2
[travis]: {% post_url 2014-05-29-travisci %}
[lambdabot]: https://github.com/mokus0/lambdabot
[saki-chan]: https://github.com/matematikaadit/saki-chan
