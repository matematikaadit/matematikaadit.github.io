---
layout: post
title: Pengorganisian Kode Go
categories: programming
---

<div style="float:left;">
<img alt="Gopher - Maskot Bahasa Go" src="/images/gopherbw-small.png">
</div>

### Workspace

Kode Go harus berada di dalam _workspace_. Workspace adalah hirarki
direktori dengan tiga direktori di bagian atas sendiri, yakni:

- _src_ berisi file kode Go yang terorganisir kedaalam _packages_ (satu
  package untuk tiap direktori),
- _pkg_ berisi _package objects_, dan
- _bin_ berisi perintah executable/file binari untuk dieksekusi.

Go tool membangun source packages dan menginstal hasil binarinya
masing-masing ke dalam direktori _pkg_ dan _bin_.

Subdirektori `src` biasanya memuat repositori dari version control
system (seperti Git atau Mercurial) yang melacak pengembangan dari satu atau
lebih source packages.

Untuk memberi ide bagaimana suatu workspace terlihat dalam praktiknya,
berikut contohnya:

{% highlight text %}
TODO: copy dari website go
{% endhighlight %}

Workspace diatas memuat dua repositori (hello dan newmath) yang menghasilkan
tiga command (gh, gotour, dan hello) dan empat pustaka (gfx, mixer, sdl, dan
ttf).

Command dan pustaka dibangun dari jenis-jenis source packages yang berbeda.
Kita akan mendiskusikannya kemudian.

### GOPATH environment variabel

Untuk menentukan dimana letak workspace kita, kita perlu mendefinisikan
environment variabel GOPATH. Dan sepertinya hanya environment variabel ini
saja yang perlu kita tentukan untuk mengembangkan kode Go.

Untuk memulai, buat direktori workspace anda dan set GOPATH sesuai
dengannya. Sebagai contoh, $HOME/go akan dijadikan sebagai workspace di
sini.

{% highlight sh %}
$ mkdir $HOME/go
$ export GOPATH=$HOME/go
{% endhighlight %}

untuk lebih mudahnya, tambahkan direktori bin-nya workspace ke dalam PATH:

{% highlight sh %}
$ export PATH=$PATH:$GOPATH/bin
{% endhighlight %}
