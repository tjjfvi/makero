#![feature(trace_macros)]

#[rustfmt::skip]
macro_rules! _makero {
  (($d:tt) macro_rules! $name:ident $($x:tt)*) => {
    _makero!(_ ($d) $name macro_rules! $name $($x)*);
  };
  (_ ($d:tt) $name:ident $(macro_rules! $fn:ident {$( ($($arg:tt)*) => {$($result:tt)*} $(;)? )*} )*) => {
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
  ($d:tt) => {
    macro_rules! makero {
      ($d($d x:tt)*) => {
        _makero!(($d) $d ($d x)*);
      }
    }
  };
}

_makero!($);

makero! {
  macro_rules! main {
    () => {
      is_x!(make_x!())
    }
  }

  macro_rules! is_x {
    (x) => {
      true
    };
    ($($x:tt)*) => {
      false
    };
  }

  macro_rules! make_x {
    () => {
      x
    };
  }
}

fn main() {
  trace_macros!(true);
  let out = main!();
  trace_macros!(false);
  println!("{}", out);
}
