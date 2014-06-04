---
layout: post
title: "Haskell di OpenShift"
---

Kalian yang membaca post ini saya asumsikan sudah mengenal
[openshift][openshift] dan [Haskell][haskell]. Pastikan kalian sudah install
`rhc` (`gem install rhc` - aka OpenShift Client Tools) dan melakukan `rhc
setup`. Keluaran yang diharapkan dari tutorial singkat ini berupa website di
OpenShift dengan backend Haskell (untuk keperluan tutorial singkat ini,
package yang dipakai hanya
[network]()). 

## Membuat Aplikasi Haskell

Ketikkan perintah berikut, ganti `nama_aplikasi` dengan nama yang kalian inginkana.

```
rhc app create nama_aplikasi http://www.accursoft.com/cartridges/network.yml -s
```

Tunggu hingga aplikasi kalian selesai terinstall. Akan ada folder baru
sesuai dengan `nama_aplikasi` yang kalian berikan di perintah diatas tadi.
Masuk ke folder tersebut:

```
cd nama_aplikasi
```

Untuk mengetahui nama domain dimana aplikasi kalian terinstall gunakan
perintah:

```
rhc app show
```

Contoh aplikasi milik saya hasil dari mengetikkan perintah diatas bisa
kalian lihat di [ircbot-matematikaadit.rhcloud.com][ircbot].

Sekarang kalian bisa edit file di dalam folder ini, setelahnya lakukan `git
push` untuk mengupdate aplikasi kalian.

## Apa Selanjutnya?

- Gunakan `cabal` dan `cabal sandbox` untuk memanajemen package aplikasi
  kalian
- Tambahkan cartridge database jika diperlukan.
- Push di Github dan gunakan [Travis CI]({% post_url 2014-05-29-travisci %})
  untuk mempermudah build dan test aplikasi kalian.

## Sumber Referensi

- [Functional Programming in the Cloud: How to Run Haskell on OpenShift][fpincloud]
- [HaskellWiki: Web/Cloud OpenShift][wiki]


[wiki]: http://www.haskell.org/haskellwiki/Web/Cloud#OpenShift
[openshift]: https://www.openshift.com/
[haskell]: http://haskell-lang.org/
[network]: https://hackage.haskell.org/package/network
[ircbot]: http://ircbot-matematikaadit.rhcloud.com/
[fpincloud]: https://www.openshift.com/blogs/functional-programming-in-the-cloud-how-to-run-haskell-on-openshift
