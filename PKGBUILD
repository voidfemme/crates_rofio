pkgname=crates_rofio
pkgver=0.1.2
pkgrel=1 
pkgdesc="Search for Rust crates on crates.io and open them in the default web browser"
arch=('x86_64')
license=('GPL')
source=("https://github.com/voidfemme/crates_rofio/archive/refs/tags/$pkgver.tar.gz")
sha256sums=('c32ed5c55bc9403466dbed5d1b7c0721a11765f0352eeb5a4b7722629a35be12')

build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release
}

package() {
  cd "$srcdir/$pkgname-$pkgver"
  install -Dm755 target/release/crates_rofio "$pkgdir/usr/bin/crates_rofio"
}
