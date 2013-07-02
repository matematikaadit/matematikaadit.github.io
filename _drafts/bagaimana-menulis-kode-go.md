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

Go tool membangun source packages dan menginstal hasil binarinya ke dalam
direktori _pkg_ dan _bin_ masing-masing.

Subdirektori `src` biasanya memuat repositori version control (seperti Git
atau Mercurial) yang melacak pengembangan dari satu atau lebih source
packages.

Untuk memberi ide bagaimana suatu workspace terlihat dalam praktiknya,
berikut contohnya:

    bin/
      streak    # perintah executable
      todo      # perintah executable
    pkg/
      linux_amd64/
        code.google.com/p/goauth2/
          auth.a  # package object
        github.com/nf/todo
          task.a  # package object
    src/
      code.google.com/p/goauth2/
        .hg/
        oauth/
          oauth.go
          oauth_test.go
      github.com/nf/
        strak/
          .git
          oauth.go
          streak.go
        todo/
          .git/
          task/
            task.go
          todo.go

Workspace diatas memuat tiga repositori (goauth2, streak, dan todo) yang
menghasilkan dari dua perintah (streak dan todo) dan dua pustaka (oauth dan
task).

Perintah dan pustaka dibangun dari jenis-jenis source packages yang berbeda.
Kita akan mendiskusikannya kemudian.

### GOPATH environment variabel

Untuk menentukan dimana letak workspace kita, kita perlu mendefinisikan
environment variabel GOPATH. Dan sepertinya hanya environment variabel ini
saja yang perlu kita tentukan untuk mengembangkan kode Go.

Untuk memulai, buat direktori workspace anda dan set GOPATH sesuai
dengannya. Sebagai contoh, $HOME/go akan dijadikan sebagai workspace di
sini.

    $ mkdir $HOME/go
    $ export GOPATH=$HOME/go

untuk lebih mudahnya, tambahkan direktori bin-nya workspace ke dalam PATH:

    $ export PATH=$PATH:$GOPATH/bin
