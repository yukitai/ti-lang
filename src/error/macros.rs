#[macro_export]
macro_rules! build_ti_error {
  (@at $token: expr, @note $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}: (line {}:{}:{})\n\t", "note".white().bold(), $token.t_at.0, $token.t_at.1.0, $token.t_at.1.1);
      println!($($t) *);
      panic!()
    }
  };
  (@at $token: expr, @info $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}: (line {}:{}:{})\n\t", "info".white().bold(), $token.t_at.0, $token.t_at.1.0, $token.t_at.1.1);
    println!($($t) *);
    panic!()}
  };
  (@at $token: expr, @err $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}: (line {}:{}:{})\n\t", "error".red().bold(), $token.t_at.0, $token.t_at.1.0, $token.t_at.1.1);
      println!($($t) *);
      panic!()
    }
  };
  (@at $token: expr, @warn $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}: (line {}:{}:{})\n\t", "warn".yellow().bold(), $token.t_at.0, $token.t_at.1.0, $token.t_at.1.1);
    println!($($t) *);
    panic!()}
  };
  (@at $token: expr, @help $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}: (line {}:{}:{})\n\t", "help".yellow().bold(), $token.t_at.0, $token.t_at.1.0, $token.t_at.1.1);
    println!($($t) *);
    panic!()}
  };
  (@note $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}:\n\t", "note".yellow().bold());
    println!($($t) *);
    panic!()}
  };
  (@info $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}:\n\t", "info".yellow().bold());
    println!($($t) *);
    panic!()}
  };
  (@err $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}:\n\t", "err".yellow().bold());
    println!($($t) *);
    panic!()}
  };
  (@warn $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}:\n\t", "warn".yellow().bold());
    println!($($t) *);
    panic!()}
  };
  (@help $($t: tt) *) => {
    {
      use colored::Colorize;
      print!("{}:\n\t", "help".yellow().bold());
    println!($($t) *);
    panic!()}
  };
}