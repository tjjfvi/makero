#![doc=include_str!("../README.md")]

/// The `makero` macro; see the [crate level documentation](index.html) for
/// more details..
#[macro_export]
#[cfg(doc)]
macro_rules! makero {
  { ... } => { ... };
}

#[macro_export]
#[cfg(not(doc))]
macro_rules! makero {
  ($(#[$attr:meta])* macro_rules! $name:ident $($x:tt)*) => {
    makero!(_ ($) $name $(#[$attr])* macro_rules! $name $($x)*);
  };
  (_ ($d:tt) $name:ident $(#[$attr:meta])* $(macro_rules! $fn:ident {$( ($($arg:tt)*) => {$($result:tt)*} $(;)? )*} )*) => {
    $(#[$attr])*
    macro_rules! $name {
      $($(
        (_makero (fn $fn $d _makero_return:tt) $($arg)*) => {
          $name!{_makero $d _makero_return $($result)*}
        };
        (_makero impl $d prev:tt ($fn ! ($d($d arg:tt)*) $d($d next:tt)*) $d ret:tt) => {
          $name!{_makero impl () ($d($d arg)*) (fn $fn (impl $d prev ($d($d next)*) $d ret))}
        };
      )*)*
      (_makero impl $d prev:tt (($d($d inner:tt)*) $d($d next:tt)*) $d ret:tt) => {
        $name!{_makero impl () ($d($d inner)*) (impl wrap () $d prev ($d($d next)*) $d ret)}
      };
      (_makero impl $d prev:tt ([$d($d inner:tt)*] $d($d next:tt)*) $d ret:tt) => {
        $name!{_makero impl () ($d($d inner)*) (impl wrap [] $d prev ($d($d next)*) $d ret)}
      };
      (_makero impl $d prev:tt ({$d($d inner:tt)*} $d($d next:tt)*) $d ret:tt) => {
        $name!{_makero impl () ($d($d inner)*) (impl wrap {} $d prev ($d($d next)*) $d ret)}
      };
      (_makero impl ($d($d prev:tt)*) ($d cur:tt $d($d next:tt)*) $d ret:tt) => {
        $name!{_makero impl ($d($d prev)* $d cur) ($d($d next)*) $d ret}
      };
      (_makero impl ($d($d prev:tt)*) () $d ret:tt) => {
        $name!{_makero $d ret $d($d prev)*}
      };
      (_makero (impl wrap () ($d($d prev:tt)*) ($d($d next:tt)*) $d ret:tt) $d($d cur:tt)*) => {
        $name!{_makero impl ($d($d prev)* ($d($d cur)*)) ($d($d next)*) $d ret}
      };
      (_makero (impl wrap [] ($d($d prev:tt)*) ($d($d next:tt)*) $d ret:tt) $d($d cur:tt)*) => {
        $name!{_makero impl ($d($d prev)* [$d($d cur)*]) ($d($d next)*) $d ret}
      };
      (_makero (impl wrap {} ($d($d prev:tt)*) ($d($d next:tt)*) $d ret:tt) $d($d cur:tt)*) => {
        $name!{_makero impl ($d($d prev)* {$d($d cur)*}) ($d($d next)*) $d ret}
      };
      (_makero (impl $d prev:tt ($d($d next:tt)*) $d ret:tt) $d($d cur:tt)*) => {
        $name!{_makero impl $d prev ($d($d cur)* $d($d next)*) $d ret}
      };
      (_makero return $d($d result:tt)*) => {
        $d($d result)*
      };
      ($d($d x:tt)*) => {
        $name!{_makero impl () ($name!($d($d x)*)) return}
      };
    }
  };
}
