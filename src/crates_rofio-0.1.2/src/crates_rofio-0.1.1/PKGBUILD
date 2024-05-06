pkgname=crates_rofio
pkgver=0.1.1
pkgrel=1 
pkgdesc="Search for Rust crates on crates.io and open them in the default web browser"
arch=('x86_64')
license=('GPL')
source=("https://github.com/voidfemme/crates_rofio/archive/refs/tags/$pkgver.tar.gz")
sha256sums=('0430c93595ef307564b654fecf29919fbc38d2d8193a087d97bb2c35b6ca40db')

build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release
}

package() {
  cd "$srcdir/$pkgname-$pkgver"
  install -Dm755 target/release/crates_rofio "$pkgdir/usr/bin/crates_rofio"
}
