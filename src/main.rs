use std::cell::Cell;

fn main() {
  fib_num(102);
}

fn fib_num(n: u16) -> u128 {
  let size: usize = usize::try_from(n).ok().unwrap_or(0);
  let f: Vec<Cell<u128>> = vec![Cell::new(0); size];

  let fib_cell: Cell<Option<&dyn Fn(u16) -> u128>> = Cell::new(None);

  let fib_impl = |n: u16| -> u128 {
    let fib = fib_cell.get().unwrap();

    if n == 0 {
      return 0;
    }

    if n == 1 || n == 2 {
      return 1;
    }

    let default = &Cell::new(0);
    let e = f
      .get(usize::try_from(n).ok().unwrap_or(0))
      .unwrap_or(default);

    if e.get() != 0 {
      return e.get();
    }

    let k = || -> u16 {
      if n & 1 == 1 {
        return (n + 1) / 2;
      } else {
        return n / 2;
      }
    };

    let fibk = fib(k());
    let fibk1 = fib(k() - 1);

    if n & 1 == 1 {
      e.set(fibk * fibk + fibk1 * fibk1);
    } else {
      e.set((2 * fibk1 + fibk) * fibk);
    }

    return e.get();
  };

  fib_cell.set(Some(&fib_impl));

  let fib = &fib_impl;

  let fibn = fib(n);

  println!(
    "The {}{} fibonacci number is: {}",
    n,
    get_ordinal_suffix(n),
    fibn
  );

  return fibn;
}

fn get_ordinal_suffix(n: u16) -> String {
  let n_ = n % 10;
  let th = n / 10 % 10 == 1 || n_ < 1 || n_ > 3;

  return match th {
    true => "th".to_string(),
    false => match n_ {
      1 => "st".to_string(),
      2 => "nd".to_string(),
      3 => "rd".to_string(),
      _ => "th".to_string(),
    },
  };
}
