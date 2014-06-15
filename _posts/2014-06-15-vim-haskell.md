---
layout: post
title: "Vim dan Haskell"
---

Persiapkan editor vim kalian menghadapi pertarungan di medan tempur bersama
Haskell. Berikut ini plugin dan setting vimrc yang bisa jadi acuan.
Diadaptasi dari posting serupa berjudul [A Vim + Haskell
Workflow][vimhaskell] oleh Stephen Diehl. Namun dengan sedikit tambahan
di sana sini.

## haskellmode-vim

Ini nih salah satu plugin yang gak disebutin di postingan yang saya tautkan
diatas. Padahal nih plugin sepertinya wajib buat dimiliki para haskeller
pengguna vim. Salah satu setting yang disebutin oleh Stephen Diehl juga
mengacu pada fungsi yand didefinisikan oleh plugin ini. Berikut cara
instalasi plugin ini. Pastikan kalian sudah pasang [pathogen][pathogen]
terlebih dahulu.

    $ cd ~/.vim/bundle
    $ git clone https://github.com/lukerandall/haskellmode-vim

Berikutnya, jangan lupa setting dua variabel ini di vimrc kalian. Tanpa dua
variabel ini, haskellmode gak akan bisa terpakai secara maksimal (serta jadi
penyebab salah satu bug yang saya temui ketika mengikuti petunjuk Stephen
Diehl).

    " setting haskellmode-vim
    au BufEnter *.hs compiler ghc
    let g:haddock_browser = "firefox"

Baris pertama (setelah comment) diperlukan oleh plugin ini untuk mengetahui
compiler haskell yang kita pakai (ganti dengan compiler lain jika compiler
kalian bukan ghc). Baris kedua diperlukan agar plugin tahu browser apa yang
yang akan digunakan untuk membuka dokumentasi Haddoc.

Dengan mensetting compiler di baris pertama diatas ke ghc, kita akan mendapatkan
fungsi `GHC_BrowseAll()` dan `GHC_ShowType(1)`. Fungsi ini bukan didapat
dari **ghc-mod** seperti apa yang disebutkan oleh Stephen Diehl. Berikut
map yang bisa kalian tambahkan untuk mempermudah menggunakan dua fungsi ini.
(Catatan: `GHC_ShowType(1)` juga bisa dijalankan dengan perintah `_T`,
kalau fungsi `GHC_BrowseAll()` entah lewat perintah apa.)

    " Reload proses GHC
    map <silent> tu :call GHC_BrowseAll()
    " Nambahkan Tipe diatas deklarasi fungsi
    map <silent> tw :call GHC_ShowType(1)

Dengan dua map diatas, kalian bisa mengetikkan perintah `tu` untuk memuat
ulang proses GHC dan perintah `tw` untuk menambahkan type signature diatas
deklarasi fungsi.

Sekian bagian pertama postingan ini, plugin yang lain akan dibahas di
postingan berikut.

[vimhaskell]: http://www.stephendiehl.com/posts/vim_haskell.html
[pathogen]: https://github.com/tpope/vim-pathogen
