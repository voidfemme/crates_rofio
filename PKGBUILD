pkgname=crates_rofio
pkgver=0.1.2
pkgrel=1 
pkgdesc="Search for Rust crates on crates.io and open them in the default web browser"
arch=('x86_64')
license=('GPL')
source=("https://github.com/voidfemme/crates_rofio/archive/refs/tags/$pkgver.tar.gz")
sha256sums=('15de02dbde7cebf4bfd096abe6b8d268b9ec1530ca4214f0d3dd4fb525b87738')

build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release
}

package() {
  cd "$srcdir/$pkgname-$pkgver"
  install -Dm755 target/release/crates_rofio "$pkgdir/usr/bin/crates_rofio"
}
