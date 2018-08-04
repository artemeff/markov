#[macro_use] extern crate rustler;
#[macro_use] extern crate lazy_static;

extern crate markov;

use std::io;
use std::path::Path;
use std::sync::RwLock;
use markov::Chain;
use rustler::{Env, Term, Error, Encoder};
use rustler::resource::ResourceArc;

mod atoms {
    rustler_atoms! {
        atom ok;
        atom nil;
        atom error;

        atom enoent; // File does not exist
        atom eacces; // Permission denied
        atom epipe;  // Broken pipe
        atom eexist; // File exists
    }
}

rustler_export_nifs!(
    "Elixir.Markov",
    [("new", 0, new),
     ("of_order", 1, of_order),
     ("empty?", 1, is_empty),
     ("feed", 2, feed),
     ("feed_str", 2, feed_str),
     ("feed_file", 2, feed_file),
     ("generate", 1, generate),
     ("generate_str", 1, generate_str),
     ("generate_from_token", 2, generate_from_token),
     ("save", 2, save),
     ("load", 1, load)],
    Some(on_load)
);

fn io_error_to_term<'a>(env: Env<'a>, err: &io::Error) -> Term<'a> {
    let error = match err.kind() {
        io::ErrorKind::NotFound => atoms::enoent().encode(env),
        io::ErrorKind::PermissionDenied => atoms::eacces().encode(env),
        io::ErrorKind::BrokenPipe => atoms::epipe().encode(env),
        io::ErrorKind::AlreadyExists => atoms::eexist().encode(env),
        _ => format!("{}", err).encode(env),
    };

    (atoms::error(), error).encode(env)
}

macro_rules! handle_io_error {
    ($env:expr, $e:expr) => {
        match $e {
            Ok(inner) => inner,
            Err(ref error) => return Ok(io_error_to_term($env, error)),
        }
    };
}

struct Markov {
    chain: RwLock<Chain<String>>
}

fn on_load(env: Env, _info: Term) -> bool {
    resource_struct_init!(Markov, env);
    true
}

fn new<'a>(env: Env<'a>, _args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let chain = Chain::new();
    let markov = Markov { chain: RwLock::new(chain) };

    Ok(ResourceArc::new(markov).encode(env))
}

fn of_order<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let order: usize = args[0].decode()?;
    let chain = Chain::of_order(order);
    let markov = Markov { chain: RwLock::new(chain) };

    Ok(ResourceArc::new(markov).encode(env))
}

fn is_empty<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let markov: ResourceArc<Markov> = args[0].decode()?;
    let chain = markov.chain.read().unwrap();

    Ok(chain.is_empty().encode(env))
}

fn feed<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let markov: ResourceArc<Markov> = args[0].decode()?;
    let tokens: Vec<String> = args[1].decode()?;
    let mut chain = markov.chain.write().unwrap();

    chain.feed(tokens);

    Ok(atoms::ok().encode(env))
}

fn feed_str<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let markov: ResourceArc<Markov> = args[0].decode()?;
    let token: &str = args[1].decode()?;
    let mut chain = markov.chain.write().unwrap();

    chain.feed_str(token);

    Ok(atoms::ok().encode(env))
}

fn feed_file<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let markov: ResourceArc<Markov> = args[0].decode()?;
    let path: String = args[1].decode()?;
    let path = Path::new(&path);

    let mut chain = markov.chain.write().unwrap();
    handle_io_error!(env, chain.feed_file(path));
    Ok(atoms::ok().encode(env))
}

fn generate<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let markov: ResourceArc<Markov> = args[0].decode()?;
    let chain = markov.chain.read().unwrap();

    if chain.is_empty() {
        return Ok(atoms::nil().encode(env))
    }

    Ok(chain.generate().encode(env))
}

fn generate_str<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let markov: ResourceArc<Markov> = args[0].decode()?;
    let chain = markov.chain.read().unwrap();

    if chain.is_empty() {
        return Ok(atoms::nil().encode(env))
    }

    Ok(chain.generate_str().encode(env))
}

fn generate_from_token<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let markov: ResourceArc<Markov> = args[0].decode()?;
    let token: String = args[1].decode()?;
    let chain = markov.chain.read().unwrap();

    Ok(chain.generate_from_token(token).encode(env))
}

fn save<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let markov: ResourceArc<Markov> = args[0].decode()?;
    let path: String = args[1].decode()?;
    let path = Path::new(&path);

    let chain = markov.chain.read().unwrap();
    handle_io_error!(env, chain.save(path));

    Ok(atoms::ok().encode(env))
}

fn load<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let path: String = args[0].decode()?;
    let path = Path::new(&path);
    let chain: Chain<String> = handle_io_error!(env, Chain::load(&path));
    let markov = Markov { chain: RwLock::new(chain) };

    Ok(ResourceArc::new(markov).encode(env))
}
