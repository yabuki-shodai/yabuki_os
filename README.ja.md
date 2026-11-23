[English](README.md) | [日本語](README.ja.md)

# YabukiOS

YabukiOSは、OSの仕組みを学習するために個人開発しているオペレーティングシステムです。

自作プログラミング言語 `.yabu` の実行を目標としています。

## YabukiOSの実行

エミュレーター上でYabukiOSをビルド・実行します。

```bash
make
make run
```

## 実機でYabukiOSを起動する

USBメモリからYabukiOSを起動するためのイメージを作成します。

```bash
make
make usb-image
```

[balenaEtcher](https://etcher.balena.io/) を使用して、以下のイメージファイルをUSBメモリに書き込みます。

```text
build/yabuki-os-0.0.1-usb.img
```

### 起動するOSの選択

1. USBメモリをPCに接続します。
2. PCを再起動します。
3. 起動時にブートメニューを開きます。
4. USBメモリを選択すると、YabukiOSが起動します。

ブートメニューを開くキーはPCのメーカーや機種によって異なります。一般的には `F7`、`F11`、`F12`、`Esc` などが使用されます。

通常使用しているOSを起動する場合は、USBメモリを取り外すか、ブートメニューから通常のOSを選択してください。

**注意事項**

- YabukiOSは現在、x86_64アーキテクチャおよびUEFIに対応したPCを対象としています。
- Secure Bootが有効な環境では、無効化が必要になる場合があります。
- USBメモリへのイメージ書き込み時には、USBメモリ内の既存データが消去されるため注意してください。