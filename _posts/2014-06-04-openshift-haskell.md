---
layout: post
title: "Bot IRC - Haskell di OpenShift"
tags: "haskell"
---

Postingan ini adalah bagian dari tutorial membuat IRC Bot kalian sendiri
dengan bahasa pemrograman Haskell. Tapi topik yang akan dibahas sekarang
tidak spesifik hanya untuk keperluan tersebut. Kalian bisa terapkah tutorial
singkat dibawah untuk membuat aplikasi apa saja yang nantinya akan kalian
jalankan di OpenShift.

Kalian yang membaca post ini saya asumsikan sudah mengenal
[openshift][openshift] dan [Haskell][haskell]. Pastikan kalian sudah install
`rhc` (`gem install rhc` aka OpenShift Client Tools) dan melakukan `rhc
setup`. Keluaran yang diharapkan dari tutorial singkat ini berupa website di
OpenShift dengan backend Haskell (untuk keperluan tutorial singkat ini,
package yang dipakai hanya
[network][network]). 

## Membuat Aplikasi Haskell

Ketikkan perintah berikut, ganti `ircbot` dengan nama yang kalian inginkan.

{% highlight console %}
$ rhc app create ircbot \
http://www.accursoft.com/cartridges/network.yml -s
{% endhighlight %}

Tunggu hingga aplikasi kalian selesai terinstall. Akan ada folder baru
dengan nama `ircbot` atau nama yang kalian berikan di perintah diatas tadi.
Masuk ke folder tersebut:

{% highlight console %}
$ cd ircbot
{% endhighlight %}

Untuk mengetahui nama domain dimana aplikasi kalian terinstall gunakan
perintah:

{% highlight console %}
$ rhc app show
{% endhighlight %}

Contohnya aplikasinya bisa kalian lihat di [ircbot-matematikaadit][ircbot].

Sekarang kalian bisa edit file di dalam folder ini, setelahnya lakukan `git
push` untuk mengupdate aplikasi kalian.

## Apa Selanjutnya?

- Gunakan `cabal` dan `cabal sandbox` untuk memanajemen package aplikasi
  kalian. Keterangan lengkapnya lihat di
  [Sandbox dan Instalasi Paket][sandbox]
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
[sandbox]: {% post_url 2014-06-13-ircbot %}
