---
layout: post
title: "XPath dan Animetake"
categories: programming
date: 2013-11-05
---

![Halaman Web Animetake](/images/ff-atk.png)

Hari ini ada [diskusi menarik tentang xpath][hn-xpath] di Hacker News.
Judulnya [XPath is actually pretty useful ...][xpath-useful]. Dan apa
yang disampaikan disana memang benar. XPath memang sangat berguna
ketika kita udah gak bingung dengannya.

Kalian yang belum pernah dengar pasti bertanya-tanya dalam hati apa
itu XPath? Sedikit mengutip [W3C Recomendation][w3-xpath], dikatakan
bahwa XPath adalah bahasa yang digunakan untuk memberikan alamat pada
bagian dari dokumen XML. Layaknya URL yang dipakai untuk memberi
alamat halaman web, tapi XPath ini untuk XML. Dan seperti yang kalian tahu,
salah satu jenis dokumen XML yang seringkali kita jumpai adalah
dokumen HTML.

Jadi, XPath mirip dengan CSS Selector. Bedanya, XPath lebih fleksibel,
powerful dan general. Tapi emang sih agak membingungkan. Pas pertama
ketemu XPath dulu (sekitar tahun 2011), gak ada referensi di internet
yang ngejelasin dengan detail tentang XPath ini. Satu-satunya sumber
yang terpercaya dan mendalam ya halaman [W3C Recomendation][w3-xpath]
tadi.

Di Post ini agar lebih kontekstual, berikut saya berikan suatu masalah
yang akan kita pecahkan dengan bantuan XPath tadi. Dan sebagai bonus
extra, saya cantumkan programnya.

Masalah pertama, ambil semua daftar link di section "The Latest Anime
Episodes" yang mengarah ke halaman download di [animetake.com][animetake].
Simpel dan mudah. Serta cocok untuk dijadikan contoh awal kita sekarang.

Sebelum masuk ke penjelasan alamat XPath-nya, kita lihat dulu dimana di
halaman link-link tadi berada. Langkah pertama yang harus dilakukan adalah
periksa _page source_ dari halaman tersebut. Biar gampang kita download aja
sourcenya pake `wget` dengan menggunakan perintah:

```text
$ wget http://www.animetake.com
--2013-11-05 07:24:29--  http://www.animetake.com/
Resolving www.animetake.com (www.animetake.com)... 217.23.12.120
Connecting to www.animetake.com (www.animetake.com)|217.23.12.120|:80... connected.
HTTP request sent, awaiting response... 200 OK
Length: unspecified [text/html]
Saving to: ‘index.html’

    [      <=>                            ] 42,241      21.0KB/s   in 2.0s

2013-11-05 07:24:34 (21.0 KB/s) - ‘index.html’ saved [42241]
```

Buka file `index.html` hasil download diatas. Search "The Latest Anime
Episodes", judul dari bagian halaman yang jadi acuan pencarian kita. Lalu
catat dimana letak link-link yang kita inginkan tadi. Hasil pengamanatan per
2013-11-05 07:24:34 menunjukkan bawha link yang kita inginkan ada di element
`<a>` _pertama_ di _dalam_ `<h4>` di dalam `<div class="updateinfo">`.

![Search The Latest Anime](/images/subl-atk.png)

Berdasarkan informasi tersebut, kita susun alamat XPath:

```text
//div[@class="updateinfo"]/h4/a
```

Penjelsannya sebagai berikut:
- Pertama kita cari semua elemen di dalam dokumen
- Setelah itu cari semua `div` dengan attribute: `class` yang 
  isinya `"updateinfo"`
- Cari `h4` di dalam `div` tadi
- Terakhir pilih `a` di dalam `h4` tadi

Masalah kedua, daftar semua link download ke torrent yang tercantum di
halaman yang ditautkan dari masalah pertama diatas. Untuk mencari link
download tersebut, kita download beberapa source halaman di animetake
dan bandingkan antara satu dengan yang lain untuk menemukan pola
umumnya.

seperti sebelumnya, pake `wget` di terminal:

```text
$ wget -O atk-umd.html \
'http://www.animetake.com/kikou-shoujo-wa-kizutsukanai-episode-5/'
$ wget -O atk-nnb.html \
'http://www.animetake.com/non-non-biyori-episode-5/'
```

Disini, kita beri nama file yang kita download masing-masing 
`atk-umd.html` dan `atk-nnb.html` dengan menggunakan opsi `-O` (huruf O
besar: singkatan dari Output file) untuk memberi nama file yang
disimpan. Buka kedua file di text editor. Dan cari link yang mengarah
ke alamat download torrent yang kita inginkan.

![Search torrent download](/images/subl-atk-dl.png)

Setelah sedikit diteliti, ternyata semua torrent terbungkus rapi di
dalam elemen `<a>` yang merupakan _child_ dari `<li class="tor">`.
Siapa yang menduga akan sesederhana ini.

XPath untuk elemen yang ini adalah:

```text
//li[@class="tor"]/a
```

Penjelasannya seperti ini:
- Pertama, cari semua elemen di dalam dokumen
- Lalu cari `<li>` dengan `class` `"tor"`
- Terakhir, pilih `<a>` yang langsung jadi _child_-nya `<li>` diatas.

Nah, dua masalah terselesaikan. Sekarang mari kita bikin programnya. Eits,
tunggu-tunggu. Saya belum jelasin ya kita mau bikin apaan pake XPath ini?
Dengerin baik-baik ya. Kita akan bikin animetake versi lite. Yap, animetake
versi tandingan yang isinya cuma link ke tempat download torrentnya. Langsung jadi satu halaman. Tanpa loading lama-lama.

Tapi berhubung waktu yang tak memungkinkan, kita tunda programnya di
pertemuan berikutnya. Sampai ketemu lagi :)

[hn-xpath]: http://news.ycombinator.com/item?id=6669338
[xpath-useful]: http://news.rapgenius.com/Mat-brown-xpath-is-actually-pretty-useful-once-it-stops-being-confusing-lyrics
[w3-xpath]: http://www.w3.org/TR/xpath/
[animetake]: http://www.animetake.com/