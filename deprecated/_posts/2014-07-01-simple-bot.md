---
layout: post
title: "Bot IRC - Kode Pertama"
tags: "haskell"
---

Postingan sebelumnya:

- [Haskell di OpenShift][haskell-openshift]
- [Sandbox dan Instalasi Paket][sandbox-install]

-----

## Start Simple

Setelah kita instal paket network di postingan sebelumnya, kini kita bisa
pakai modul tersebut untuk **Main.hs** kita (yang baru). Hapus isinya dan
ganti dengan kode berikut ini:

    import Network
    import System.IO

    server :: HostName
    server = "chat.freenode.net"

    port :: PortID
    port = PortNumber $ fromIntegral 6667

    main = do
        h <- connectTo server port
        hSetBuffering h NoBuffering
        t <- hGetContents h
        putStrLn t

Jalankan program diatas dengan mengetikkan:

    $ cabal run
    Package has never been configured. Configuring with default flags. If
    this fails, please run configure manually.
    Resolving dependencies...
    Configuring ircbot-0.1...
    Warning: Packages using 'cabal-version: >= 1.10' must specify the
    'default-language' field for each component (e.g. Haskell98 or
    Haskell2010).
    If a component uses different languages in different modules then list
    the other ones in the 'other-languages' field.
    Preprocessing executable 'server' for ircbot-0.1...
    [1 of 1] Compiling Main             ( Main.hs, dist/build/server/server-tmp/Main.o )
    Linking dist/build/server/server ...
    :kornbluth.freenode.net NOTICE * :*** Looking up your hostname...
    :kornbluth.freenode.net NOTICE * :*** Checking Ident
    :kornbluth.freenode.net NOTICE * :*** Couldn't look up your hostname
    :kornbluth.freenode.net NOTICE * :*** No Ident response
    ERROR :Closing Link: 127.0.0.1 (Connection timed out)

Meskipun sedikit berisik, kode sederhana kita berhasil jalan (pastikan
internet kalian nyala). Lakukan git commit untuk menyimpan checkpoint
pertama kita. Agar tidak diberi peringatan oleh si **Cabal**, kita tambahkan
default-language ke file **server.cabal**.

    name:          ircbot
    version:       0.1
    cabal-version: >= 1.18
    build-type:    Simple
    
    executable server
      main-is:       Main.hs
      build-depends: base, network, random
      default-language: Haskell2010

Oke, langkah pertama berhasil. Selanjutnya kita buat si Bot join channel
**#haskell-id**. Jaga-jaga biar gak terjadi konflik nickname, kita buat nick
si bot sedikit acak. Itu sebabnya kita tambahkan paket random di
build-depends diatas.

## Join Channel

Ubah **Main.hs** sehingga menjadi seperti berikut ini:

    import Network
    import System.IO
    import System.Random
    import Text.Printf

    server :: HostName
    server = "chat.freenode.net"

    port :: PortID
    port = PortNumber $ fromIntegral 6667

    channel :: String
    channel = "#haskell-id"

    getRandomNick :: IO String
    getRandomNick = do
        n <- randomRIO (1, 10000)
        return $ "tutbot" ++ show (n :: Int)

    main = do
        h <- connectTo server port
        hSetBuffering h NoBuffering
        nick <- getRandomNick
        write h "NICK" nick
        write h "USER" $ nick ++ " 0 * :tutorial bot"
        write h "JOIN" channel
        listen h

    write :: Handle -> String -> String -> IO ()
    write h s t = do
        hPrintf h "%s %s\r\n" s t
        printf "> %s %s\n" s t

    listen :: Handle -> IO ()
    listen h = forever $ do
                 s <- hGetLine h
                 putStrLn s
               where
                 forever a = do a; forever a

Namun sayang, bot gak akan bertahan lama, soalnya dia gak diprogram ngereply
ping yang dikirim server. Ubah definisi listen menjadi seperti berikut agar
bot bisa balas ping-nya server.

    import Data.List (isPrefixOf)
    -- sebagian kode tidak ditampilkan

    listen :: Handle -> IO ()
    listen h = forever $ do
                 t <- hGetLine h
                 let s = init t
                 putStrLn s
                 when (ping s) $ pong s
               where
                 forever a = a >> forever a
                 when b a = if b then a else return ()
                 ping x = "PING :" `isPrefixOf` x
                 pong x = write h "PONG" $ dropPing x
                 dropPing = drop 5

Dengan ini lengkap sudah bot sederhana kita. Join **#haskell-id** lalu push
ke OpenShift untuk ngelihat hasilnya.

## Catatan

Akan ada folder **dist** yang isinya build artifact hasil dari cabal run
diatas. Biar gak masuk ke repo, daftar folder tersebut di **.gitignore**

    .cabal-sandbox
    cabal.sandbox.config
    dist
 

[network-docs]: http://hackage.haskell.org/package/network-2.5.0.0/docs/Network.html
[haskell-openshift]: {% post_url 2014-06-04-openshift-haskell %}
[sandbox-install]: {% post_url 2014-06-13-ircbot %}
