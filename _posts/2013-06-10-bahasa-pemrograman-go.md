---
layout: post
title: Bahasa Pemrograman Go
---

{{ page.title }}
================

<p class="meta">10 Juni 2013</p>

![Gopher - Maskot Bahasa Go](/images/gopherbw-small.png)

[Go][0] adalah bahasa pemrograman yang simpel, ringkas, dan efisien.
Diciptakan oleh Rob Pike, Ken Thompson, dan Robert Griesemer. Pertama kali
dikembangkan dan didesain di Google dan kemudian dijadikan proyek open
source. Terkenal di kalangan programmer python dan ruby meskipun pada
awalnya ditujukan sebagai pengganti C dan C++. Karena kelebihan mekanisme
konkurensinya, go banyak digunakan untuk *back-end* pada server.

Memulai Go di Ubuntu
--------------------

Berikut instruksi untuk memulai Go di Ubuntu. Diawali dari download
binarynya sampai menjalankan program "hello world".

1. Download Go di [halaman download proyek Go][1].
   Pilih binary untuk linux dan sesuaikan dengan arsitektur prosesor
   komputer kalian (x86 32-bit atau x86 64-bit).

2. Ekstrak hasil download tadi ke `/usr/local`. Misal file yang yang
   didownload adalah go1.1.linux-386.tar.gz, maka perintah yang diketikkan
   di terminal:

        $ sudo tar -C /usr/local -xvf go1.1.linux-386.tar.gz

3. Tambahkan `/usr/local/go/bin` ke environment variable PATH.

        $ echo 'export PATH=$PATH:/usr/local/go/bin' >> ~/.profile

4. Agar bisa langsung digunakan, muat ulang isi dari `~/.profile`.

        $ . ~/.profile

5. Buat file dengan nama `halo.go` yang isinya:

        package main
        import "fmt"

        func main() {
            fmt.Printf("halo, dunia\n")
        }

6. Di terminal jalankan perintah berikut:

        $ go run halo.go 
        halo, dunia

(Diadaptasi dari [Getting Started with Go][2])

[0]: http://golang.org/
[1]: http://code.google.com/p/go/downloads
[2]: http://golang.org/doc/install
