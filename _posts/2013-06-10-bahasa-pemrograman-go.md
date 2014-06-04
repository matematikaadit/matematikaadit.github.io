---
layout: post
title: "Bahasa Pemrograman Go"
---

[Golang][0] adalah bahasa pemrograman yang simpel, ringkas, dan efisien.
Diciptakan oleh Rob Pike, Ken Thompson, dan Robert Griesemer. Pertama kali
dikembangkan dan didesain di Google dan kemudian dijadikan proyek open
source. Terkenal di kalangan programmer python dan ruby meskipun pada
awalnya ditujukan sebagai pengganti C dan C++. Karena kelebihan mekanisme
konkurensinya, go banyak digunakan untuk *back-end* pada server.

<h2 class="clear">Memulai Go di Ubuntu</h2>

Berikut instruksi untuk memulai Go di Ubuntu. Diawali dari download
binarinya sampai menjalankan program "hello world".

Download Go di [halaman download proyek Go][1].
Pilih binari untuk linux dan sesuaikan dengan arsitektur prosesor
komputer kalian (x86 32-bit atau x86 64-bit).

Ekstrak hasil download tadi ke `/usr/local`. Misal file yang yang
didownload adalah go1.1.linux-386.tar.gz, maka perintah yang diketikkan
di terminal:

{% highlight console %}
$ sudo tar -C /usr/local -xvf go1.1.linux-386.tar.gz
{% endhighlight %}

Tambahkan `/usr/local/go/bin` ke environment variable PATH.

{% highlight console %}
$ echo 'export PATH=$PATH:/usr/local/go/bin' >> ~/.profile
{% endhighlight %}

Agar bisa langsung digunakan, muat ulang isi dari `~/.profile`.

{% highlight console %}
$ . ~/.profile
{% endhighlight %}

Buat file dengan nama `halo.go` yang isinya:

{% highlight go %}
package main
import "fmt"

func main() {
    fmt.Printf("halo, dunia\n")
}
{% endhighlight %}

Di terminal jalankan perintah berikut:

{% highlight console %}
$ go run halo.go
halo, dunia
{% endhighlight %}

(Diadaptasi dari [Getting Started with Go][2])

[0]: http://golang.org/
[1]: http://code.google.com/p/go/downloads
[2]: http://golang.org/doc/install
