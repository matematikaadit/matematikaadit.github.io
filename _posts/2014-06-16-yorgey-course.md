---
layout: post
title: "Yorge Course - Bab 1 - Haskell Basic"
---

Postingan ini adalah bagian dari proyek penerjemahan [Yorgey Course][yorgey]
ke dalam bahasa Indonesia. Bagian **pertama** ini memuat sebagian dari [Bab
1][ch1]. Kelanjutannya akan dimuat di postingan berikutnya.  Terjemahan
dimulai dari paragraph dibawah ini.

------------------------------

## Apa itu Haskell?

Haskell adalah bahasa pemrograman yang *lazy* dan fungsional yang diciptakan
pada akhir tahun 1980-an oleh sekelompok komite akademisi. Pada saat itu,
ada banyak bahasa pemrograman fungsional berseliweran, setiap orang punya
favoritnya sendiri-sendiri dan hal ini mempersulit mereka mengkomunikasikan
ide. Akibatnya sekelompok orang berkumpul bersama dan mendesain bahasa baru
dengan mengambil beberapa ide terbaik dari bahasa yang sudah ada (dan
beberapa ide baru milik mereka sendiri). Akhirnya lahirlah Haskell. 

Jadi, seperti apa Haskell? Ehm. Haskell itu:

**Fungsional**

Tidak ada pengertian yang presisi dan diterima untuk istilah "fungsional".
Tapi ketika kita mengatakan bahwa Haskell adalah bahasa pemrograman
fungsional, kita biasanya punya dua hal di kepala kita:

* Fungsi-nya *first-class*, yakni fungsi adalah nilai yang bisa digunakan
  layaknya nilai-nilai yang lain.
* Arti dari program Haskell berpusat disekitar *mengevaluasi ekspresi*
  ketimbang *mengeksekusi instruksi*.

Perpaduan keduanya menghasilkan cara berfikir pemrograman yang sepenuhnya
berbeda. Kebanyakan waktu kita di semester ini akan dihabiskan
mengeksplorasi cara berfikir ini.

**Pure**

Ekspresi di Haskell selalu *referentially transparent*, yakni:

* Tanpa mutasi! Semuanya (variable, struktur data...) *immutable*
* Ekspresi tidak punya "efek samping" (seperti memperbarui variabel global
  atau mencentak ke layar).
* Memanggil fungsi yang sama dengan argumen yang sama menghasilkan output
  yang sama setiap waktu.

Hal ini sepertinya terdengar gila. Bagaimana mungkin bisa mengerjakan
sesuatu tanpa mutasi dan efek samping? Begitulah, ini tentunya memerlukan
perubahan cara berpikir (itu jika kalian terbiasa dengan paradigma
pemrograman berbasis objek). Tapi setelah kalian mengalami perubahan
tersebut, akan ada beberapa benefit menakjubkan.

* *Equational reasoning* dan *refactoring*. Di Haskell kita bisa "mengganti
  *equals* dengan *equals*", seperti halnya yang kita pelajari di pelajaran
  aljabar.
* *Parallelism*. Mengevaluasi ekspresi secara paralel amatlah mudah ketika
  mereka dijamin tidak mempengaruhi yang lain.
* *Lebih sedikit sakit kepala*. Sederhananya, efek tanpa batasan dan
  *action-at-a-distance*  membuat program sulit di-*debug*, di-*maintain*, dan
  dipertimbangkan.

**Lazy**

Di Haskell, ekspresi *tidak akan dievaluasi sampai hasilnya benar-benar
dibutuhkan*. Hal ini adalah keputusan sederhana dengan konsekuensi yang
merambat kemana-mana, yang akan kita eksplorasi sepanjang semester ini.
Beberapa konsekuensinya antara lain:

* Sangat mudah mendefinisikan *control structure* baru lewat pendefinisian
  fungsi.
* Sangat mungkin mendefinisikan dan bekerja dengan *struktur data tak
  hingga*.
* Menyediakan lebih banyak model pemrograman komposisional (lihat *wholemeal
  programming* di bawah, RED: di postingan berikut).
* Salah satu aspek negatif utamanya adalah pertimbangan terhadap penggunaan
  waktu dan *space* menjadi lebih rumit.

**Statically Typed**

Setiap ekspresi di Haskell memiliki tipe, dan tipe-tipe tersebut semuanya di
periksa pada *waktu kompilasi*. Program dengan kesalahan tipe tidak akan
di-*compile*, apalagi dijalankan.

[yorgey]: http://www.seas.upenn.edu/~cis194/lectures.html
[ch1]: http://www.seas.upenn.edu/~cis194/lectures/01-intro.html
