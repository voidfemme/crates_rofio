pkgname=crates_rofio
pkgver=0.1.0
pkgrel=1 
pkgdesc="Search for Rust crates on crates.io and open them in the default web browser"
arch=('x86_64')
license=('GPL')
source=('http://github.com/voidfemme/crates_rofio')
sha256sums=('SKIP')

package() {
  cd "$srcdir/$pkgname-$pkgver"
  install -Dm755 crates_rofio "$pkgdir/usr/bin/crates_rofio"
}
