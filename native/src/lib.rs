#[macro_use] extern crate rustler;
#[macro_use] extern crate lazy_static;

extern crate markov;

use std::path::Path;
use std::sync::RwLock;
use markov::Chain;
use rustler::{Env, Term, NifResult, Error, Encoder};
// use rustler::types::OwnedBinary;
use rustler::resource::{ResourceArc};

mod atoms {
    rustler_atoms! {
        atom ok;
        atom nil;
        // atom error;

        // atom empty_chain;

        // Posix
        // atom enoent; // File does not exist
        // atom eacces; // Permission denied
        // atom epipe;  // Broken pipe
        // atom eexist; // File exists
    }
}

// new/0
// of_order/1
// is_empty/1
// feed/2
// feed_str/2
// feed_file/2
// generate/1
// generate_from_token/2
// save/2
// load/1

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
    let chain = markov.chain.write().unwrap();

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
    chain.feed_file(path).unwrap();

    Ok(atoms::ok().encode(env))
}

fn generate<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let markov: ResourceArc<Markov> = args[0].decode()?;
    let chain = markov.chain.write().unwrap();

    if chain.is_empty() {
        return Ok(atoms::nil().encode(env))
    }

    Ok(chain.generate().encode(env))
}

fn generate_str<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let markov: ResourceArc<Markov> = args[0].decode()?;
    let chain = markov.chain.write().unwrap();

    if chain.is_empty() {
        return Ok(atoms::nil().encode(env))
    }

    Ok(chain.generate_str().encode(env))
}

fn generate_from_token<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let markov: ResourceArc<Markov> = args[0].decode()?;
    let token: String = args[1].decode()?;
    let chain = markov.chain.write().unwrap();

    Ok(chain.generate_from_token(token).encode(env))
}

fn save<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let markov: ResourceArc<Markov> = args[0].decode()?;
    let path: String = args[1].decode()?;
    let path = Path::new(&path);

    let chain = markov.chain.write().unwrap();
    // TODO
    chain.save(path).unwrap();

    Ok(atoms::ok().encode(env))
}

fn load<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let path: String = args[0].decode()?;
    let path = Path::new(&path);
    // TODO
    let chain: Chain<String> = Chain::load(&path).unwrap();
    let markov = Markov { chain: RwLock::new(chain) };

    Ok(ResourceArc::new(markov).encode(env))
}
