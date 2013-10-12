---
layout: post
title: Bagaimana Menulis Kode Go
categories: programming
---

![Gopher - Maskot Bahasa Go](/images/gopherbw-small.png)

Organisasi Kode
---------------

### Workspace

Kode Go harus tetap berada di dalam _workspace_. Workspace adalah hirarki
direktori dengan tiga direktori di bagian atas sendiri, yakni:

- _src_ berisi file kode Go yang terorganisir kedaalam _packages_ (satu
  package untuk tiap direktori),
- _pkg_ berisi _package objects_, dan
- _bin_ berisi perintah executable.

Go tool membangun source packages dan menginstal hasil binarinya
masing-masing ke dalam direktori _pkg_ dan _bin_.

Subdirektori `src` biasanya memuat version control repositori (seperti Git
atau Mercurial) yang melacak pengembangan dari satu atau lebih source
packages.

Untuk memberi ide bagaimana suatu workspace terlihat dalam praktiknya,
berikut contohnya:

{% highlight text %}
$GOPATH
├── bin
│   ├── gh
│   ├── gotour
│   └── hello
├── pkg
│   └── linux_386
│       └── github.com
│           └── banthar
│               └── Go-SDL
│                   ├── gfx.a
│                   ├── mixer.a
│                   ├── sdl.a
│                   └── ttf.a
└── src
    └── github.com
        └── matematikaadit
            ├── hello
            │   └── hello.go
            └── newmath
                ├── sqrt.go
                └── sqrt_test.go
{% endhighlight %}

Workspace diatas memuat tiga repositori (goauth2, streak, dan todo) yang
menghasilkan dua command (streak dan todo) dan dua pustaka (oauth dan
task).

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
