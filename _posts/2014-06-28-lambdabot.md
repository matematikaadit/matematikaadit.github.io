---
layout: "post"
title: "Lambdabot"
tags: "haskell"
---

Sulit banget cari dokumentasi Lambdabot yang lengkap. Terutama kalau
tujuannya pengen oprek lambdabot untuk keperluan sendiri. Googling pun gak
terlalu banyak ngebantu. Halaman wikinya juga gak nyebutin gimana kalau kita
pengen mulai pakai lambdabot. Mesti pinter-pinter cari dari berbagai sumber.

Nah, berikut ini cerita gimana perjuangan untuk instalasi lambdabot, cara
pakainya, serta beberapa cara buat perbaiki error yang dihadapi di sepanjang
jalan.

Berikut spesifikasi sistem saya, mungkin ngebantu kalau ada beberapa hal
beda yang kalian temui nanti:

- OS: Ubuntu 13.03
- Versi GHC: 7.8.2
- Versi cabal-install: 1.20.0.2
- Versi GCC: 4.7.3

## Instalasi

Pertama, *clone* [repo lambdabot][repolambda] dari github. Perintah-perintah
selanjutnya akan kita eksekusi dari direktori *lambdabot/lambdabot* hasil
*cloningan* ini.

    $ git clone https://github.com/mokus0/lambdabot
    $ cd lambdabot/lambdabot

Setelah itu, kita buat sandbox baru agar paket-paket milik lambdabot tidak
bercampur dengan paket-paket milik `--user`. Serta tambahkan paket-paket
*lambdabot-related* yang ada di root repo ini sebagai source.

    $ cabal sandbox init
    $ for file in ../lambdabot-*; do \
        cabal sandbox add-source $file; \
      done

Nah, berikutnya dilanjut dengan instalasi. Sebenarnya kemarin versi
lambdabot yang ada di *branch master* github masih punya *bug* untuk GHC
versi 7.8.2, untungnya dan kebetulan banget [*pull request*][prlambda] yang
perbaikin bug ini udah di merge sama maintainternya sekitar 17 jam yang lalu
(ketika tulisan ini dibuat). Jadi perintah berikut aman buat diketikkan.

    $ cabal install

Sekarang setelah sukses terinstal, mari kita jalankan:

    $ cabal exec lambdabot
    [NOTICE] : Initialising plugins
    [ERROR] Plugin.djinn: Djinn command failed: djinn: readProcess:
    runInteractiveProcess: exec: does not exist (No such file or directory)
    [WARNING] Plugin.seen: WARNING: failed to read Seen module state:
    Data.Binary.Get.runGet at position 0: demandInput: not enough bytes
    [NOTICE] : Done loading plugins
    lambdabot>

## Djinn

Yang terakhir diatas gara-gara djinn belum terinstall. Agar kita bisa
jalankan plugin Djinn, kita juga harus install binary djinn dari paket
[djinn][djinn] di hackage. Namun sayang paket ini bermasalah ketika
diinstall dengan GCC versi 4.7.3. Untuk itu kita perlu install dari [repo
djinn yang ini][selfdjinn].

    $ cd ../..
    $ git clone http://github.com/matematikaadit/djinn
    $ cd djinn
    $ cabal sandbox init --sandbox=../lambdabot/lambdabot/.cabal-sandbox
    $ cabal install
    $ cd ../lambdabot/lambdabot

NOTE: kalau versi GCC kalian 4.6.3, kalian bisa langsung ketikkan perintah
`caball install djinn` di dalam directory lambdabot/lambdabot tadi tanpa
harus ikuti perintah diatas.

Sekarang kalian bisa eksekusi lambdabot tanpa masalah.

    $ cabal exec lambdabot
    [NOTICE] : Initialising plugins
    [NOTICE] : Done loading plugins
    lambdabot>

Tekan CTRL-D untuk exit lambdabot.

## Mueval dan Trusted Package

Tapi masalah berikutnya muncul kalau kalian coba eksekusi ekspresi haskell
(pake plugin mueval). Kayak berikuti ni

    lambdabot> > 5 + 6
    Could not find module Lambdabot.Plugin.Haskell.Eval.Trusted
    Use -v to see a list of the files searched for.
    lambdabot>

Penyebabnya karena modul yang disebutin diatas gak ditemuin oleh lambdabot.
Oleh sebab itu kita butuh beberapa perbaikan sebagai berikut.

Pertama, tambahakan lambdabot-trusted ke daftar build-depends di
lambdabot.cabal

    -- File lambdabot/lambdabot/lambdabot.cabal
    -- Sebagian isi file tidak ditampilkan
    ghc-options: -Wall -threaded
    build-depends: base >= 3 && < 5,
                   lambdabot-core,
                   lambdabot-haskell-plugins,
                   lambdabot-irc-plugins,
                   lambdabot-misc-plugins,
                   lambdabot-novelty-plugins,
                   lambdabot-reference-plugins,
                   lambdabot-social-plugins,
                   lambdabot-trusted

Entah kenapa paket lambdabot-trusted yang ada di root repo tidak di jadikan
salah satu build-depends dari lambdabot. Oh, ya, jangan lupa kasih koma
setelah lambdabot-social-plugins.

Selanjutnya, kita tambahkan lambdabot-trusted sebagai paket yang secara
default di percaya oleh lambdabot. Ganti file berikut ini

    -- File lambdabot-haskell-plugins/src/Lambdabot/Config/Haskell.hs
    -- Sebagian isi file tidak ditampilkan
    import Lambdabot.Config
    
    config "evalPrefixes"       [t| [String]                |] [| [">"]         |]
    
    trustedPkgs :: [String]
    trustedPkgs =
        [ "array"
        , "base"
        , "bytestring"
        , "containers"
        , "lambdabot-haskell-plugins"
        , "lambdabot-trusted"
        , "random"
        ]

Setelah itu lakukan install sekali lagi.

    $ cabal install

## L.hs dan Pristine.hs serta deprecated module di GHC 7.8.2

Untuk mereka pengguna GHC versi 7.8.2, akan ditemukan satu masalah lagi,
yakni ada import yang di deprecated oleh GHC. Berikut ini pesan error yang
mungkin kalian temui.

    $ cabal exec lambdabot -- -e '> 5 + 6'
    [NOTICE] : Initialising plugins
    [NOTICE] : Done loading plugins
     mueval-core: GHC returned a result but said: [GhcError {errMsg =
     "L.hs:37:1:\n    Module \8216Control.Monad.Instances\8217 is
     deprecated:\n      This module now contains no instances and will be
     removed in the future"}]

Kita butuh cpp language pragma disini. Tambahkan baris ini di file
Pristine.hs lalu copy isi file tersebut ke L.hs

    -- File lambdabot/labmdabot/State/Pristine.hs
    {-# LANGUAGE UnicodeSyntax #-}
    {-# LANGUAGE CPP #-}
    module L where
    import Control.Applicative
    import Control.Arrow
    import Control.Monad
    import Control.Monad.Cont
    import Control.Monad.Identity
    #if __GLASGOW_HASKELL__ < 708
    import Control.Monad.Instances
    #endif
    import Control.Monad.Reader
    import Control.Monad.ST.Safe

Kita tambahkan CPP language pragma dan `#if` guard untuk baris import
Control.Monad.Instances agar modul ini diimport hanya untuk haskell versi
dibawah 7.8. Catat bahwa versinya ditulis dalam format xyy dimana x adalah
versi mayor, dan yy adalah dua digit versi minor, mengunakan leading zero
jika hanya satu digit. Sekarang copy ke L.hs, lalu instal ulang.

    $ cp State/Pristine.hs State/L.hs
    $ cabal install

Nah, akhirnya setelah perjalan panjang ini, kita bisa eksekusi ekspresi
haskell di lambdabot.

    $ cabal exec lambdabot -- -e '> 5 + 6'
    [NOTICE] : Initialising plugins
    [NOTICE] : Done loading plugins
     11
    $

## Referensi

- [Lambdabot oleh @computionist][complambda]
- [Pull Request yang fix GHC 7.8.2 build][prlambda]
- [Repo lambdabot][repolambda]
- [Repo Paket Djinn dengan Patch][selfdjinn]
- [Control.Monad.Instances source (deprecated)][cmi]
- [CPP preprocessor in GHC][cpp]
- [CPP LANGUAGE Pragma][cpppragma]

[cpppragma]: http://www.haskell.org/ghc/docs/7.2.1/html/users_guide/pragmas.html
[cpp]: http://www.haskell.org/ghc/docs/7.0-latest/html/users_guide/options-phases.html#c-pre-processor
[repolambda]: https://github.com/mokus0/lambdabot
[prlambda]: https://github.com/mokus0/lambdabot/pull/78
[complambda]: http://www.おまかせ.net/posts/2014-06-25-lambdabot.html
[djinn]: http://hackage.haskell.org/package/djinn
[selfdjinn]: https://github.com/matematikaadit/djinn
[cmi]: https://hackage.haskell.org/package/base-4.7.0.0/docs/src/Control-Monad-Instances.html
